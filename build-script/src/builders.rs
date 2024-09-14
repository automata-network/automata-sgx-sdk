use std::{
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use cc::Build;

use crate::{cutils::Cutils, Env};

#[derive(Debug, Clone, Copy)]
pub enum BuildMode {
    BuildScript,
    Shell,
}

impl BuildMode {
    pub fn apply_build(&self, b: &mut Build) {
        match self {
            BuildMode::BuildScript => {}
            BuildMode::Shell => {
                b.target("test")
                    .host("test")
                    .opt_level(2)
                    .shared_flag(false)
                    .cargo_metadata(false);
            }
        }
    }

    pub fn trace_file<P: AsRef<Path>>(&self, p: P) {
        if !matches!(self, BuildMode::BuildScript) {
            return;
        }
        println!("cargo:rerun-if-changed={}", p.as_ref().display());
    }
}

pub struct UntrustedProxyBuilder {
    mode: BuildMode,
}

impl UntrustedProxyBuilder {
    pub fn new(mode: BuildMode) -> Self {
        Self { mode }
    }

    pub fn build(&self, source: &PathBuf, output: &PathBuf) {
        let includes = vec![
            Env::sdk_path().join("include"),
            Env::sdk_root_dir().join("common").join("inc"),
            Env::sdk_root_dir().join("sgx_edl").join("edl"),
        ];

        let mut build = cc::Build::new();
        self.mode.apply_build(&mut build);
        apply_flags(&mut build, &Cutils::cflags());
        apply_flags(&mut build, &["-fPIC", "-Wno-attributes"]);
        if Env::is_64bits() {
            build.flag_if_supported("-m64");
        } else {
            build.flag_if_supported("-m32");
        }
        build.includes(includes);
        build.out_dir(parent_path(output));
        build.file(source).compile(&link_name(output));
        // self.mode.trace_file(source);
    }
}

pub struct TrustedProxyBuilder {
    mode: BuildMode,
}

impl TrustedProxyBuilder {
    pub fn new(mode: BuildMode) -> Self {
        Self { mode }
    }

    pub fn build(&self, source: &PathBuf, output: &PathBuf) {
        let includes = vec![
            Env::sdk_root_dir().join("common").join("inc"),
            Env::sdk_root_dir().join("common").join("inc").join("tlibc"),
            Env::sdk_root_dir().join("sgx_edl").join("edl"),
        ];
        let mut build = cc::Build::new();
        self.mode.apply_build(&mut build);
        apply_flags(&mut build, &Cutils::cflags());
        apply_flags(&mut build, &Cutils::enclave_cflags());
        if Env::is_64bits() {
            build.flag_if_supported("-m64");
        } else {
            build.flag_if_supported("-m32");
        }
        build.includes(includes);

        let mut cmd = build.get_compiler().to_command();
        cmd.args(["-c", &format!("{}", source.display())]);
        cmd.args(["-o", &format!("{}", output.display())]);
        run_cmd(cmd);
    }
}

pub struct EnclaveSharedObjectBuilder {
    mode: BuildMode,
}

impl EnclaveSharedObjectBuilder {
    pub fn new(mode: BuildMode) -> Self {
        Self { mode }
    }

    pub fn build(
        &self,
        trusted_proxy: &PathBuf,
        enclave_object: &PathBuf,
        lds: &PathBuf,
        output: &PathBuf,
    ) {
        // self.mode.trace_file(trusted_proxy);
        // self.mode.trace_file(enclave_object);
        self.mode.trace_file(lds);

        let mut build = cc::Build::new();
        self.mode.apply_build(&mut build);
        let mut cmd = build.get_compiler().to_command();
        cmd.args([
            &format!("{}", trusted_proxy.display()),
            "-o",
            &format!("{}", output.display()),
        ]);
        cmd.args(Cutils::enclave_ldflags());
        cmd.args([
            "-Wl,--no-undefined",
            "-nostdlib",
            "-nodefaultlibs",
            "-nostartfiles",
        ]);
        cmd.arg(format!("-Wl,--version-script={}", lds.display()));
        cmd.args([
            "-Wl,--start-group",
            &format!("-L{}", parent_path(enclave_object)),
            &format!("-l{}", link_name(enclave_object)),
            // &format!("-L/usr/lib/x86_64-linux-gnu"),
            //  "-lssl", "-lcrypto",
            "-Wl,--end-group",
        ]);

        run_cmd(cmd)
    }
}

fn link_name(p: &PathBuf) -> String {
    let n = p.file_name().unwrap().to_str().unwrap();
    n.trim_end_matches(".o")
        .trim_end_matches(".a")
        .trim_start_matches("lib")
        .into()
}

fn parent_path(p: &PathBuf) -> String {
    let p = p.parent().unwrap();
    let p_str = format!("{}", p.display());
    if p_str == "" {
        ".".into()
    } else {
        p_str
    }
}

pub struct Edger8r {
    mode: BuildMode,
    path: PathBuf,
    search_path: Vec<PathBuf>,
}

impl Edger8r {
    pub fn new(mode: BuildMode) -> Self {
        let bin = Env::sgx_bin_path().join("sgx_edger8r");
        let search_path = vec![
            Env::custom_edl_path(),
            Env::sgx_builder_path().join("edl"),
            Env::custom_common_path().join("inc"),
        ];

        Self {
            mode,
            path: bin,
            search_path,
        }
    }

    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_path.push(path);
    }

    pub fn build(&self, edl: &PathBuf, trusted: bool, dir: &PathBuf) -> PathBuf {
        self.mode.trace_file(edl);

        let mut cmd = Command::new(&self.path);
        // let current_dir = std::env::current_dir().unwrap();
        // let edl = current_dir.join(edl);
        if format!("{}", dir.parent().unwrap().display()) != "" {
            cmd.current_dir(dir.parent().unwrap());
        }

        if trusted {
            cmd.args(["--trusted", &format!("{}", edl.display())]);
            cmd.args(["--trusted-dir", &format!("{}", dir.display())]);
        } else {
            cmd.args(["--untrusted", &format!("{}", edl.display())]);
            cmd.args(["--untrusted-dir", &format!("{}", dir.display())]);
        }
        for s in &self.search_path {
            cmd.args(["--search-path", s.to_str().unwrap()]);
        }

        run_cmd(cmd);

        let name = edl.file_stem().unwrap();
        dir.join(format!(
            "{}_{}.c",
            name.to_string_lossy(),
            match trusted {
                true => "t",
                false => "u",
            }
        ))
    }
}

