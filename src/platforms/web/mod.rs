use crate::prelude::*;
use bevy_builder::*;

use super::{elements, ClickEvent, SubmitEvent, OnClick, BindableChanged, OnShow};
use bevy::utils::HashMap;

use bevy::hierarchy::HierarchyEvent;
use bevy::prelude::{Changed, Children, Color, Commands, Entity, Parent, Query, RemovedComponents, EventReader, Without, Vec4};
use bevy::ui::BackgroundColor;

use bevy::ecs::event::{Event, EventWriter};

use wasm_bindgen::{prelude::*, JsCast};

use std::sync::{Mutex, mpsc};

use web_sys::{Document, Element, HtmlIFrameElement, HtmlInputElement, Window, window, UrlSearchParams};
use url::Url;

use elements::{Button, Label};

use lazy_static::lazy_static;
use std::sync::mpsc::{Receiver, Sender};

use web_sys::HtmlDivElement;

use base64::{engine::general_purpose, Engine as _};

lazy_static! {
    pub static ref ROUTE_CHANNEL: Mutex<(Sender<RouteChange>, Receiver<RouteChange>)> = {
        let (tx, rx) = mpsc::channel();
        Mutex::new((tx, rx))
    };
}

#[derive(Default, Event)]
pub struct TranscribeEvent(pub String);

#[derive(Default, Event)]
pub struct ResponseEvent(pub String);

pub fn setup() {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();

    let closure = Closure::wrap(Box::new(move || {
        route();
    }) as Box<dyn FnMut()>);

    window.add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();

    route();
}

pub fn create_iframe_element() -> Result<(), JsValue> {
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let element = document.create_element("iframe")?;
    let iframe = element.clone().dyn_into::<HtmlIFrameElement>()?;

    iframe.set_width("560");
    iframe.set_height("315");
    iframe.set_src("https://www.youtube.com/embed/xfDzXkRhFsE");
    iframe.set_title("YouTube video player");
    iframe.set_frame_border("0");
    //iframe.set_allow("accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share;");
    iframe.set_allow_fullscreen(true);

    //let style = iframe.style();
    element.set_attribute(
        "style",
        "border-radius: 10px; box-shadow: 0 1px 6px rgb(32 33 36 / 28%)",
    );

    body.append_child(&iframe)?;

    Ok(())
}

pub fn iframe_change_detection(
    _commands: Commands,
    _query: Query<(Entity, &Control, &IFrame, Changed<IFrame>)>,
) {
}

pub fn label_change_detection(
    _commands: Commands,
    _query: Query<(Entity, &Control, &Label, Changed<Label>)>,
) {
}

