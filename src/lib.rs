pub mod sgxlib;

#[cfg(feature = "builder")]
pub mod cutils;

#[cfg(feature = "builder")]
mod builders;
#[cfg(feature = "builder")]
pub use builders::*;

#[cfg(feature = "builder")]
mod env;
#[cfg(feature = "builder")]
pub use env::*;

#[cfg(feature = "builder")]
pub fn build_app() {
    let mode = BuildMode::BuildScript;
    let out_dir = Env::out_dir();
    let edl_path = Env::sgx_metadata_edl();
    let lds_path = Env::sgx_metadata_lds();
    let key_path = Env::sgx_metadata_key();
    let signed_output_path = Env::sgx_metadata_output();
    let config_path = Env::sgx_metadata_config();
    let edl_name = edl_path.file_stem().unwrap().to_str().unwrap();
    let enclave_object = Env::sgx_metadata_enclave();
    let enclave_name = enclave_object.file_stem().unwrap().to_str().unwrap();
    let proxy_trusted_dir = out_dir.join("proxy_trusted");
    let proxy_untrusted_dir = out_dir.join("proxy_untrusted");

    let proxy_trusted_source = Edger8r::new(mode).build(&edl_path, true, &proxy_trusted_dir);
    let proxy_untrusted_source = Edger8r::new(mode).build(&edl_path, false, &proxy_untrusted_dir);

    UntrustedProxyBuilder::new(mode).build(
        &proxy_untrusted_source,
        &proxy_untrusted_dir.join(format!("{}_u.o", edl_name)),
    );
    TrustedProxyBuilder::new(mode).build(
        &proxy_trusted_source,
        &proxy_trusted_dir.join(format!("{}_t.o", edl_name)),
    );
    EnclaveSharedObjectBuilder::new(mode).build(
        &proxy_trusted_dir.join(format!("{}_t.o", edl_name)),
        &enclave_object,
        &lds_path,
        &out_dir.join(format!("{}.so", enclave_name)),
    );
    SgxSigner::new(mode).sign(
        &config_path,
        &signed_output_path,
        &out_dir.join(format!("{}.so", enclave_name)),
        &key_path,
    );

    println!(
        "cargo:rustc-link-search=native={}",
        Env::sgx_lib_path().display()
    );
    match Env::sgx_mode().as_str() {
        "SIM" | "SW" => println!("cargo:rustc-link-lib=dylib=sgx_urts_sim"),
        "HYPER" => println!("cargo:rustc-link-lib=dylib=sgx_urts_hyper"),
        "HW" => println!("cargo:rustc-link-lib=dylib=sgx_urts"),
        _ => println!("cargo:rustc-link-lib=dylib=sgx_urts"),
    }
}
