use bevy::prelude::*;
use bevy_builder::prelude::*;
use crate::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bevy_trait_query::RegisterExt;

pub struct SilkPlugin;

impl Plugin for SilkPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_component_as::<dyn BindableList, AutoBindableList>()
        .register_component_as::<dyn Bindable, AutoBindable>()
        .register_component_as::<dyn Bindable, UsageView>()
        .register_component_as::<dyn Bindable, InteractState>()
        .register_component_as::<dyn Bindable, SearchInput>()
        .add_plugins(bevy::MinimalPlugins)
        .add_plugin(bevy::hierarchy::HierarchyPlugin)
        .insert_resource(bevy_builder::prelude::Bindings { ..default() } )
        //.insert_resource(client)
        .add_event::<RouteChange>()
        .add_event::<LogoutEvent>()
        .add_event::<HostGameEvent>()
        .add_event::<UpdateChatEvent>()
        //.add_event::<CancelSubscriptionEvent>()
        //.add_event::<aws_client::NetworkEvent>()
        .add_event::<SignUpEvent>()
        .add_event::<LogInEvent>()
        .add_event::<ClickEvent>()
        .add_event::<SubmitEvent>()
        .add_event::<SnapScrollY>()
        .add_systems(Update,
            (
                route_detection,
                base_change_detection,
                list_change_detection,
                update_heirarchy,
                //taby_client::network_detection,
                update_route,
                route_detection,
                event_detection,
                base_change_detection,
                list_change_detection,
                process_responsive_elements,
                //taby_client::process,
                //main_view::process,
                //process_search_input,
                //process_sliders,
                propogate_forms,
                process_form_on_submit,
                //base_change_detection,
                remove_detection
            ).chain()
        ).add_systems(PostUpdate,
            (
                base_change_detection,
                list_change_detection,
                update_heirarchy,
                event_detection,
                //propogate_forms,
                //process_form_on_submit,
                //process_responsive_elements,
                remove_detection
            ).chain()
        )
        .add_systems(Update,
            (
                on_show_detection
            ).chain()
        )
        .add_startup_system(setup);
    }
}