pub fn list_change_detection(
    _commands: Commands,
    query: Query<(
        Entity,
        &Control,
        &Container,
        Option<&VList>,
        Option<&HList>,
        Option<&GridList>,
        Option<Changed<Control>>,
        Option<Changed<Parent>>,
        Option<Changed<VList>>,
        Option<Changed<HList>>,
        Option<Changed<GridList>>,
    )>) {
    for (
        entity,
        control,
        _contianer,
        vlist,
        hlist,
        grid_list,
        control_changed,
        parent_changed,
        vlist_changed,
        hlist_changed,
        gridlist_changed,
    ) in &query
    {
        let mut style_dictionary = HashMap::<String, String>::new();

        if control_changed.is_some_and(|x| x)
            || parent_changed.is_some_and(|x| x)
            || vlist_changed.is_some_and(|x| x)
            || hlist_changed.is_some_and(|x| x)
            || gridlist_changed.is_some_and(|x| x)
        {
            if vlist.is_some() {
                let vlist = vlist.unwrap();
                style_dictionary.insert("display".to_string(), "flex".to_string());
                style_dictionary.insert("flex-direction".to_string(), "column".to_string());
                match vlist.anchor {
                    Anchor::UpperLeft => {
                        style_dictionary.insert("justify-content".to_string(), "safe start".to_string());
                        style_dictionary.insert("align-content".to_string(), "start".to_string());
                        style_dictionary.insert("align-items".to_string(), "start".to_string());
                    }
                    Anchor::UpperCenter => {
                        style_dictionary.insert("justify-content".to_string(), "safe start".to_string());
                        style_dictionary.insert("align-content".to_string(), "center".to_string());
                        style_dictionary.insert("align-items".to_string(), "center".to_string());
                    }
                    Anchor::MiddleLeft => {
                        style_dictionary
                            .insert("justify-content".to_string(), "safe center".to_string());
                        style_dictionary.insert("align-content".to_string(), "start".to_string());
                        style_dictionary.insert("align-items".to_string(), "start".to_string());
                    }
                    Anchor::MiddleCenter => {
                        style_dictionary
                            .insert("justify-content".to_string(), "safe center".to_string());
                        style_dictionary.insert("align-content".to_string(), "center".to_string());
                        style_dictionary.insert("align-items".to_string(), "center".to_string());
                    }
                    Anchor::LowerCenter => {
                        style_dictionary.insert("justify-content".to_string(), "safe end".to_string());
                        style_dictionary.insert("align-content".to_string(), "center".to_string());
                        style_dictionary.insert("align-items".to_string(), "center".to_string());
                    },
                    Anchor::LowerRight => {
                        style_dictionary
                            .insert("justify-content".to_string(), "safe end".to_string());
                        style_dictionary.insert("align-content".to_string(), "end".to_string());
                        style_dictionary.insert("align-items".to_string(), "end".to_string());
                    }
                    _other => {
                        style_dictionary
                            .insert("justify-content".to_string(), "center".to_string());
                    }
                }

                style_dictionary.insert("gap".to_string(), vlist.spacing.to_string() + "px");
                if vlist.wrapped {
                    style_dictionary.insert("flex-wrap".to_string(), "wrap".to_string());
                }

                if vlist.stretch_children {
                    style_dictionary.insert("align-items".to_string(), "stretch".to_string());
                }
            }

            if hlist.is_some() {
                let hlist = hlist.unwrap();
                style_dictionary.insert("display".to_string(), "flex".to_string());
                style_dictionary.insert("flex-direction".to_string(), "row".to_string());
                match hlist.anchor {
                    Anchor::UpperLeft => {
                        style_dictionary.insert("justify-content".to_string(), "start".to_string());
                        style_dictionary.insert("align-content".to_string(), "start".to_string());
                        style_dictionary.insert("align-items".to_string(), "start".to_string());
                    }
                    Anchor::UpperCenter => {
                        style_dictionary
                            .insert("justify-content".to_string(), "center".to_string());
                        style_dictionary.insert("align-content".to_string(), "start".to_string());
                        style_dictionary.insert("align-items".to_string(), "start".to_string());
                    }
                    Anchor::UpperRight => {
                        style_dictionary.insert("justify-content".to_string(), "end".to_string());
                        style_dictionary.insert("align-content".to_string(), "start".to_string());
                        style_dictionary.insert("align-items".to_string(), "start".to_string());
                    }
                    Anchor::MiddleLeft => {
                        style_dictionary.insert("justify-content".to_string(), "end".to_string());
                        style_dictionary.insert("align-content".to_string(), "start".to_string());
                        style_dictionary.insert("align-items".to_string(), "start".to_string());
                    }
                    Anchor::MiddleRight => {
                        style_dictionary.insert("justify-content".to_string(), "end".to_string());
                        style_dictionary.insert("align-content".to_string(), "center".to_string());
                        style_dictionary.insert("align-items".to_string(), "center".to_string());
                    },
                    Anchor::LowerLeft => {
                        style_dictionary
                            .insert("justify-content".to_string(), "start".to_string());
                        style_dictionary.insert("align-content".to_string(), "end".to_string());
                        style_dictionary.insert("align-items".to_string(), "end".to_string());
                    }
                    Anchor::LowerCenter => {
                        style_dictionary
                            .insert("justify-content".to_string(), "center".to_string());
                        style_dictionary.insert("align-content".to_string(), "end".to_string());
                        style_dictionary.insert("align-items".to_string(), "end".to_string());
                    }
                    _other => {
                        style_dictionary
                            .insert("justify-content".to_string(), "center".to_string());
                        style_dictionary.insert("align-content".to_string(), "center".to_string());
                        style_dictionary.insert("align-items".to_string(), "center".to_string());
                    }
                }

                style_dictionary.insert("gap".to_string(), hlist.spacing.to_string() + "px");
                if hlist.wrapped {
                    style_dictionary.insert("flex-wrap".to_string(), "wrap".to_string());
                }

                if hlist.stretch_children {
                    style_dictionary.insert("align-items".to_string(), "stretch".to_string());
                }
            }

            // Todo: Flesh out or get rid of--vertical and horizontal lists can already function as grids when wrapping is enabled
            if grid_list.is_some() {
                let _grid_list = grid_list.unwrap();
                //display: grid
                //grid-gap: 0px
                //grid-template-columns: repeat(auto-fit, 50%)
                style_dictionary.insert("background".to_string(), "none".to_string());
            }

            if !control.IsVisible {
                style_dictionary.insert("display".to_string(), "none".to_string());
            }

            let element = add_or_get_element(entity, None);
            insert_style(&element, style_dictionary);
        }
    }
}

