use bevy::{
    ecs::{archetype::Archetypes, component::ComponentId},
    prelude::{Changed, Component, Entity, Event, Query, Resource, World},
    reflect::{Reflect, TypeRegistry, reflect_trait},
};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::*;
use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

pub mod plugin;
pub use plugin::*;

pub mod commands;
pub use commands::*;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct RouteState {
    pub path: Vec<String>,
    pub params: HashMap<String, String>,
}

impl RouteState {
    pub fn to_string(&self) -> String {
        let mut route = self.path.join("/");
        if self.params.len() > 0 {
            route = route + "?" + &to_url_params(&self.params);
        }
        return route;
    }
}

pub(crate) fn to_url_params(params: &HashMap<String, String>) -> String {
    // Routing compares serialized queries, so map iteration must not change the URL.
    let mut pairs: Vec<_> = params.iter().collect();
    pairs.sort_unstable_by(|left, right| left.0.cmp(right.0));
    url::form_urlencoded::Serializer::new(String::new())
        .extend_pairs(pairs)
        .finish()
}

#[cfg(test)]
mod routing_tests {
    use super::*;

    #[test]
    fn query_values_round_trip_without_becoming_extra_parameters() {
        let params = HashMap::from([
            ("search".to_owned(), "a&b=c + #?".to_owned()),
            ("odd key&".to_owned(), "".to_owned()),
        ]);
        let encoded = to_url_params(&params);
        let decoded: HashMap<String, String> =
            url::form_urlencoded::parse(encoded.as_bytes()).into_owned().collect();
        assert_eq!(decoded, params);
    }

    #[test]
    fn query_order_is_stable_and_empty_routes_stay_clean() {
        let params = HashMap::from([
            ("z".to_owned(), "last".to_owned()),
            ("a".to_owned(), "first value".to_owned()),
        ]);
        assert_eq!(to_url_params(&params), "a=first+value&z=last");
        assert_eq!(to_url_params(&HashMap::new()), "");
        assert_eq!(RouteState { path: vec!["devices".into()], params }.to_string(),
            "devices?a=first+value&z=last");
        assert_eq!(RouteState::default().to_string(), "");
    }
}

#[derive(Default, Event, Clone)]
pub struct RouteChange {
    pub path: Vec<String>,
    pub params: HashMap<String, String>,
}

#[derive(Debug, Clone, Component)]
pub struct Binding {
    //<TSource, TTarget> where TSource: Component, TTarget: Component {
    pub source_entity_id: Entity,
    pub source_component_id: usize,
    pub source_property_name: String,
    pub target_entity_id: Entity,
    pub target_component_id: usize,
    pub target_property_name: String,
}

pub fn process_bindings<T>(_parent_query: Query<(Entity, Ref<T>)>)
where
    T: Component,
{
}

#[derive(Debug, Clone, Default, Resource)]
pub struct ChangedComponents {
    pub changed_components: Vec<(Entity, usize)>,
}

#[derive(Debug, Clone, Default, Resource)]
pub struct Bindings {
    pub sources: HashMap<(Entity, usize), Binding>,
}

/*
pub fn get_components_for_entity<'a>(
    entity: &Entity,
    archetypes: &'a Archetypes,
) -> Option<impl Iterator<Item = ComponentId> + 'a> {
    for archetype in archetypes.iter() {
        //archetype.entities().get(index)
        //for entity in archetype.entities().iter() {
        //    entity.table_row()
        //}

        //archetype.table_id()
        if archetype.entities().iter().any(|e| e.entity() == *entity) {
            return Some(archetype.components());
        }
    }
    None
}


pub fn process(world: &mut World) {
    /*
    let mut system_state: SystemState<(Query<(Entity, &mut Binding)>)> = SystemState::new(world);

    //let mut w_3 = w_3.lock().unwrap();
    let (query) = system_state.get_mut(world);

    let mut source_component_ids:Vec<usize> = Vec::<usize>::new();

    for (entity, mut binding) in &query {
        source_component_ids.push(binding.source_component_id);
    }

    let mut system_state: SystemState<(Query<(Entity, &mut Binding)>)> = SystemState::new(world);
    */

    //let mut query = DynamicQuery::new(world, vec![FetchKind::Ref(ComponentId::new(0))], vec![FilterKind::Without(ComponentId::new(0))]);
    //assert_eq!(query.iter().count(), 1);

    //let query: EcsValueRefQuery;

    let type_registry = TypeRegistry::default();
    let type_registry = type_registry.read();

    let archetypes = world.archetypes();
    let entities = world.iter_entities();
    //world.components().iter();
    for entity in entities {
        let components = get_components_for_entity(&entity.id(), archetypes).unwrap();
        for component in components {
            let info = world.components().get_info(component).unwrap();
            let type_id = info.type_id();
            if type_id.is_some() {
                let type_id = type_id.unwrap();
                //let id: u64 = type_id.try_into().unwrap();
                //console::log!(id.to_string());
                let type_info = type_registry.get_type_info(type_id);
                if type_info.is_some() {
                    let type_info = type_info.unwrap();
                    //console::log!(type_info.type_name());
                }
            }
            //type_data.
        }
    }

    /*
    let es:Vec<Entity> = world.iter_entities().collect();
    //world.components().iter();
    for e in es {
        for c in world.get_entity_mut(e).unwrap().archetype().components() {


            //world.get_by_id(e, c);
            //let info = world.components().get_info(c);
            //if (info.unwrap().type_id() == Some(TypeId::of::<ChatInput>())) {

            //}
        }
    }
    */

    //world.init_resource::<Binding>();
}
*/
