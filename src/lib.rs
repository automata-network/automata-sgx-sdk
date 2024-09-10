#![cfg_attr(target_vendor = "teaclave", feature(rustc_private))]
#![feature(panic_unwind)]

pub mod sgxlib;

pub mod cutils;

pub mod types;

mod patch;

#[cfg(feature = "app")]
pub mod app;

pub mod dcap;

#[cfg(not(target_vendor = "teaclave"))]
mod builders;
#[cfg(not(target_vendor = "teaclave"))]
pub use builders::*;

mod env;
pub use env::*;

pub use ctor::ctor;

#[cfg(all(not(target_vendor = "teaclave"), feature = "tstd_app"))]
pub fn build_app() {
    build_enclave_objs();
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

#[cfg(not(target_vendor = "teaclave"))]
pub fn enalbe_compatibility_mode() {
    #[cfg(not(feature = "tstd_app"))]
    std::env::set_var("CARGO_BUILD", "1")
}

#[cfg(all(not(target_vendor = "teaclave"), not(feature = "tstd_app")))]
pub fn build_app() {
    use std::{os::unix::fs::symlink, process::Command};

    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let out_dir = Env::out_dir();
    let search_path = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    match Env::cargo_sgx_output() {
        Some(cargo_sgx_output) => {
            println!("cargo:rustc-link-search=native={}", search_path.display());
            for enclave in &cargo_sgx_output.metadata {
                let enclave_name = enclave
                    .enclave_archive
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .trim_start_matches("lib");

                println!(
                    "cargo:rerun-if-changed={}",
                    search_path.join(format!("lib{}.a", enclave_name)).display()
                );
                println!("cargo:rustc-link-lib={}", enclave_name);
            }
        }
        None => {
            println!(
                "cargo:warning={} is intended to build from `cargo sgx build`, please try install it by `cargo install cargo-sgx`, now will goto compatibility mode (rebuild everytime)",
                pkg_name
            );
            // println!("cargo:rerun-if-env-changed=CARGO_BUILD");
            // if std::env::var("CARGO_BUILD") != Ok("1".to_owned()) {
            //     let note = ["set CARGO_BUILD=1 to enable compatibility mode.", "or you can add automata_sgx_builder::enalbe_compatibility_mode(); to the build script."];
            //     println!("cargo:warning=NOTICE:\n\nset CARGO_BUILD=1 to enable compatibility mode. \n\n\n", "=".repeat(80));
            // }
            println!("cargo:rerun-if-changed=compatibility mode");

            let profile = std::env::var("PROFILE").unwrap();
            let origin_target_dir = search_path.parent().unwrap();
            let new_target_dir = origin_target_dir.join("tmp-target");
            let _ = std::fs::create_dir_all(&new_target_dir.join(&profile));

            let _ = symlink(
                &origin_target_dir.join(&profile).join("build"),
                new_target_dir.join(&profile).join("build"),
            );
            let _ = symlink(
                &origin_target_dir.join(&profile).join("deps"),
                new_target_dir.join(&profile).join("deps"),
            );

            println!(
                "cargo:rustc-link-search=native={}",
                new_target_dir.join(&profile).display()
            );
            for (lib_name, pkg_name) in get_metadata_pkgs() {
                let mut cmd = Command::new(std::env::var("CARGO").unwrap());
                cmd.arg("build");
                if profile == "release" {
                    cmd.arg("--release");
                }
                cmd.arg("--target-dir").arg(&new_target_dir);
                cmd.arg("-p").arg(&pkg_name).arg("--color").arg("never");
                assert!(cmd.status().unwrap().success());
                println!("cargo:rustc-link-lib={}", lib_name);
            }
            return;
        }
    };
}

#[cfg(not(target_vendor = "teaclave"))]
pub fn get_metadata_pkgs() -> Vec<(String, String)> {
    use std::path::PathBuf;
    let cwd = std::env::current_dir().unwrap();
    let data = std::fs::read_to_string(PathBuf::new().join(cwd).join("Cargo.toml")).unwrap();

    let cargo_metadata: toml::Value = toml::from_str(&data).unwrap();
    match cargo_metadata.get("package") {
        Some(pkg) => match pkg.get("metadata") {
            Some(md) => match md.get("sgx") {
                Some(sgx) => {
                    if let Some(table) = sgx.as_table() {
                        let mut out = Vec::new();
                        for (lib_name, t) in table {
                            let path =
                                PathBuf::new().join(t.get("path").unwrap().as_str().unwrap());
                            let pkg_name = path.file_stem().unwrap().to_str().unwrap().to_owned();
                            out.push((lib_name.clone(), pkg_name));
                        }
                        return out;
                    }
                }
                None => {}
            },
            None => {}
        },
        None => {}
    }
    Vec::new()
}

#[cfg(not(target_vendor = "teaclave"))]
pub fn build_enclave_objs() {
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let cargo_sgx_output = match Env::cargo_sgx_output() {
        Some(n) => n,
        None => {
            println!("cargo:warning={} is intended to build from `cargo sgx build`, please try install it by `cargo install cargo-sgx`", pkg_name);
            return;
        }
    };
    let mode = BuildMode::BuildScript;
    let out_dir = Env::out_dir();
    let proxy_trusted_dir = out_dir.join("proxy_trusted");
    let proxy_untrusted_dir = out_dir.join("proxy_untrusted");
    for enclave in &cargo_sgx_output.metadata {
        mode.trace_file(&enclave.enclave_archive);
        let edl_name = enclave.edl.file_stem().unwrap().to_str().unwrap();
        let enclave_name = enclave
            .enclave_archive
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        println!(
            "cargo:rustc-env=ENCLAVE_{}=1",
            snake_to_camel(enclave_name.trim_start_matches("lib"))
        );
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
}

#[cfg(not(target_vendor = "teaclave"))]
fn snake_to_camel(snake: &str) -> String {
    let mut camel = String::new();
    let mut upper_next = true;

    for c in snake.chars() {
        if c == '_' {
            upper_next = true;
        } else {
            if upper_next {
                camel.push(c.to_ascii_uppercase());
                upper_next = false;
            } else {
                camel.push(c);
            }
        }
    }

    camel
}