pub fn base_change_detection(
    _commands: Commands,
    query: Query<(
        Entity,
        &Control,
        Option<&Parent>,
        Option<&VScroll>,
        Option<&BackgroundColor>,
        Option<&ImageRect>,
        Option<&Label>,
        Option<&InputField>,
        Option<&Shadow>,
        Option<&Button>,
        Option<Changed<Control>>,
        Option<Changed<Parent>>,
        Option<Changed<ImageRect>>,
        Option<Changed<Label>>,
        Option<Changed<InputField>>,
    )>,
    parent_container_query: Query<(&Container, Option<&VList>, Option<&HList>)>,
) {
    for (
        entity,
        control,
        parent,
        vscroll,
        background_color,
        image_rect,
        label,
        input_field,
        shadow,
        button,
        changed_control,
        changed_parent,
        changed_image_rect,
        changed_label,
        changed_input_field,
    ) in &query
    {
        //if changed_control.is_some_and(|x| x) {
        //    let id = entity.to_bits().to_string();
        //    console::log!(format!("Control with ID {id} changed!"));
        //}

        if changed_control.is_some_and(|x| x)
            || changed_parent.is_some_and(|x| x)
            || changed_image_rect.is_some_and(|x| x)
            || changed_label.is_some_and(|x| x)
            || changed_input_field.is_some_and(|x| x)
        {
            // Used for debugging
            /* 
            let id = entity.to_bits().to_string();
            let was_control_changed = changed_control.is_some_and(|x| x);
            let was_parent_changed = changed_parent.is_some_and(|x| x);
            let was_label_changed = changed_label.is_some_and(|x| x);
            let was_input_changed = changed_input_field.is_some_and(|x| x);

            console::log!(format!("CHANGED ENTITY: {id}.\nCONTROL CHANGED: {was_control_changed}.\nPARENT CHANGED: {was_parent_changed}.\nLABEL CHANGED: {was_label_changed}.\nINPUT CHANGED: {was_input_changed}"));
            */

            let mut is_number = false;
            let mut element_type = "div".to_string();
            let mut attribute_dictionary = HashMap::<String, String>::new();
            let mut style_dictionary = HashMap::<String, String>::new();
            let mut text_content = "".to_string();
            let mut use_pointer = false;

            let mut is_parent_container = false;

            if parent.is_some() {
                let parent = parent.unwrap();

                let parent_container = parent_container_query.get(parent.get());
                if parent_container.is_ok() {
                    let (_container, _vlist, _hlist) = parent_container.unwrap();
                    is_parent_container = true;
                }
            }

            style_dictionary.insert("display".to_string(), "grid".to_string());
            style_dictionary.insert("background".to_string(), "none".to_string());
            if control.IsOverflow {
                style_dictionary.insert("overflow".to_string(), "unset".to_string());
            } else {
                style_dictionary.insert("overflow".to_string(), "auto".to_string());
            }

            if vscroll.is_some() {
                style_dictionary.insert("overflow-y".to_string(), "auto".to_string());
                use_pointer = true;
            }

            if let Some(input_field) = input_field {
                element_type = "input".to_string();
                style_dictionary.insert("background".to_string(), "none".to_string());
                style_dictionary.insert("font-size".to_string(), input_field.font_size.to_string() + "px");

                attribute_dictionary.insert("autocomplete".to_string(), "off".to_string());
                attribute_dictionary
                    .insert("placeholder".to_string(), input_field.placeholder.clone());

                text_content = input_field.text.clone();
                use_pointer = true;

                match input_field.input_type {
                    InputType::Default => {

                    }
                    InputType::Password => {
                        attribute_dictionary.insert("type".to_string(), "password".to_string());
                    },
                    InputType::PhoneNumber => {
                        is_number = true;
                        //attribute_dictionary.insert("type".to_string(), "number".to_string());
                    }
                }
            }

            if button.is_some() {
                let _image_button = button.unwrap();
                element_type = "button".to_string();

                style_dictionary.insert("background".to_string(), "none".to_string());
                style_dictionary.insert("cursor".to_string(), "pointer".to_string());
                use_pointer = true;
            }

            if background_color.is_some() {
                let background_color = background_color.unwrap();
                let r = (background_color.0.r() * 256.0) as u8;//.to_string();
                let g = (background_color.0.g() * 256.0) as u8;//.to_string();
                let b = (background_color.0.b() * 256.0) as u8;//.to_string();
                let a = (background_color.0.a() * 256.0) as u8;//.to_string();
                let hex_color = format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a);
                style_dictionary.insert(
                    "background".to_string(),
                    hex_color
                   // "rgba(".to_string() + r + ", " + g + ", " + b + ", " + a + ")",
                );
            }

            if control.ExpandWidth {
                style_dictionary.insert("width".to_string(), "100%".to_string());
                style_dictionary.insert("left".to_string(), "0".to_string());
                style_dictionary.insert("right".to_string(), "0".to_string());
            } else {
                style_dictionary.insert("width".to_string(), "auto".to_string());
            }

            if control.ExpandHeight {
                style_dictionary.insert("height".to_string(), "100%".to_string());
                style_dictionary.insert("top".to_string(), "0".to_string());
                style_dictionary.insert("bottom".to_string(), "0".to_string());
            } else {
                style_dictionary.insert("height".to_string(), "auto".to_string());
            }

            let scale = control.Scale;
            style_dictionary.insert("scale".to_string(), scale.to_string());

            let left = control.BorderRadius.x;
            let top = control.BorderRadius.y;
            let right = control.BorderRadius.z;
            let bottom = control.BorderRadius.w;

            style_dictionary.insert(
                "border-radius".to_string(),
                format!("{top}px {right}px {bottom}px {left}px"),
            );

            if control.BorderRadius != Vec4::ZERO {
                style_dictionary.insert("overflow".to_string(), "revert".to_string());
            }

            let border_width = control.BorderWidth;
            style_dictionary.insert("border".to_string(), format!("{border_width}px solid #000"));
            style_dictionary.insert("outline".to_string(), "none".to_string());

            if image_rect.is_some() {
                let image_rect = image_rect.unwrap();
                //element_type = "img".to_string();

                //style_dictionary.insert("background".to_string(), "none".to_string());
                let brightness = &image_rect.brightness.to_string();
                style_dictionary.insert(
                    "filter".to_string(),
                    "brightness(".to_string() + brightness + ")",
                );
                
                if image_rect.multiply {
                    style_dictionary.insert("mix-blend-mode".to_string(), "multiply".to_string());
                }
                if image_rect.is_nine_slice {
                    element_type = "div".to_string();

                    let x = image_rect.border_image_slice.x;
                    let y = image_rect.border_image_slice.y;
                    let z = image_rect.border_image_slice.z;
                    let w = image_rect.border_image_slice.w;
                    style_dictionary.insert("border-image-slice".to_string(), format!("{x} {y} {z} {w} fill").to_string());
                    let x = image_rect.border_image_width.x;
                    let y = image_rect.border_image_width.y;
                    let z = image_rect.border_image_width.z;
                    let w = image_rect.border_image_width.w;
                    style_dictionary.insert("border-image-width".to_string(), format!("{x}px {y}px {z}px {w}px").to_string());
                    style_dictionary.insert("border-image-outset".to_string(), "0px 0px 0px 0px".to_string());
                    let image = image_rect.image.clone();
                    style_dictionary.insert("border-image-source".to_string(), format!("url({image})"));
                    style_dictionary.insert("border-image-repeat".to_string(), "stretch stretch".to_string());
                    style_dictionary.insert("border-style".to_string(), "solid".to_string());
                    style_dictionary.remove("border");
                } else {
                    element_type = "div".to_string();
                    //attribute_dictionary.insert("src".to_string(), image_rect.image.clone());
                    let mut image = image_rect.image.clone();
                    style_dictionary.remove("background");

                    if image_rect.data.len() > 0 {// image.is_empty() {
                        let b64 = general_purpose::STANDARD.encode(image_rect.data.clone());
                        image = format!("data:image/jpeg;base64,@Convert.ToBase64String(electedOfficial.Picture)");
                    }
                    style_dictionary.insert("background-image".to_string(), format!("url('{image}')"));
                    style_dictionary.insert("background-repeat".to_string(), "no-repeat".to_string());
                    style_dictionary.insert("background-size".to_string(), "contain".to_string());
                    style_dictionary.insert("background-position".to_string(), "center".to_string());
                }
                if let Some(aspect_ratio) = image_rect.aspect_ratio {
                    style_dictionary.insert("min-width".to_string(), "100%".to_string());
                    style_dictionary.insert("min-height".to_string(), "100%".to_string());
                    style_dictionary.insert("aspect-ratio".to_string(), aspect_ratio.to_string());
                }
                if image_rect.cover_background {
                    style_dictionary.insert("min-width".to_string(), "100%".to_string());
                    style_dictionary.insert("min-height".to_string(), "100%".to_string());
                    style_dictionary.insert("background-size".to_string(), "cover".to_string());
                }
            }

            if is_parent_container {
                style_dictionary.insert("position".to_string(), "relative".to_string());
            } else {
                style_dictionary.insert("position".to_string(), "absolute".to_string());

                if parent.is_some() {
                    let pivot = control.Pivot.vector_from_anchor();
                    let pivot_x = &(-pivot.x * 100.0).to_string();
                    let pivot_y = &(-pivot.y * 100.0).to_string();
                    style_dictionary.insert(
                        "transform".to_string(),
                        "translate(".to_string() + pivot_x + "%, " + pivot_y + "%)",
                    );

                    let top = &(pivot.x * 100.0).to_string();
                    let left = &(pivot.y * 100.0).to_string();
                    style_dictionary.insert("left".to_string(), top.to_string() + "%");
                    style_dictionary.insert("top".to_string(), left.to_string() + "%");
                } else {
                    // Use anchors if there isn't a parent container controlling layout
                    let left_margin = &0.to_string();
                    let left_pos = &control.LocalPosition.x.to_string();
                    let left_pivot = &(control.fixed_width / 2.0).to_string();
                    style_dictionary.insert(
                        "left".to_string(),
                        "calc(".to_string()
                            + left_margin
                            + "% + "
                            + left_pos
                            + "px)"
                            //+ "px - "
                            //+ left_pivot
                            //+ "px)",
                    );

                    let top_margin = &0.to_string();
                    let top_pos = &control.LocalPosition.y.to_string();
                    let top_pivot = &(control.fixed_height / 2.0).to_string();
                    style_dictionary.insert(
                        "top".to_string(),
                        "calc(".to_string()
                            + top_margin
                            + "% + "
                            + top_pos
                            + "px)"
                            //+ "px - "
                            //+ top_pivot
                            //+ "px)",
                    );
                }
            }

            if (!style_dictionary.contains_key("min-height")) {
                style_dictionary.insert("min-height".to_string(), "auto".to_string());
            }
            if (!style_dictionary.contains_key("min-width")) {
                style_dictionary.insert("min-width".to_string(), "auto".to_string());
            }

            style_dictionary.insert("box-sizing".to_string(), "border-box".to_string());
            style_dictionary.insert("word-break".to_string(), "break-word".to_string());

            if control.FitWidth {
                style_dictionary.insert("width".to_string(), "fit-content".to_string());
            } else if control.fixed_width > -1.0 {
                style_dictionary.insert("width".to_string(), control.fixed_width.to_string() + "px");
                style_dictionary.insert("min-width".to_string(), control.fixed_width.to_string() + "px");
            }

            if control.FitHeight {
                style_dictionary.insert("height".to_string(), "fit-content".to_string());
            } else if control.fixed_height > -1.0 {
                style_dictionary.insert("height".to_string(), control.fixed_height.to_string() + "px");
                style_dictionary
                    .insert("min-height".to_string(), control.fixed_height.to_string() + "px");
            }

            let left = control.Padding.x;
            let top = control.Padding.y;
            let right = control.Padding.z;
            let bottom = control.Padding.w;

            style_dictionary.insert(
                "padding".to_string(),
                format!("{top}px {right}px {bottom}px {left}px"),
            );

            if let Some(label) = label {
                element_type = "p".to_string();
                text_content = label.text.to_string();
                //style_dictionary.insert("overflow".to_string(), "unset".to_string());
                style_dictionary.insert("font-family".to_string(), label.font.clone());

                style_dictionary.insert("font-size".to_string(), label.font_size.to_string() + "px");
                style_dictionary.insert("color".to_string(), get_css_string(label.color));
                
                if label.is_shadow {
                    style_dictionary.insert("text-shadow".to_string(), "2px 2px 15px rgba(0,0,0,.4)".to_string());
                }                

                let alignment: String;
                match label.alignment {
                    Anchor::UpperLeft | Anchor::MiddleLeft | Anchor::LowerLeft => {
                        alignment = "left".to_string()
                    }
                    Anchor::UpperCenter | Anchor::MiddleCenter | Anchor::LowerCenter => {
                        alignment = "center".to_string()
                    }
                    Anchor::UpperRight | Anchor::MiddleRight | Anchor::LowerRight => {
                        alignment = "right".to_string()
                    }
                }

                style_dictionary.insert("display".to_string(), "block".to_string());
                style_dictionary.insert("text-align".to_string(), alignment);

                use_pointer = true;

                // Margin override for evil fonts
                if label.font == "Mogra".to_string() {
                    let offset = 0.0;//label.FontSize / 6.0;
                    style_dictionary.insert("margin".to_string(), format!("0px 0px -{offset}px 0px").to_string());
                } else {
                    style_dictionary.insert("margin".to_string(), 0.to_string());
                }

                if label.is_single_line && !control.ExpandWidth {
                    style_dictionary.insert("box-sizing".to_string(), "content-box".to_string());
                    style_dictionary.insert("word-break".to_string(), "normal".to_string());
                    style_dictionary.insert("width".to_string(), "max-content".to_string());
                    style_dictionary.insert("flex-shrink".to_string(), "0".to_string());
                }

                if label.is_italic {
                    style_dictionary.insert("font-style".to_string(), "italic".to_string());
                }
                if label.is_bold {
                    style_dictionary.insert("font-weight".to_string(), "bold".to_string());
                } else {
                    let font_weight = label.font_weight;
                    style_dictionary.insert("font-weight".to_string(), format!("{font_weight}").to_string());
                }

                if label.is_3d {
                    let color = "#aeaeae";

                    let mut depth_string = "".to_string();
                    let depth = (label.font_size / 5.0) as i32;
                    for i in 0..depth {
                        let depth_val = i + 1;
                        depth_string += &format!("0px {depth_val}px 0px {color}, ");
                    }

                    let mut stroke_string = "".to_string();
                    //let depth = (label.font_size / 5.0) as i32;
                    let max_depth = depth+10;
                    //stroke_string += &format!("10px {max_depth}px 0 #000, -10px {max_depth}px 0 #000, -10px -10px 0 #000, 10px -10px 0 #000");
                    
                    //for i in 0..3 {
                    //    let depth_val = i + 1;
                    //    depth_string += &format!("1px {depth_val}px 1px {color}, ");
                    //}

                    style_dictionary.insert("text-shadow".to_string(), format!(r#"1px 1px 10px {color}, {depth_string} 1px 18px 6px rgba(16,16,16,0.4), 1px 22px 10px rgba(16,16,16,0.2), 1px 25px 35px rgba(16,16,16,0.2), 1px 30px 60px rgba(16,16,16,0.75)"#).to_string());

                    style_dictionary.insert("transform".to_string(), "perspective(1000px) rotateX(25deg)".to_string());
                }

                if let Some(line_height) = label.line_height {
                    style_dictionary.insert("line-height".to_string(), format!("{line_height}px").to_string());
                }
            }

            if shadow.is_some() {
                let _shadow = shadow.unwrap();
                style_dictionary.insert(
                    "border-color".to_string(),
                    "rgba(223,225,229,0)".to_string(),
                );
                style_dictionary.insert(
                    "-webkit-appearance".to_string(),
                    "none".to_string(),
                );
                style_dictionary.insert(
                    "-webkit-appearance".to_string(),
                    "none".to_string(),
                );
                style_dictionary.insert(
                    "-webkit-box-shadow".to_string(),
                    "0 1px 6px rgb(32 33 36 / 28%) !important".to_string(),
                );
                style_dictionary.insert(
                    "box-shadow".to_string(),
                    "0 1px 6px rgb(32 33 36 / 28%) !important".to_string(),
                );
                
                // TODO: Add back in as an optional attribute
                //style_dictionary.insert(
                //    "filter".to_string(),
                //    "drop-shadow(0px 1px 6px #444)".to_string(),
                //);
            }

            if !control.IsVisible {
                style_dictionary.insert("display".to_string(), "none".to_string());
            }

            if use_pointer {
                style_dictionary.insert("pointer-events".to_string(), "all".to_string());
            } else {
                style_dictionary.insert("pointer-events".to_string(), "none".to_string());
            }

            if control.UseBackground {
                style_dictionary.insert("background".to_string(), "white".to_string());
            }

            if control.stretch {
                style_dictionary.insert("align-self".to_string(), "stretch".to_string());
            }

            style_dictionary.insert("white-space".to_string(), "pre-wrap".to_string());

            let element = add_or_get_element(entity, Some(element_type));

            if is_number {
                crate::prelude::main_js::phoneNumber(element.clone());
            }

            for (key, val) in attribute_dictionary {
                element.set_attribute(&key, &val);
            }

            insert_style(&element, style_dictionary);

            if label.is_some() {
                element.set_inner_html(&text_content);
            }

            //if let Ok(input_element) = element.clone().dyn_into::<HtmlInputElement>() {
                //input_element.set_value(&text_content);
            //}

            // Add hook to button click event
            if input_field.is_some() {
                let _input_field = input_field.unwrap();

                let e = element.clone();
                let f = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
                    if ev.key() == "Enter" {
                        //console::info!("Enter key pressed!");
                        let _  = e.set_attribute("was_submitted", &true.to_string());
                    }
                }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

                let _  = element.add_event_listener_with_callback("keypress", f.as_ref().unchecked_ref());
                f.forget();

                let e = element.clone();
                let f = Closure::wrap(Box::new(move |_ev: web_sys::Event| {
                    let _  = e.set_attribute("was_input", &true.to_string());
                }) as Box<dyn FnMut(web_sys::Event)>);

                let _  = element.add_event_listener_with_callback("input", f.as_ref().unchecked_ref());
                f.forget();
            }
        }
    }
}