pub struct LdsBuilder {}

impl LdsBuilder {
    pub fn new() -> Self {
        LdsBuilder {}
    }

    pub fn generate(&self, shared_lib: &str, out: &PathBuf) {
        let mut data = vec![0_u8; 0];
        writeln!(data, "{}\n{{", shared_lib).unwrap();
        writeln!(data, "\tglobal:").unwrap();
        let segs = [
            "g_global_data_hyper",
            "g_global_data_sim",
            "g_global_data",
            "enclave_entry",
            "g_peak_heap_used",
            "g_peak_rsrv_mem_committed",
        ];
        for seg in segs {
            writeln!(data, "\t\t{};", seg).unwrap();
        }
        writeln!(data, "\tlocal:\n\t\t*;").unwrap();
        writeln!(data, "}};").unwrap();
        std::fs::write(out, data).unwrap();
    }
}

pub struct SgxSigner {
    mode: BuildMode,
    bin: PathBuf,
}

impl SgxSigner {
    pub fn new(mode: BuildMode) -> Self {
        let bin = Env::sgx_bin_path().join("sgx_sign");
        Self { mode, bin }
    }

    pub fn generate_config(&self, out: &PathBuf) {
        let cfg = r#"<EnclaveConfiguration>
  <ProdID>0</ProdID>
  <ISVSVN>0</ISVSVN>
  <StackMaxSize>0x80000</StackMaxSize>
  <HeapMaxSize>0x40000000</HeapMaxSize>
  <TCSNum>33</TCSNum>
  <TCSPolicy>0</TCSPolicy>
  <DisableDebug>0</DisableDebug>
  <MiscSelect>0</MiscSelect>
  <MiscMask>0xFFFFFFFF</MiscMask>
</EnclaveConfiguration>"#;
        std::fs::write(out, cfg.as_bytes()).unwrap();
    }

    pub fn sign(&self, cfg: &PathBuf, out: &PathBuf, enclave: &PathBuf, pem: &PathBuf) {
        self.mode.trace_file(cfg);
        // self.mode.trace_file(enclave);
        self.mode.trace_file(pem);

        let mut cmd = Command::new(&self.bin);
        cmd.arg("sign");
        cmd.args(["-config", &format!("{}", cfg.display())]);
        cmd.args(["-out", &format!("{}", out.display())]);
        cmd.args(["-enclave", &format!("{}", enclave.display())]);
        cmd.args(["-key", &format!("{}", pem.display())]);
        run_cmd(cmd);
    }
}

pub struct EdlBuilder {
    imports: Vec<String>,
    trusted: Vec<String>,
}

impl EdlBuilder {
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
            trusted: Vec::new(),
        }
    }

    pub fn preset_imports(&mut self) -> &mut Self {
        self.add_import("sgx_env.edl");
        self.add_import("sgx_sync.edl");
        self.add_import("sgx_stdio.edl");
        self.add_import("sgx_net.edl");
        self.add_import("sgx_fs.edl");
        self.add_import("sgx_thread.edl");
        self.add_import("sgx_process.edl");
        self.add_import("sgx_tstd.edl");
        self.add_import("sgx_cpuid.edl");
        self
    }

    pub fn add_import(&mut self, name: &str) -> &mut Self {
        self.imports.push(name.into());
        self
    }

    pub fn add_trusted(&mut self, data: &str) -> &mut Self {
        self.trusted.push(data.into());
        self
    }

    pub fn generate(&self, path: &PathBuf) {
        let mut data = vec![0_u8; 0];
        writeln!(data, "enclave {{").unwrap();
        for import in &self.imports {
            writeln!(data, "\tfrom {:?} import *;", import).unwrap();
        }
        writeln!(data, "\ttrusted {{").unwrap();
        for line in &self.trusted {
            writeln!(data, "\t\t{}", line).unwrap();
        }
        writeln!(data, "\t}};").unwrap();
        writeln!(data, "}};").unwrap();

        std::fs::write(path, data).unwrap();
    }
}

fn apply_flags(b: &mut cc::Build, flags: &[&'static str]) {
    for flag in flags {
        b.flag_if_supported(flag);
    }
}

fn run_cmd(mut cmd: Command) {
    match cmd.status() {
        Ok(status) => {
            if !status.success() {
                panic!("exec {:?} failed: {:?}", cmd, status);
            }
        }
        Err(err) => {
            panic!("exec {:?} failed: {:?}", cmd, err);
        }
    }
}
