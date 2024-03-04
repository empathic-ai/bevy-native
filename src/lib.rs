#![allow(warnings)]
#![allow(unused)]

mod platforms;
pub use platforms::*;

mod plugin;
pub use plugin::*;

use bevy::{prelude::*, utils::HashMap};

#[derive(Default, Event)]
pub struct RouteChange {
    pub path: Vec<String>,
    pub params: HashMap<String, String>
}