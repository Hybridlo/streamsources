use yew::prelude::*;
use yew_hooks::use_async;
use yewdux::prelude::use_store;

use twitch_sources_client::apis::default_api::api_request_login_get;

use crate::state::ClientConfig;
use crate::state::ErrorState;

const TWITCH_AUTH_URL: &str = "https://id.twitch.tv/oauth2/authorize";

#[function_component(LoginButton)]
pub fn login_button() -> Html {
    let (client_config, _) = use_store::<ClientConfig>();
    let (_, error_state_setter) = use_store::<ErrorState>();

    let async_state = {
        let error_state_setter = error_state_setter.clone();
        use_async::<_, (), ()>(async move {
            let window = web_sys::window().unwrap();
            let location = window.location();
            let href = location.href().unwrap();
            
            let res = api_request_login_get(&client_config.config, &href)
                                                    .await;
            
            match res {
                Ok(data) => {
                    match serde_urlencoded::ser::to_string(data) {
                        Ok(data_encoded) => {
                            location.set_href(
                                &(TWITCH_AUTH_URL.to_string()
                                + "?"
                                + &data_encoded)
                            );
                        }
                        Err(err) => error_state_setter.reduce(|_| ErrorState { show_error: true, error_message: err.to_string() })
                    }
                },
                Err(err) => {
                    error_state_setter.reduce(|_| ErrorState { show_error: true, error_message: err.to_string() });
                }
            }

            Ok(())
        })
    };
    
    let button_onclick = {
        let state = async_state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    html! {
        <button onclick={button_onclick} disabled={async_state.loading} type="button" class="btn login-button">
            { "Login with Twitch" }
        </button>
    }
}