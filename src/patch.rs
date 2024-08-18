#[cfg(feature = "dummy_pthread_atfork")]
#[no_mangle]
pub unsafe extern "C" fn pthread_atfork(
    _: Option<unsafe extern "C" fn()>,
    _: Option<unsafe extern "C" fn()>,
    _: Option<unsafe extern "C" fn()>,
) -> std::ffi::c_int {
    0
}

#[cfg(feature = "patch_assert_fail")]
#[no_mangle]
pub extern "C" fn __assert_fail(
    __assertion: *const u8,
    __file: *const u8,
    __line: u32,
    __function: *const u8,
) -> ! {
    use std::ffi::CStr;
    use crate::sgxlib::sgx_libc::abort;

    let assertion = unsafe { CStr::from_ptr(__assertion as *const _).to_str() }
        .expect("__assertion is not a valid c-string!");
    let file = unsafe { CStr::from_ptr(__file as *const _).to_str() }
        .expect("__file is not a valid c-string!");
    let line = unsafe { CStr::from_ptr(__line as *const _).to_str() }
        .expect("__line is not a valid c-string!");
    let function = unsafe { CStr::from_ptr(__function as *const _).to_str() }
        .expect("__function is not a valid c-string!");
    println!("{}:{}:{}:{}", file, line, function, assertion);

    unsafe { abort() }
}
