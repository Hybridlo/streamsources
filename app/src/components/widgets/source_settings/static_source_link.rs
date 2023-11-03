use std::{rc::Rc, cell::RefCell};

use gloo_timers::callback::Timeout;
use serde::Serialize;
use web_sys::Element;
use yew::prelude::*;
use yew_hooks::use_clipboard;

#[derive(Properties, PartialEq)]
pub struct SourceLinkProps<T: Serialize + PartialEq> {
    pub options: T,
    pub source_name: &'static str,
    pub skin: String,
}

#[function_component(StaticSourceLink)]
pub fn static_source_link<T: Serialize + PartialEq>(props: &SourceLinkProps<T>) -> Html {
    let window = web_sys::window().expect("Window object to exist");
    let location = window.location();
    let href = location.protocol().expect("To have a protocol") + "//" + &location.host().expect("To have a host");

    let clipboard_handle = use_clipboard();

    let options_encoded = serde_urlencoded::ser::to_string(&props.options).expect("Source link options to be serializable");

    let copy_timer: Rc<RefCell<Option<Timeout>>> = use_mut_ref(|| None);
    let copy_button_text = use_state(|| "Copy");
    let copy_button_width = use_state_eq(|| 0);
    let button_ref = use_node_ref();

    let update_copy_button_data = {
        let button_ref = button_ref.clone();
        let copy_button_text = copy_button_text.clone();
        let copy_button_width = copy_button_width.clone();

        move |new_text| {
            let button_ref = button_ref.clone();
            let copy_button_text = copy_button_text.clone();
            let copy_button_width = copy_button_width.clone();
            copy_button_text.set(new_text);
            Timeout::new(0, move || {
                copy_button_width.set(button_ref.cast::<Element>().unwrap().client_width());
            }).forget();
        }
    };

    {
        let update_copy_button_data = update_copy_button_data.clone();
        use_effect_with_deps(
            move |_| {
                update_copy_button_data("Copy");
                || ()
            },
            button_ref.clone()
        );
    }

    let copy_click_callback = {
        let href = href.clone();
        let clipboard_handle = clipboard_handle.clone();
        let copy_timer = copy_timer.clone();
        let options_encoded = options_encoded.clone();

        let source_name = props.source_name;
        let skin = props.skin.clone();

        Callback::from(move |_| {
            let href = href.clone();
            
            clipboard_handle.write_text(
                    href + "/sources"
                    + "/" + source_name
                    + "/" + &skin
                    + "?" + &options_encoded
            );
            
            update_copy_button_data("Copied!");

            if let Some(copy_timer) = (copy_timer).take() {
                copy_timer.cancel();
            }
            let timeout = {
                let update_copy_button_data = update_copy_button_data.clone();
                Timeout::new(2_000, move || {
                    update_copy_button_data("Copy");
                })
            };
            copy_timer.replace(Some(timeout));
        })
    };

    html! {
        <div class="container input-group mb-3">
            <input
                type="text"
                class="form-control text-center"
                readonly=true
                aria-label="Source link"
                aria-describedby="button-addon2"
                style={"padding-right: 0; padding-left: ".to_string() + &format!("{}", *copy_button_width) + "px;"}
                placeholder={
                    href.clone() + "/sources"
                    + "/" + &props.source_name
                    + "/" + &props.skin
                    + "?" + &options_encoded
                }
            />
            <button class="btn btn-outline-secondary" type="button" id="srcCopy" ref={button_ref}
                onclick={copy_click_callback}
            >{ *copy_button_text }</button>
        </div>
    }
}