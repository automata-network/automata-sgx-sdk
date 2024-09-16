include!("../build_dep.rs");

fn main() {
    #[cfg(not(target_vendor = "teaclave"))]
    build_sysroot();
}

fn build_sysroot() {
    let sgx_target = "x86_64-automata-linux-sgx";

    let out_dir = PathBuf::new().join(std::env::var("OUT_DIR").unwrap());
    if out_dir.as_os_str().to_str().unwrap().contains(sgx_target) {
        return;
    }

    println!("cargo:rerun-if-env-changed=SGX_MODE");

    let sdk_path = get_teaclave_sdk_path().expect("unable to locate teaclave_sdk");
    let rust_target_path = sdk_path.join("rustlib");
    std::fs::write(
        out_dir.join("TEACLAVE_SGX_SDK_ROOT_DIR"),
        sdk_path.to_str().unwrap(),
    )
    .unwrap();
    let target = rust_target_path.join(format!("{}.json", sgx_target));

    let sysroot = PathBuf::new()
        .join(std::env::var("OUT_DIR").unwrap())
        .join("sysroot");
    println!(
        "cargo:warning=building enclave sysroot to {:?}, source={:?}",
        sysroot.display(),
        sdk_path.display(),
    );
    // may cause deadlock when called by cargo-sgx
    let mut cmd = Command::new(std::env::var("CARGO").unwrap());
    cmd.args(["build", "--manifest-path"]);
    cmd.arg(format!("{}/std/Cargo.toml", rust_target_path.display()));
    cmd.args([
        "-Z",
        "build-std=core,alloc",
        "--release",
        "--features",
        "env,net,thread,untrusted_time,untrusted_fs,unsupported_process,capi,backtrace",
        "--target-dir",
        &format!("{}", sysroot.join("target").display()),
    ]);
    cmd.arg("--target");
    cmd.arg(format!("{}", target.display()));
    assert!(cmd.status().unwrap().success());

    let std_target_path = PathBuf::new()
        .join(std::env::var("OUT_DIR").unwrap())
        .join("sysroot")
        .join("target")
        .join(sgx_target)
        .join("release")
        .join("deps");

    let sysroot = PathBuf::new()
        .join(std::env::var("OUT_DIR").unwrap())
        .join("sysroot")
        .join("lib")
        .join("rustlib")
        .join(sgx_target)
        .join("lib");
    let _ = std::fs::remove_file(&sysroot);
    std::fs::create_dir_all(sysroot.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(std_target_path, &sysroot).unwrap();
}
