use crate::*;
use bevy_builder::prelude::*;

use bevy::utils::HashMap;

use bevy::hierarchy::HierarchyEvent;
use bevy::prelude::*;

use bevy::ecs::event::{Event, EventWriter};

use wasm_bindgen::{prelude::*, JsCast};

use web_sys::*;

pub fn route_detection(
    query: Query<(
        Entity,
        &Router,
        &Children,
        Changed<Router>,
    )>,
    mut route_query: Query<(
        Entity,
        &mut Control,
        &Route
    ), Without<Router>>
) {
    for (entity, router, children, router_changed) in &query {
        if router_changed {
            //let first_path = router.path[0].clone();
            //console::log!(format!("ROUTER CHANGED"));

            for child in children.iter() {
                let mut is_path_part = true;
                if let Ok(mut route) = route_query.get_component::<Route>(*child) {
                    is_path_part = router.path.len() > 0 && (route.name == router.path[0]);
                }
                if let Ok(mut control) = route_query.get_component_mut::<Control>(*child) {
                    control.IsVisible = is_path_part;
                }
            }
        }
    }
}

pub fn get_route() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let path = location.pathname().unwrap();

    return path;
}

pub fn get_route_params() -> HashMap<String, String> {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();

    let search = location.search().unwrap(); // This gets the part of the URL after the `?`
    let params = UrlSearchParams::new_with_str(&search).unwrap();

    let params = convert_to_dictionary(params);

    return params;
}

pub fn convert_to_dictionary(search_params: UrlSearchParams) ->  HashMap<String, String> {
    let mut dictionary: HashMap<String, String> = HashMap::new();

    let iterator = js_sys::try_iter(&search_params).unwrap().unwrap();
    for x in iterator {
        let item = x.unwrap();
        let key = unsafe { js_sys::Reflect::get(&item, &JsValue::from_str("0")).unwrap().as_string().unwrap() };
        let value = unsafe { js_sys::Reflect::get(&item, &JsValue::from_str("1")).unwrap().as_string().unwrap() };
        dictionary.insert(key, value);
    }

    dictionary
}

pub fn set_route(mut path: &str) {
    use wasm_bindgen::JsValue;

    let mut path = path.to_string();
    let window = web_sys::window().expect("no global `window` exists");
    if path == "".to_string() {
        path = "/".to_string();
    }
    window.history().unwrap().push_state_with_url(&JsValue::from_str(""), "Taby", Some(&path)).unwrap();

    route();
    //window. .pushState('page2', 'Title', '/page2.php');
}

pub fn update_route(
    mut query: bevy::prelude::Query<(Entity, &mut Router)>, mut ev_writer: EventWriter<RouteChange>) {


    let (tx, rx) = &mut *ROUTE_CHANNEL.lock().unwrap();

    match rx.try_recv() {
        Ok(ev) => {
            for (entity, mut router) in query.iter_mut() {
                //console::log!(format!("UPDATING ROUTER"));
                router.path = ev.path.clone();
                router.params = ev.params.clone();
            }

            let new_route = ev.path.join("/");
            if get_route().trim_start_matches('/') != new_route || get_route_params() != ev.params {
                set_route(&new_route);
            }

            ev_writer.send(ev);
        }
        Err(_) => {
        }
    }
}

pub fn route() {
    let route = get_route();

    let (tx, rx) = &mut *ROUTE_CHANNEL.lock().unwrap();

    //console::log!(format!("ROUTE CHANGE: {route}"));

    let path_list = split_path_to_list(&route.trim_start_matches('/').trim_end_matches('/'));

    tx.send(RouteChange{
        path: path_list,
        params: get_route_params()
    });
    /* 
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let path = location.pathname().unwrap();

    let search = location.search().unwrap(); // This gets the part of the URL after the `?`
    let params = UrlSearchParams::new_with_str(&search).unwrap();

    match path.as_str() {
        "/login" => login(aws_client, params.get("code").unwrap()),
        _ => home(),
    }
    */
}

fn split_path_to_list(path: &str) -> Vec<String> {
    path.split('/')
        .map(|s| s.to_string())
        .collect()
}