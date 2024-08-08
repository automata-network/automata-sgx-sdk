use std::path::PathBuf;

fn main() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root_path = PathBuf::new().join(&root_dir);

    let sdk_path = root_path.join("incubator-teaclave-sgx-sdk");

    println!(
        "cargo:rustc-env=TEACLAVE_SGX_SDK_ROOT_DIR={}",
        sdk_path.display()
    );
}
