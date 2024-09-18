use sgx_dcap_ql_rs::{quote3_error_t, sgx_report_t, sgx_target_info_t};
use super::Result;

extern "C" {
    fn dcap_get_target(retval: &mut quote3_error_t, target: &mut sgx_target_info_t);
    fn dcap_get_quote(
        retval: &mut quote3_error_t,
        target: &sgx_report_t,
        out_size: usize,
        out: *mut u8,
        filled: *mut usize,
    );
}

pub struct Dcap {}

impl Dcap {
    pub fn get_target() -> Result<sgx_target_info_t> {
        let mut target = sgx_target_info_t::default();
        let mut ret = quote3_error_t::SGX_QL_SUCCESS;
        unsafe { dcap_get_target(&mut ret, &mut target) };

        if ret == quote3_error_t::SGX_QL_SUCCESS {
            Ok(target)
        } else {
            Err(ret.into())
        }
    }

    pub fn create_report(target: sgx_target_info_t, data: [u8; 64]) -> Result<sgx_report_t> {
        use crate::sgxlib::sgx_trts::se::{AlignReport, AlignReportData, AlignTargetInfo};

        let target = AlignTargetInfo(unsafe { std::mem::transmute(target) });
        let report_data = { AlignReportData(unsafe { std::mem::transmute(data) }) };
        let result = AlignReport::for_target(&target, &report_data).unwrap();
        Ok(unsafe { std::mem::transmute(result.0) })
    }

    pub fn get_quote(report: &sgx_report_t) -> Result<Vec<u8>> {
        let mut out = vec![0_u8; 25600];
        let mut ret = quote3_error_t::SGX_QL_SUCCESS;
        let mut filled = 0;
        unsafe { dcap_get_quote(&mut ret, report, out.len(), out.as_mut_ptr(), &mut filled) };

        if ret == quote3_error_t::SGX_QL_SUCCESS {
            out.truncate(filled);
            Ok(out)
        } else {
            Err(ret.into())
        }
    }
}
