#![allow(warnings)]
#![allow(unused)]

mod platforms;
use std::collections::HashMap;

pub use platforms::*;

#[cfg(not(target_arch = "xtensa"))]
mod plugin;
#[cfg(not(target_arch = "xtensa"))]
pub use plugin::*;
