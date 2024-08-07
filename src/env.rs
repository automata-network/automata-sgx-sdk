use std::path::PathBuf;

pub struct Env {}

impl Env {
    // run in cargo-sgx
    pub fn sgx_metadata_edl() -> PathBuf {
        PathBuf::new().join(must_get_env("SGX_METADATA_EDL"))
    }

    pub fn sgx_metadata_lds() -> PathBuf {
        PathBuf::new().join(must_get_env("SGX_METADATA_LDS"))
    }

    pub fn sgx_metadata_key() -> PathBuf {
        PathBuf::new().join(must_get_env("SGX_METADATA_KEY"))
    }

    pub fn sgx_metadata_config() -> PathBuf {
        PathBuf::new().join(must_get_env("SGX_METADATA_CONFIG"))
    }

    pub fn sgx_metadata_enclave() -> PathBuf {
        PathBuf::new().join(must_get_env("SGX_METADATA_ENCLAVE"))
    }

    pub fn sgx_metadata_output() -> PathBuf {
        PathBuf::new().join(must_get_env("SGX_METADATA_OUTPUT"))
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
        "x86_64-unknown-linux-sgx".into()
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
