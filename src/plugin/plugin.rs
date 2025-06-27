use bevy::{ecs::schedule::ScheduleConfigs, prelude::*};
#[cfg(feature = "native_ui")]
use bevy_simple_subsecond_system::SimpleSubsecondSystemSet;
use flux::prelude::*;
use crate::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use bevy_trait_query::RegisterExt;

pub struct BevyNative;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UISets {
    RenderUI,
}

impl Plugin for BevyNative {
    fn build(&self, app: &mut App) {

        #[cfg(feature = "native_ui")]
        #[cfg(target_arch = "wasm32")]
        app
        .register_component_as::<dyn BindableList, AutoBindableList>()
        //.register_component_as::<dyn Bindable, AutoBindable>()
        //.register_component_as::<dyn Bindable, UsageView>()
        //.register_component_as::<dyn Bindable, InteractState>()
        //.register_component_as::<dyn Bindable, SearchInput>()
        //.add_plugins((bevy::MinimalPlugins, bevy::hierarchy::HierarchyPlugin))
        .insert_resource(flux::prelude::BindingsConfig::default())
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
        .configure_sets(PreUpdate, (
            UISets::RenderUI.after(SimpleSubsecondSystemSet::UpdateFunctionPtrs),
        ))
        .add_systems(PreUpdate,
            render_ui().in_set(UISets::RenderUI)
        )
        .add_systems(Update,
            render_ui()
        )
        .add_systems(PostUpdate,
            (
                base_change_detection,
                list_change_detection,
                //update_heirarchy,
                event_detection,
                //propogate_forms,
                //process_form_on_submit,
                //process_responsive_elements,
                remove_detection
            ).chain()
        ).add_observer(update_heirarchy)
        //.add_systems(PostStartup, route_detection)
        .add_systems(Startup, setup);
    }
}

pub fn render_ui() -> impl IntoScheduleConfigs<Box<dyn System<In = (), Out = Result<(), BevyError>>>, ()> {
    (
        update_route,
        route_detection,
        on_show_detection,

 
        base_change_detection,
        list_change_detection,
        
        //update_heirarchy,

        //taby_client::network_detection,
        //update_route,
        //route_detection,
        //on_show_detection,
  
        event_detection,
        base_change_detection,
        list_change_detection,

        // TODO: Handle recursive updating better--currently just reruns basic updates afterwards
        // May modify width and heights of elements based on other elements updated
        process_responsive_elements,

        //taby_client::process,
        //main_view::process,
        //process_search_input,
        //process_sliders,
        //propogate_forms,
        process_form_on_submit,
        //base_change_detection,
        remove_detection
    ).chain()
}