mod builders;
pub use builders::*;

mod env;
pub use env::*;

mod cutils;
pub use cutils::*;

mod sgx_app;
pub use sgx_app::*;
mod std_app;
pub use std_app::*;

pub fn build_app() {
    match Env::cargo_sgx_output() {
        Some(n) if !n.std_mode => build_sgx_app(),
        _ => build_std_app(),
    }
}
