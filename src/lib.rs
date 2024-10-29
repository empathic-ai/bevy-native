#![allow(warnings)]
#![allow(unused)]

mod platforms;
use std::collections::HashMap;

pub use platforms::*;

mod plugin;
pub use plugin::*;

use bevy::{prelude::*};

#[derive(Default, Event)]
pub struct RouteChange {
    pub path: Vec<String>,
    pub params: HashMap<String, String>
}