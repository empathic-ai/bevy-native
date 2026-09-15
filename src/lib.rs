#![allow(warnings)]
#![allow(unused)]

mod layout;

mod platforms;
use std::collections::HashMap;

pub use platforms::*;

#[cfg(not(target_arch = "xtensa"))]
#[cfg(feature = "bevy")]
mod plugin;
#[cfg(not(target_arch = "xtensa"))]
#[cfg(feature = "bevy")]
pub use plugin::*;
