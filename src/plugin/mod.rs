use bevy::{
    ecs::{archetype::Archetypes, component::ComponentId},
    prelude::{Changed, Component, Entity, Query, Resource, World, Event},
    reflect::{TypeRegistry, reflect_trait, Reflect}
};

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::*;
use bevy::prelude::*;
use bevy_trait_query::RegisterExt;

pub mod plugin;
pub use plugin::*;

use bevy::{prelude::*};

#[derive(Default, Event)]
pub struct RouteChange {
    pub path: Vec<String>,
    pub params: HashMap<String, String>//HashMap<String, Box<dyn Reflect>>
}

/*
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomMessage {
    pub id: Uuid,
    pub msg: String,
}
*/

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