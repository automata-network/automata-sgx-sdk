#[cfg(all(feature = "dcap", not(target_vendor = "teaclave")))]
mod untrusted;
#[cfg(all(feature = "dcap", not(target_vendor = "teaclave")))]
pub use untrusted::*;

#[cfg(all(feature = "dcap", target_vendor = "teaclave"))]
mod trusted;
#[cfg(all(feature = "dcap", target_vendor = "teaclave"))]
pub use trusted::*;

pub use sgx_dcap_ql_rs::{quote3_error_t, sgx_qe_get_target_info, sgx_report_t, sgx_target_info_t};

pub type Result<T> = core::result::Result<T, quote3_error_t>;

#[cfg(all(feature = "dcap", target_vendor = "teaclave"))]
mod api {
    // trusted
    use super::*;

    pub fn dcap_quote(data: [u8; 64]) -> Result<Vec<u8>> {
        let target = Dcap::get_target()?;
        let report = Dcap::create_report(target, data)?;
        let quote = Dcap::get_quote(&report)?;
        Ok(quote)
    }
}

#[cfg(all(feature = "dcap", not(target_vendor = "teaclave")))]
mod api {
    // untrusted
    use super::*;
    
    pub fn dcap_quote(_data: [u8; 64]) -> Result<Vec<u8>> {
        panic!("unable to generate DCAP quote in untrusted env");
    }
}

pub use api::*;