//pub fn get_page_origin() -> String {
//    return web::get_page_origin().unwrap()
//}

pub fn update_heirarchy(mut ev_hierarchy: EventReader<HierarchyEvent>,
    _query: Query<(Entity, &Control)>,
    parents_query: Query<(Entity, &Children), Changed<Children>>,
) {
    for ev in ev_hierarchy.iter() {
        match ev {
            HierarchyEvent::ChildAdded { child, parent } => {
                let child_element = add_or_get_element(*child, None);

                let _child_id = child.to_bits().to_string();
                let _parent_id = parent.to_bits().to_string();
                //console::info!(format!("Parent of {_child_id} changed to {_parent_id}!"));
    
                //if let Some(child) = result {
                //let child = result.unwrap();
                let parent_element = add_or_get_element(*parent, None);
                let _  = parent_element.append_child(&child_element);
            }
            HierarchyEvent::ChildRemoved { child, parent } => {

            }
            HierarchyEvent::ChildMoved { child, previous_parent, new_parent } => {

            }
        }
    }
    /*
    for (parent_entity, children) in &parents_query {
        for child in children.iter() {
            let result = get_element(*child);

            let _child_id = child.to_bits().to_string();
            let _parent_id = parent_entity.to_bits().to_string();
            console::info!(format!("Parent of {_child_id} changed to {_parent_id}!"));

            //if let Some(child) = result {
            let child = result.unwrap();
            let parent_element = get_element(parent_entity).unwrap();
            parent_element.append_child(&child);
            //}
        }
    }
     */
}

