use std::collections::HashMap;

use bevy::{ecs::system::SystemParam, prelude::*};
use flux::prelude::*;

use crate::RouteChange;

#[derive(SystemParam)]
pub struct NativeCommands<'w, 's> {
    commands: Commands<'w, 's>,
    route_ev: EventWriter<'w, RouteChange>,
}

impl NativeCommands<'_, '_> {


}

pub trait NetworkCommandsExt {
    fn set_route_with_id(&mut self, route: String, id: Id);
    fn set_route(&mut self, route: String, params: HashMap<String, String>);
}

// implement our trait for Bevy's `Commands`
impl<'w, 's> NetworkCommandsExt for Commands<'w, 's> {
    fn set_route_with_id(&mut self, route: String, id: Id) {
        let mut params = HashMap::new();
        params.insert("id".to_string(), id.to_string());
        self.set_route(route, params);
    }

    fn set_route(&mut self, route: String, params: HashMap<String, String>) {
        self.send_event(RouteChange {
            path: vec![route],
            params: params
        });
    }
}