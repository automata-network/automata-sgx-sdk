use std::path::PathBuf;
use serde::{Serialize, Deserialize};

pub struct Env {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoSgxOutput {
    pub version: String,
    pub metadata: Vec<CargoSgxOutputMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoSgxOutputMetadata {
    pub edl: PathBuf,
    pub lds: PathBuf,
    pub key: PathBuf,
    pub config: PathBuf,
    pub enclave_archive: PathBuf,
    pub output_signed_so: PathBuf,
}

impl Env {
    // run in cargo-sgx
    pub fn cargo_sgx_output() -> Option<CargoSgxOutput> {
        let data = get_env("CARGO_SGX_OUTPUT", "");
        if data.is_empty() {
            return None;
        }
        serde_json::from_str(&data).unwrap()
    }

    pub fn sgx_builder_path() -> PathBuf {
        PathBuf::new().join(env!("AUTOMATA_SGX_BUILDER_DIR"))
    }

    pub fn is_64bits() -> bool {
        match std::env::var("CARGO_CFG_TARGET_ARCH") {
            Ok(n) => n.contains("64"),
            Err(_) => cfg!(target_pointer_width = "64"),
        }
    }

    pub fn sgx_bin_path() -> PathBuf {
        Self::sdk_path().join("bin").join(match Env::is_64bits() {
            true => "x64",
            false => "x86",
        })
    }

    pub fn sdk_path() -> PathBuf {
        PathBuf::new().join(get_env("SGX_SDK", "/opt/intel/sgxsdk"))
    }

    pub fn sdk_root_dir() -> PathBuf {
        PathBuf::new().join(env!("TEACLAVE_SGX_SDK_ROOT_DIR"))
    }

    pub fn out_dir() -> PathBuf {
        PathBuf::new().join(std::env::var("OUT_DIR").unwrap())
    }

    pub fn pkg_name() -> String {
        std::env::var("CARGO_PKG_NAME").unwrap()
    }

    pub fn sgx_lib_path() -> PathBuf {
        let sdk_path = Self::sdk_path();
        match Env::is_64bits() {
            true => sdk_path.join("lib64"),
            false => sdk_path.join("lib"),
        }
    }

    pub fn sgx_mode() -> String {
        get_env("SGX_MODE", "HW")
    }

    pub fn rust_target_path() -> PathBuf {
        Self::sdk_root_dir().join("rustlib")
    }

    pub fn sgx_target_name() -> String {
        "x86_64-automata-linux-sgx".into()
    }

    pub fn sgx_target_json() -> PathBuf {
        Self::rust_target_path().join(format!("{}.json", Self::sgx_target_name()))
    }

    pub fn sgx_common_cflags() -> Vec<&'static str> {
        vec![match Env::is_64bits() {
            true => "-m64",
            false => "-m32",
        }]
    }

    pub fn custom_edl_path() -> PathBuf {
        Self::sdk_root_dir().join("sgx_edl").join("edl")
    }

    pub fn custom_common_path() -> PathBuf {
        Self::sdk_root_dir().join("common")
    }
}

#[allow(dead_code)]
pub(crate) fn must_get_env(key: &str) -> String {
    println!("cargo:rerun-if-env-changed={}", key);
    match std::env::var(key) {
        Ok(n) => n,
        Err(_) => panic!("missing env: {}", key),
    }
}

pub(crate) fn get_env(key: &str, def: &str) -> String {
    println!("cargo:rerun-if-env-changed={}", key);
    match std::env::var(key) {
        Ok(n) => n,
        Err(_) => def.into(),
    }
}