pub fn is_extension() -> bool {
    let url = web_sys::window()
    .unwrap()
    .document()
    .unwrap()
    .url()
    .expect("Could not get window URL!");

    url.starts_with("chrome-extension://")
}

pub fn go_to_url(url: String) -> Result<(), JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No global `window` exists"))?;
    let location = window.location();
    location.assign(&url).map_err(|_| JsValue::from_str("Unable to navigate to the specified URL"))
}

pub async fn import_font(font_name: String, font_path: String) {
    let _link = get_document().create_element("link").unwrap();
    let fontface =
        web_sys::FontFace::new_with_str(&font_name, &format!("url({})", font_path)).unwrap();
    let promise = fontface.load().unwrap();
    let _result = wasm_bindgen_futures::JsFuture::from(promise).await;

    get_document().fonts().add(&fontface).unwrap();
}

pub fn write_event(key: &str, value: String) {
    let _  = get_root_element().unwrap().set_attribute(key, &value);
}

pub fn read_event(key: &str) -> Option<String> {
    get_root_element().unwrap().get_attribute(key)
}

pub fn clear_event(key: &str) {
    let _  = get_root_element().unwrap().remove_attribute(key);
}

pub fn get_root_element() -> Option<Element> {
    get_document().get_element_by_id("main")
}

