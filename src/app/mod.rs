#[cfg(feature = "tstd_app")]
mod enclave;
use std::path::PathBuf;

#[cfg(feature = "tstd_app")]
pub use enclave::*;

#[cfg(not(feature = "tstd_app"))]
mod untrusted;
#[cfg(not(feature = "tstd_app"))]
#[allow(unused_imports)]
pub use untrusted::*;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

base::stack_error! {
    #[derive(Debug, Clone)]
    name: AppError,
    stack_name: AppErrorStack,
    error: {

    },
    wrap: {
        Sgx(crate::types::SgxStatus),
    },
    stack: {
        CreateEnclave(path: PathBuf),
        OnEcall(name: &'static str),
    }
}