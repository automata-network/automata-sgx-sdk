include!("build_dep.rs");

fn main() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = std::path::PathBuf::new().join(root_dir).join("Cargo.toml");
    let teaclave_sdk_path =
        get_teaclave_sdk_path(manifest_path).expect("unable to locate teaclave_sdk");
    println!(
        "cargo:rustc-env=TEACLAVE_SGX_SDK_ROOT_DIR={}",
        teaclave_sdk_path.display()
    );
}
