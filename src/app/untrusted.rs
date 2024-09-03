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
        pub struct $enclave_name();

        extern "C" {
            $(
            fn $fn_name($($arg_name: $arg_type,)* retval: *mut $crate::types::SgxStatus)  $(-> $ret_type)?;
            )*
        }

        impl $enclave_name {
            pub fn new(_debug: bool) -> $crate::types::SgxResult<Self> {
                Ok($enclave_name())
            }

            $(
                pub fn $fn_name(&self, $($arg_name: $arg_type),*) -> $crate::types::SgxResult<$($ret_type)?> {
                    eprintln!("{}", "=".repeat(80));
                    eprintln!("WARNING: Currently running in untrusted mode, for development use only");
                    eprintln!("{}", "=".repeat(80));
                    let mut retval = $crate::types::SgxStatus::Success;
                    let ret = unsafe {
                        $fn_name($($arg_name,)* &mut retval)
                    };
                    if retval != $crate::types::SgxStatus::Success {
                        return Err(retval);
                    }
                    Ok(ret)
                }
            )*
        }
    }
}
