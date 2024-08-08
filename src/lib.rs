pub mod sgxlib;

pub mod cutils;

#[cfg(feature = "builder")]
mod builders;
#[cfg(feature = "builder")]
pub use builders::*;

mod env;
pub use env::*;

#[cfg(feature = "builder")]
pub fn build_app() {
    let mode = BuildMode::BuildScript;
    let out_dir = Env::out_dir();
    let proxy_trusted_dir = out_dir.join("proxy_trusted");
    let proxy_untrusted_dir = out_dir.join("proxy_untrusted");

    let cargo_sgx_output = Env::cargo_sgx_output();
    for enclave in &cargo_sgx_output.metadata {
        let edl_name = enclave.edl.file_stem().unwrap().to_str().unwrap();
        let enclave_name = enclave.enclave_archive.file_stem().unwrap().to_str().unwrap();
        let proxy_trusted_source = Edger8r::new(mode).build(&enclave.edl, true, &proxy_trusted_dir);
        let proxy_untrusted_source =
            Edger8r::new(mode).build(&enclave.edl, false, &proxy_untrusted_dir);

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
            &enclave.enclave_archive,
            &enclave.lds,
            &out_dir.join(format!("{}.so", enclave_name)),
        );
        SgxSigner::new(mode).sign(
            &enclave.config,
            &enclave.output_signed_so,
            &out_dir.join(format!("{}.so", enclave_name)),
            &enclave.key,
        );
    }

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