pub fn get_node(entity: Entity) -> Option<web_sys::Node> {
    let element = get_element(entity);
    element.as_ref()?;
    Some(element.unwrap().dyn_into::<web_sys::Node>().unwrap())
}

pub fn insert_style(element: &Element, mut target_style_dictionary: HashMap<String, String>) {
    let source_style_dictionary = element.get_attribute("style");
    if let Some(source_style_dictionary) = source_style_dictionary {
        let key_value_pairs = source_style_dictionary.split("; ");
        for key_value_pair in key_value_pairs {
            if !key_value_pair.trim_start().trim_end().is_empty() {
                let key_and_value: Vec<&str> = key_value_pair.split(':').collect();
                if key_and_value.len() > 1 {
                    let key = key_and_value[0].trim_start().trim_end();
                    let value = key_and_value[1].trim_start().trim_end();

                    if !target_style_dictionary.contains_key(key) {
                        target_style_dictionary.insert(key.to_string(), value.to_string());
                    }
                } else {
                    panic!("Invalid CSS style: {}", key_value_pair);
                }
            }
        }
    }

    let mut styleString: String = "".to_string();
    for (key, val) in target_style_dictionary {
        let res = key.clone() + ": " + &val + "; ";
        styleString += &res;
    }

    element.set_attribute("style", &styleString);
}

