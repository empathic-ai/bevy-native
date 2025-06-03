use crate::*;
use bevy::reflect::{List, DynamicList, DynamicStruct};
use flux::prelude::*;

use std::collections::HashMap;

//use bevy::hierarchy::HierarchyEvent;
use bevy::prelude::*;

use bevy::ecs::event::{Event, EventWriter};

use common::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};

use web_sys::*;

pub fn route_detection(mut commands: Commands,
    query: Query<(
        Entity,
        Ref<Router>,
        &Children
    )>,
    mut route_query: Query<(
        Entity,
        &mut Control,
        &Route,
        Option<&mut AutoBindableProperty>
    ), Without<Router>>
) {
    for (entity, router, children) in &query {
        if router.is_changed() {
            if router.path.len() > 0 {
                let first_path = router.path[0].clone();
            }

            for child in children.iter() {
                let mut is_path_part = true;
                if let Ok((_, _, mut route, _)) = route_query.get_mut(child) {
                    is_path_part = router.path.len() > 0 && (route.name == router.path[0]);
                }
                if let Ok((entity, mut control, _, bindable)) = route_query.get_mut(child) {
                    control.is_visible = is_path_part;
                    if is_path_part {
                        commands.trigger_targets(ShowView { params: router.params.clone() }, entity);
                    }
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
    set_route_simple(path);

    route();
    //window. .pushState('page2', 'Title', '/page2.php');
}

pub fn set_route_simple(mut path: &str) {
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
    mut query: bevy::prelude::Query<(Entity, &mut Router)>, mut evs: ResMut<Events<RouteChange>>) {

    if let Ok((_, mut router)) = query.get_single_mut() {

        let (tx, rx) = &mut *ROUTE_CHANNEL.lock().unwrap();
        match rx.try_recv() {
            Ok(ev) => {

                    let params = ev.params.clone(); //ev.params.iter().map(|(key, value)| (key.clone(), reflect_to_json(value.as_reflect()).to_string())).collect();
                    if router.path != ev.path || router.params != params {
                        router.path = ev.path.clone();
                        router.params = params;
                        evs.send(ev);
                    }
    
                    /*
                    for (entity, mut router) in query.iter_mut() {
                        //console::log!(format!("UPDATING ROUTER"));
                        router.path = ev.path.clone();
                        router.params = ev.params.clone();
                    }
        
                    let new_route = ev.path.join("/");
                    if get_route().trim_start_matches('/') != new_route || get_route_params() != ev.params {
                        let new_route = new_route + "/" + &to_url_params(&ev.params);
                        info!("New route: {}", new_route);
                        set_route(&new_route);
                    }*/    

            }
            Err(_) => {
            }
        }
        
        for ev in evs.get_cursor().read(&evs) {
            let params = ev.params.clone();//.iter().map(|(key, value)| (key.clone(), reflect_to_json(value.as_reflect()).to_string())).collect();

            if router.path != ev.path || router.params != params {
                router.path = ev.path.clone();
                router.params = params.clone();

                let mut new_route = ev.path.join("/");
                let params = to_url_params(&params);

                if get_route().trim_start_matches('/') != new_route || to_url_params(&get_route_params()) != params {
                    if ev.params.len() > 0 {
                        new_route = new_route + "?" + &params;
                    }
            
                    //info!("New route: {}", new_route);
                    set_route_simple(&new_route);
                }
            }
        }
    }
}

pub fn route() {
    let route = get_route();

    let params = to_url_params(&get_route_params());
    //info!("Browser route: {}", route);
    //info!("Browser params: {}", params);

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