use std::path::PathBuf;
use std::sync::Mutex;

use crate::sgxlib::{sgx_types, sgx_urts};

pub use sgx_types::error::SgxStatus;

pub struct SgxEnclave {
    pub debug: bool,
    name: String,
    enclave: Mutex<Option<Result<sgx_urts::enclave::SgxEnclave, super::AppError>>>,
}

impl SgxEnclave {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            debug: std::env::var("SGX_DEBUG").unwrap_or_default() != "".to_string(),
            enclave: Mutex::new(None),
        }
    }

    pub fn camel_to_snake(s: &str) -> String {
        let mut snake_case = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() {
                if i != 0 {
                    snake_case.push('_');
                }
                for lower in c.to_lowercase() {
                    snake_case.push(lower);
                }
            } else {
                snake_case.push(c);
            }
        }
        snake_case
    }

    pub fn eid(&self) -> Result<u64, super::AppError> {
        let mut enclave = self.enclave.lock().unwrap();
        match enclave.as_ref() {
            None => {
                let args = std::env::args().collect::<Vec<_>>();
                let enclave_path = PathBuf::new().join(&args[0]).parent().unwrap().join(&self.name);
                let result = sgx_urts::enclave::SgxEnclave::create(&enclave_path, self.debug)
                    .map_err(super::AppError::CreateEnclave(&enclave_path));
                *enclave = Some(result);
                match enclave.as_ref().unwrap() {
                    Ok(result) => Ok(result.eid()),
                    Err(err) => Err(err.clone()),                    
                }
            }
            Some(enclave) => match enclave {
                Ok(result) => Ok(result.eid()),
                Err(err) => Err(err.clone()),
            },
        }
    }
}

#[macro_export]
macro_rules! enclave {
    (
        name: $enclave_name:ident,
        ecall: {
            $(
            fn $fn_name:ident ( $($arg_name:ident : $arg_type:ty),* ) $(-> $ret_type:ty)?;
            )*
        }
    ) => {
        pub struct $enclave_name($crate::app::SgxEnclave);

        extern "C" {
            $(
            fn $fn_name(eid: $crate::types::EnclaveId, $($arg_name: $arg_type,)* retval: *mut $crate::types::SgxStatus)  $(-> $ret_type)?;
            )*
        }

        impl $enclave_name {
            pub fn new() -> Self {
                let name = $crate::app::SgxEnclave::camel_to_snake(stringify!($enclave_name));
                env!(concat!("ENCLAVE_", stringify!($enclave_name)), concat!("the enclave ", stringify!($enclave_name) ," is not defined in Cargo.toml"));

                let name = format!("lib{}.signed.so", name);
                let enclave = $crate::app::SgxEnclave::new(&name);
                Self(enclave)
            }

            $(
                pub fn $fn_name(&self, $($arg_name: $arg_type),*) -> $crate::app::AppResult<$($ret_type)?> {
                    use $crate::app::AppError;
                    let eid = self.0.eid().map_err(AppError::OnEcall(&stringify!($fn_name)))?;
                    let mut retval = $crate::types::SgxStatus::Success;
                    let ret = unsafe {
                        $fn_name(eid, $($arg_name,)* &mut retval)
                    };
                    if retval != $crate::types::SgxStatus::Success {
                        return Err(retval).map_err(AppError::OnEcall(&stringify!($fn_name)));
                    }
                    Ok(ret)
                }
            )*
        }
    };
}
