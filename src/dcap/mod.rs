#[cfg(all(feature = "dcap", not(target_vendor = "teaclave")))]
mod untrusted;
#[cfg(all(feature = "dcap", not(target_vendor = "teaclave")))]
pub use untrusted::*;

#[cfg(all(feature = "dcap", target_vendor = "teaclave"))]
mod trusted;
#[cfg(all(feature = "dcap", target_vendor = "teaclave"))]
pub use trusted::*;

pub type Result<T> = core::result::Result<T, DcapError>;

base::stack_error! {
    name: DcapError,
    stack_name: DcapErrorStack,
    error: {
        Quote3(String),
        Unsupported,
    },
    stack: {
        GetTarget(),
        CreateReport(),
        GetQuote(),
    }
}

#[cfg(all(feature = "dcap", target_vendor = "teaclave"))]
mod api {
    // trusted
    use super::*;

    impl From<sgx_dcap_ql_rs::quote3_error_t> for DcapError {
        fn from(err: sgx_dcap_ql_rs::quote3_error_t) -> DcapError {
            DcapError::Quote3(format!("{:?}", err))
        }
    }

    pub fn dcap_quote(data: [u8; 64]) -> Result<Vec<u8>> {
        let target = Dcap::get_target().map_err(DcapError::GetTarget())?;
        let report = Dcap::create_report(target, data).map_err(DcapError::CreateReport())?;
        let quote = Dcap::get_quote(&report).map_err(DcapError::GetQuote())?;
        Ok(quote)
    }
}

#[cfg(any(
    not(feature = "dcap"),
    all(feature = "dcap", not(target_vendor = "teaclave"))
))]
mod api {
    // untrusted
    use super::*;

    pub fn dcap_quote(_data: [u8; 64]) -> Result<Vec<u8>> {
        Err(DcapError::Unsupported)
    }
}

pub use api::*;
