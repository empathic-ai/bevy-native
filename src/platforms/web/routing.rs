use bevy::reflect::{List, DynamicList, DynamicStruct};
use flux::prelude::*;

use std::collections::HashMap;

//use bevy::hierarchy::HierarchyEvent;
use bevy::prelude::*;

use bevy::ecs::event::{Event, EventWriter};

use common::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

use web_sys::*;

use crate::{ROUTE_CHANNEL, RouteChange};

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

pub fn route_detection(
    mut commands: Commands,
    mut route_state: ResMut<RouteState>,
    query: Query<(
        Entity,
        &Router,
        &Children,
    )>,
    mut route_query: Query<(
        Entity,
        &mut Control,
        &Route,
        Option<&mut AutoBindableProperty>
    ), Without<Router>>
) {
    /*
    for (entity, router, children) in &query {
        let route_name = route_state.path.first().map(String::as_str);
        let mut matched_entity = None;

        for child in children.iter() {
            let Ok((child_entity, mut control, route, _)) = route_query.get_mut(child) else {
                continue;
            };

            let is_match = route_name.is_some_and(|name| name == route.name);
            control.is_visible = is_match;
            if is_match {
                matched_entity = Some(child_entity);
            }
        }

        if let Some(matched_entity) = matched_entity {
            if route_state.matched_generation != route_state.generation {
                route_state.matched_generation = route_state.generation;

                info!("Sending show view event for route: {:?}", route_state.path);

                commands.trigger_targets(
                    ShowView {
                        params: route_state.params.clone(),
                    },
                    matched_entity,
                );
            }
        }
    }
    */
}

pub fn get_native_route_path() -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let path = location.pathname().unwrap();

    return path;
}

pub fn get_native_route_params() -> HashMap<String, String> {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();

    let search = location.search().unwrap(); // This gets the part of the URL after the `?`
    let params = UrlSearchParams::new_with_str(&search).unwrap();

    let params = convert_to_dictionary(params);

    return params;
}

fn to_url_params(params: &HashMap<String, String>) -> String {
    let mut url_params = String::new();

    for (key, value) in params {
        if !url_params.is_empty() {
            url_params.push('&');
        }
        url_params.push_str(&format!("{}={}", key, value));
    }

    url_params
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
    update_native_route_history(path);

    send_route_change();
    //window. .pushState('page2', 'Title', '/page2.php');
}

pub fn update_native_route_history(mut path: &str) {
    use wasm_bindgen::JsValue;

    let mut path = path.to_string();
    let window = web_sys::window().expect("no global `window` exists");
    if path == "".to_string() {
        path = "/".to_string();
    }
    window.history().unwrap().push_state_with_url(&JsValue::from_str(""), "", Some(&path)).unwrap();

    //route();
    //window. .pushState('page2', 'Title', '/page2.php');
}

/*
#[cfg(not(target_arch = "xtensa"))]
pub fn map_route() {
    for ev in ev_reader.read() {
        if ev.path.len() == 0 || ev.path[0] == "" || ev.path[0] == "/" {
            //ev_writer.send(RouteChange {
            //    path: (&["sign_up".to_string()]).to_vec(),
            //    ..default()
            //});
            /*
            if !client.user.lock().unwrap().is_none() {
                set_route("multi_chat");
            } else {
                set_route(DEFAULT_TABY_ROUTE);
            }
            */
        }
        //if ev.path.len() > 0 && ev.path[0] == "profile" {
        //    if client.id_token.lock().unwrap().is_none() {
        //        set_route("entry".to_string());
        //    }
        //}
    }
} */

pub fn update_route(
    mut commands: Commands,
    mut query: bevy::prelude::Query<(Entity, Ref<Router>, Ref<Children>)>,
    mut evs: ResMut<Events<RouteChange>>,
    mut route_state: ResMut<RouteState>,
    mut route_query: Query<(
        Entity,
        &mut Control,
        &Route,
        Option<&mut AutoBindableProperty>
    ), Without<Router>>
) {

    let route_change_ev = if let Some(route_change_ev) = evs.get_cursor().read(&evs).last() {
        Some(route_change_ev.clone())
    }
    else {
        let mut route_change_ev = None;
        {
            let (_, rx) = &mut *ROUTE_CHANNEL.lock().unwrap();
            while let Ok(event) = rx.try_recv() {
                route_change_ev = Some(event);
            }
        }
        route_change_ev
    };

    let mut is_router_state_changed = false;

    if let Some(route_change_ev) = route_change_ev {

        route_state.path = route_change_ev.path.clone();
        route_state.params = route_change_ev.params.clone();

        is_router_state_changed = true;

        let mut route_path = route_state.path.join("/");
        let route_params = to_url_params(&route_state.params);

        // Update the browser's URL if it doesn't match the current route and params
        if get_native_route_path().trim_start_matches('/') != route_path || to_url_params(&get_native_route_params()) != route_params {
            if route_state.params.len() > 0 {
                route_path = route_path + "?" + &route_params;
            }
    
            update_native_route_history(&route_path);
        }
    }

    if let Ok((_, mut router, children)) = query.get_single_mut() {

        if is_router_state_changed || router.is_added() || children.is_changed() {

            info!("Updating router to route: {:?}", route_state.to_string());    

            let route_name = route_state.path.first().map(String::as_str);
            let mut matched_entity = None;

            for child in children.iter() {
                let Ok((child_entity, mut control, route, _)) = route_query.get_mut(child) else {
                    continue;
                };

                let is_match = route_name.is_some_and(|name| name == route.name);
                control.is_visible = is_match;
                commands.entity(child_entity).insert(RouteVisibilityResolved);
                if is_match {
                    matched_entity = Some(child_entity);
                }
            }

            if let Some(matched_entity) = matched_entity {
                //if route_state.matched_generation != route_state.generation {
                //    route_state.matched_generation = route_state.generation;

                    info!("Sending show view event for route: {:?}", route_state.to_string());

                    // Apply child construction before the first route event.
                    commands.queue(mount_visible_views);
                    commands.trigger_targets(
                        ShowView {
                            params: route_state.params.clone(),
                        },
                        matched_entity,
                    );
                //}
            }
        }
    }
}

pub fn send_route_change() {
    let route = get_native_route_path();

    let params = to_url_params(&get_native_route_params());
    //info!("Browser route: {}", route);
    //info!("Browser params: {}", params);

    let (tx, rx) = &mut *ROUTE_CHANNEL.lock().unwrap();

    //console::log!(format!("ROUTE CHANGE: {route}"));

    let path_list = split_path_to_list(&route.trim_start_matches('/').trim_end_matches('/'));

    tx.send(RouteChange{
        path: path_list,
        params: get_native_route_params()
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