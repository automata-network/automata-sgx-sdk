use std::{path::PathBuf, process::Command};

fn main() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_path = PathBuf::new().join(&root_dir);

    let sdk_path = root_path.join("incubator-teaclave-sgx-sdk");

    println!(
        "cargo:rustc-env=TEACLAVE_SGX_SDK_ROOT_DIR={}",
        sdk_path.display()
    );

    let rust_target_path = sdk_path.join("rustlib");
    let sgx_target = "x86_64-unknown-linux-sgx";

    let target = rust_target_path.join(format!("{}.json", sgx_target));

    // let build_std = std::env::var("CARGO_FEATURE_BUILD_STD") == Ok("1".to_owned());

    let build_std = get_env("BUILD_STD", "") == "1";
    // println!("cargo:warning=build_std={:?}", build_std);
    if build_std {
        let sysroot = PathBuf::new()
            .join(std::env::var("OUT_DIR").unwrap())
            .join("sysroot");
        println!(
            "cargo:warning=building enclave std to {:?}",
            sysroot.display()
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
            "env,net,thread,untrusted_time,untrusted_fs,unsupported_process,capi",
        ]);
        cmd.arg("--target");
        cmd.arg(format!("{}", target.display()));
        assert!(cmd.status().unwrap().success());

        let std_target_path = root_path
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

        let sysroot = PathBuf::new()
            .join(std::env::var("OUT_DIR").unwrap())
            .join("sysroot");

        println!(
            "cargo:rustc-env=TEACLAVE_SGX_SYS_ROOT_DIR={}",
            sysroot.display()
        );
    }
}

fn get_env(key: &str, def: &str) -> String {
    println!("cargo:rerun-if-env-changed={}", key);
    match std::env::var(key) {
        Ok(n) => n,
        Err(_) => def.into(),
    }
}
