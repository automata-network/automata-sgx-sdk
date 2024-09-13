use std::path::PathBuf;

use crate::sgxlib::{sgx_types, sgx_urts};

pub use sgx_types::error::SgxResult;

pub struct SgxEnclave {
    enclave: sgx_urts::enclave::SgxEnclave,
}

impl SgxEnclave {
    pub fn new(name: &str, debug: bool) -> SgxResult<Self> {
        let args = std::env::args().collect::<Vec<_>>();
        let enclave_path = PathBuf::new().join(&args[0]).parent().unwrap().join(name);
        let enclave = sgx_urts::enclave::SgxEnclave::create(enclave_path, debug)?;
        Ok(Self { enclave })
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

    pub fn eid(&self) -> u64 {
        self.enclave.eid()
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

        const _: () = {
            #[$crate::ctor]
            fn fix_link() {
                unsafe { $crate::sgxlib::sgx_types::function::sgx_ecall(0, 0, std::ptr::null(), std::ptr::null()) };
            }
        };

        impl $enclave_name {
            pub fn new(debug: bool) -> $crate::types::SgxResult<Self> {
                let name = $crate::app::SgxEnclave::camel_to_snake(stringify!($enclave_name));
                env!(concat!("ENCLAVE_", stringify!($enclave_name)), concat!("the enclave ", stringify!($enclave_name) ," is not defined in Cargo.toml"));

                let name = format!("lib{}.signed.so", name);
                let enclave = $crate::app::SgxEnclave::new(&name, debug)?;
                Ok(Self(enclave))
            }

            $(
                pub fn $fn_name(&self, $($arg_name: $arg_type),*) -> $crate::types::SgxResult<$($ret_type)?> {
                    let mut retval = $crate::types::SgxStatus::Success;
                    let ret = unsafe {
                        $fn_name(self.0.eid(), $($arg_name,)* &mut retval)
                    };
                    if retval != $crate::types::SgxStatus::Success {
                        return Err(retval);
                    }
                    Ok(ret)
                }
            )*
        }
    };
}
