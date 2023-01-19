use std::{rc::Rc, cell::RefCell};

use gloo_timers::callback::Timeout;
use twitch_sources_client::apis::default_api::api_generate_login_token_get;
use twitch_sources_rework::front_common::IntoWithLogin;
use yew::prelude::*;
use yew_hooks::{use_clipboard, use_async};
use yewdux::prelude::use_store;

use crate::state::{ClientConfig, ErrorState};

#[derive(Properties, PartialEq)]
pub struct SourceLinkProps<T: IntoWithLogin + PartialEq + Clone> {
    pub options: T,
    pub source_name: &'static str,
    pub skin: String,
}

#[function_component(LoginSourceLink)]
pub fn login_source_link<T: IntoWithLogin + PartialEq + Clone + 'static>(props: &SourceLinkProps<T>) -> Html {
    let window = web_sys::window().expect("Window object to exist");
    let location = window.location();
    let href = location.protocol().expect("To have a protocol") + "//" + &location.host().expect("To have a host");

    let clipboard_handle = use_clipboard();

    let copy_timer: Rc<RefCell<Option<Timeout>>> = use_mut_ref(|| None);
    let copy_button_text = use_state(|| "Copy");

    let (client_config, _) = use_store::<ClientConfig>();
    let (_, error_state_setter) = use_store::<ErrorState>();

    let async_state = {
        let href = href.clone();
        let copy_timer = copy_timer.clone();
        let copy_button_text = copy_button_text.clone();
        let error_state_setter = error_state_setter.clone();

        let source_name = props.source_name;
        let skin = props.skin.clone();
        let options = props.options.clone();

        use_async::<_, (), ()>(async move {
            match api_generate_login_token_get(&client_config.config).await {
                Ok(res) => { 
                    let options_encoded = serde_urlencoded::ser::to_string(
                        &options.with_login(&res.token)
                    ).expect("Predictions state options to be serializable");

                    clipboard_handle.write_text(
                        href + "/sources"
                        + "/" + source_name
                        + "/" + &skin
                        + "?" + &options_encoded
                    );
                    copy_button_text.set("Copied!");
                },
                Err(err) => {
                    copy_button_text.set("Error!");
                    error_state_setter.reduce(|_| ErrorState { show_error: true, error_message: err.to_string() });
                },
            }

            if let Some(copy_timer) = (copy_timer).take() {
                copy_timer.cancel();
            }
            let timeout = {
                let copy_button_text = copy_button_text.clone();
                Timeout::new(2_000, move || {
                    copy_button_text.set("Copy");
                })
            };
            copy_timer.replace(Some(timeout));

            Ok(())
        })
    };

    let login_copy_click_callback = {
        let copy_button_text = copy_button_text.clone();
        let async_state = async_state.clone();

        Callback::from(move |_| {
            copy_button_text.set("Loading...");
            async_state.run()
        })
    };

    html! {
        <div class="container input-group mb-3">
            <input id="srcLink" type="text" class="form-control text-center" readonly=true aria-label="Source link" aria-describedby="button-addon2"
                placeholder={"Hidden, contains sensitive infomation"}
            />
            <button class="btn btn-outline-secondary" type="button" id="srcCopy"
                onclick={login_copy_click_callback}
            >{ *copy_button_text }</button>
        </div>
    }
}