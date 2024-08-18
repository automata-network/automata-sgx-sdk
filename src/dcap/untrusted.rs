use sgx_dcap_ql_rs::{
    quote3_error_t, sgx_qe_get_quote, sgx_qe_get_target_info, sgx_report_t, sgx_target_info_t,
};

#[no_mangle]
pub unsafe extern "C" fn dcap_get_target(target: *mut sgx_target_info_t) -> quote3_error_t {
    sgx_qe_get_target_info(target.as_mut().unwrap())
}

#[no_mangle]
pub unsafe extern "C" fn dcap_get_quote(
    report: *const sgx_report_t,
    out_size: usize,
    out: *mut u8,
    fill_out_len: *mut usize,
) -> quote3_error_t {
    let (err, quote) = sgx_qe_get_quote(report.as_ref().unwrap());
    let quote = quote.unwrap_or_default();
    if quote.len() > out_size {
        println!("buffer too small");
        panic!();
    }
    *fill_out_len = quote.len();
    let msg_out = std::slice::from_raw_parts_mut(out, quote.len());
    msg_out.copy_from_slice(&quote);
    err
}
