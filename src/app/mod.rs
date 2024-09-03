#[cfg(feature = "tstd_app")]
mod enclave;
#[cfg(feature = "tstd_app")]
pub use enclave::*;

#[cfg(not(feature = "tstd_app"))]
mod untrusted;
#[cfg(not(feature = "tstd_app"))]
#[allow(unused_imports)]
pub use untrusted::*;