pub fn create_element(element_type: Option<String>) -> Element {
    let document = get_document();
    let _body = document.body().expect("document should have a body");

    let _element_type: String;
    if element_type.is_none() {
        _element_type = "div".to_string();
    } else {
        _element_type = element_type.unwrap();
    }

    let element = document.create_element(&_element_type).unwrap();

    let root_element = get_root_element().expect("Failed to get child of body!");
    root_element.append_child(&element).unwrap();
    element
}

pub fn add_or_get_element(entity: Entity, element_type: Option<String>) -> Element {
    let document = get_document();
    let _body = document.body().expect("document should have a body");

    let element = get_element(entity);

    if element.is_none() {
        //if (!control_tracker.is_added()) {
        //    console::info!(format!("CHANGED PROBLEM!"));
        //}

        let element = create_element(element_type);

        let silk_id = entity.to_bits().to_string();
        let _  = element.set_attribute("silk-id", &silk_id.clone());

        let e = element.clone();
        let f = Closure::wrap(Box::new(move |ev: js_sys::Array| {
            let entry = web_sys::ResizeObserverEntry::from(ev.get(0));
            let width = entry.content_rect().width();
            let height = entry.content_rect().height();
            //console::log!(width);
            let _  = e.set_attribute("width_change", &width.to_string());
            let _  = e.set_attribute("height_change", &height.to_string());
        }) as Box<dyn FnMut(js_sys::Array)>);
        let observer = web_sys::ResizeObserver::new(f.as_ref().unchecked_ref()).unwrap();
        f.forget();
        let e = element.clone();
        observer.observe(&e);

        let e = element.clone();
        let f = Closure::wrap(Box::new(move || {
            let _  = e.set_attribute("was_clicked", &true.to_string());
        }) as Box<dyn FnMut()>);
        let _  = element.add_event_listener_with_callback("click", f.as_ref().unchecked_ref());
        f.forget();

        let e = element.clone();
        let f = Closure::wrap(Box::new(move || {
            let _  = e.set_attribute("was_focused", &true.to_string());
        }) as Box<dyn FnMut()>);
        let _  = element.add_event_listener_with_callback("onfocusin", f.as_ref().unchecked_ref());
        f.forget();

        let e = element.clone();
        let f = Closure::wrap(Box::new(move || {
            let _  = e.set_attribute("was_unfocused", &true.to_string());
        }) as Box<dyn FnMut()>);
        let _  = element.add_event_listener_with_callback("onfocusout", f.as_ref().unchecked_ref());
        f.forget();


        let e = element.clone();
        let s_id = silk_id.clone();
        let f = Closure::wrap(Box::new(move || {
            let e = e.clone();
            let _  = e.set_attribute("was_mouse_over", &false.to_string());
            if let Ok(e) = e.dyn_into::<HtmlDivElement>() {
                let width = e.offset_width();
                let height = e.offset_height();
                //console::log!(format!("{s_id} MOUSE LEAVE: {width}, {height}"))
            }
        }) as Box<dyn FnMut()>);
        let _  = element.add_event_listener_with_callback("mouseleave", f.as_ref().unchecked_ref());
        f.forget();

        let e = element.clone();
        let s_id = silk_id.clone();
        let f = Closure::wrap(Box::new(move || {
            let e = e.clone();
            let _  = e.set_attribute("was_mouse_over", &true.to_string());
            if let Ok(e) = e.dyn_into::<HtmlDivElement>() {
                let width = e.offset_width();
                let height = e.offset_height();
                //console::log!(format!("{s_id} MOUSE ENTER: {width}, {height}"))
            }
        }) as Box<dyn FnMut()>);
        let _  = element.add_event_listener_with_callback("mouseenter", f.as_ref().unchecked_ref());
        f.forget();
 
        element
    } else {
        element.unwrap()
    }
}

pub fn get_element(entity: Entity) -> Option<Element> {
    let id = entity.to_bits().to_string();
    get_element_from_str(id)
}

