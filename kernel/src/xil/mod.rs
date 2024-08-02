//! Xilinx embeddedsw bindings.
//!
//! This module provides bindings to the Xilinx embeddedsw HAL library (libxil).

#![allow(
    unused_variables,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case,
    clippy::missing_safety_doc
)]

pub mod exception;
pub mod gic;
pub mod time;
pub mod timer;
pub mod uart;
pub mod wdt;
