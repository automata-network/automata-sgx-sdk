#[cfg(all(feature = "dcap", not(target_vendor = "teaclave")))]
mod untrusted;
#[cfg(all(feature = "dcap", not(target_vendor = "teaclave")))]
pub use untrusted::*;

#[cfg(all(feature = "dcap", target_vendor = "teaclave"))]
mod trusted;
#[cfg(all(feature = "dcap", target_vendor = "teaclave"))]
pub use trusted::*;

pub type Result<T> = core::result::Result<T, String>;

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

#[cfg(any(not(feature = "dcap"), all(feature = "dcap", not(target_vendor = "teaclave"))))]
mod api {
    // untrusted
    use super::*;
    
    pub fn dcap_quote(_data: [u8; 64]) -> Result<Vec<u8>> {
        Err("unable to generate DCAP quote in untrusted env".into())
    }
}

pub use api::*;