pub fn get_element_from_str(id: String) -> Option<Element> {
    get_document().query_selector(&format!(r#"[silk-id="{id}"]"#)).unwrap()
}

pub fn get_document() -> web_sys::Document {
    let window = web_sys::window().expect("no global `window` exists");
    window.document().expect("should have a document on window")
}

pub fn get_css_string(color: Color) -> String {
    let r = color.r() * 255.0;
    let g = color.g() * 255.0;
    let b = color.b() * 255.0;
    format!("rgb({r}, {g}, {b})")
}

pub fn remove_detection(mut removals: RemovedComponents<Control>) {
    for entity in removals.iter() {
        //console::info!("DELETED {}", entity.to_bits().to_string());
        if let Some(element) = get_element(entity) {
            element.remove();
        }
    }
}

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

pub fn on_show_detection(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Control,
        &mut OnShow),
        Changed<Control>>
) {
    for (entity, mut control, mut on_show) in query.iter_mut() {
        if control.IsVisible && !on_show.was_visible || !control.IsVisible && on_show.was_visible {
            on_show.was_visible = control.IsVisible;
            if control.IsVisible {
                //let c: &mut Commands<'_, '_> = &mut commands;
                if let Some(func) = on_show.func.as_ref() {
                    func.call(&mut commands);
                }
            }
        }
    }
}

pub fn event_detection(
    mut commands: Commands,
    mut ev_click: EventWriter<ClickEvent>,
    mut ev_submit: EventWriter<SubmitEvent>,
    mut ev_snap_scroll_y: EventReader<SnapScrollY>,
    mut query: Query<(
        Entity,
        &mut Control,
        //Option<Changed<Control>>,
        //Option<&mut OnShow>,
        Option<&mut InteractState>,
        Option<&mut OnClick>,
        Option<&mut elements::Button>,
        Option<&mut InputField>
    )>
) {
    // Manufacture the element we're gonna append
    let document = get_document();
    let _body = document.body().expect("document should have a body");

    for ev in ev_snap_scroll_y.iter() {
        let element: Element = add_or_get_element(ev.0, None);
        element.set_scroll_top(element.scroll_height());
        //console::log!("SNAPPING SCROLL Y");
    }

    for (entity, mut control, mut interact_state, on_click, button, input_field) in query.iter_mut() {
        let element = get_element(entity);
        if element.is_some() {
            let element = element.unwrap();

            //control.ScrollTop = element.scroll_top() as f32;

            if let Some(mut interact_state) = interact_state {
                let was_hover = element.get_attribute("was_mouse_over");
                if let Some(was_hover) = was_hover {
                    let was_hover: bool = was_hover.parse().unwrap();
                    let _ = element.remove_attribute("was_mouse_over");
                    //console::log!("MOUSE OVER");
                    interact_state.is_hovering = was_hover;
                    commands.entity(entity).insert(BindableChanged {});
                }

                let was_focused = element.get_attribute("was_focused");
                if was_focused.is_some() {
                    //console::info!(format!("{} focused.", entity.to_bits().to_string()));

                    let _ = element.remove_attribute("was_focused");
                    interact_state.is_focused = false;
                    commands.entity(entity).insert(BindableChanged {});
                }

                let _was_unfocused = element.get_attribute("was_unfocused");
                if was_focused.is_some() {
                    //console::info!(format!("{} unfocused.", entity.to_bits().to_string()));

                    let _ = element.remove_attribute("was_unfocused");
                    interact_state.is_focused = false;
                    commands.entity(entity).insert(BindableChanged {});
                }
/* 
                let was_clicked = element.get_attribute("was_clicked");
                if was_clicked.is_some() {
                    console::info!(format!("{} clicked.", entity.to_bits().to_string()));

                    element.remove_attribute("was_clicked");
                    interact_state.is_focused = false;
                    commands.entity(entity).insert(BindableChanged {});
                }
                */
            }

            if let Some(width) = element.get_attribute("width_change") {
                control.width = width.parse().unwrap();
                control.height = element.get_attribute("height_change").unwrap().parse().unwrap();
                let _ = element.remove_attribute("width_change");
                let _ = element.remove_attribute("height_change");
            }
            // May return as None if Control was just added
            // TODO: Possibly handle additions using Changed here rather than a separate system
            if let Some(mut input_field) = input_field {
                let was_input = element.get_attribute("was_input");
                if was_input.is_some() {
                    let _ = element.remove_attribute("was_input");

                    let input_element: HtmlInputElement = element.clone().dyn_into().unwrap();
                    input_field.text = input_element.value();
                    log(input_field.text.clone());
                }

                let was_submitted = element.get_attribute("was_submitted");
                if was_submitted.is_some() {
                    let _ = element.remove_attribute("was_submitted");
                    //console::info!(format!("{} submitted.", entity.to_bits().to_string()));
                    ev_submit.send(SubmitEvent(entity));
                    if let Some(on_submitted) = input_field.on_submitted.as_ref() {
                        on_submitted.call(&mut commands);
                    }
                }
            }

            if let Some(on_click) = on_click.as_ref() {
                let was_clicked = element.get_attribute("was_clicked");
                if was_clicked.is_some() {
                    let _ = element.remove_attribute("was_clicked");
                    on_click.func.call(&mut commands);
                    ev_click.send(ClickEvent(entity));
                }
            }
            else if let Some(button) = button.as_ref() {
                let was_clicked = element.get_attribute("was_clicked");
                if was_clicked.is_some() {
                    //console::info!(format!("{} clicked.", entity.to_bits().to_string()));

                    let _ = element.remove_attribute("was_clicked");
                    if let Some(on_click) = button.on_click.as_ref() {
                        on_click.call(&mut commands);
                    }
                    ev_click.send(ClickEvent(entity));
                }
            }
            /*
            console::info!("{}'s parent changed: {}.", entity.to_bits().to_string(), parent.to_bits().to_string());
            let document = get_document();
            let element = document.get_element_by_id(&entity.to_bits().to_string()).expect("Failed to get element from entity ID!");
            let parent_element = document.get_element_by_id(&parent.to_bits().to_string()).expect("Failed to get parent from entity ID!");
            parent_element.append_child(&element);
            */
        }
    }
}
