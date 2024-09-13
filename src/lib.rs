#![cfg_attr(target_vendor = "teaclave", feature(rustc_private))]
#![feature(panic_unwind)]

pub mod sgxlib;
pub mod types;

mod patch;

pub mod app;
pub mod dcap;

pub use ctor::ctor;

#[cfg(feature = "builder")]
pub use automata_build_script::*;