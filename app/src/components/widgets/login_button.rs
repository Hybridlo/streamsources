use yew::prelude::*;
use yew_hooks::use_async;
use yewdux::prelude::use_store;

use twitch_sources_client::apis::default_api::api_request_login_get;

use crate::state::ClientConfig;
use crate::state::ErrorState;

use web_sys::console::log_1;

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

            log_1(&"start".into());
            
            let res = api_request_login_get(&client_config.config, &href)
                                                    .await.map_err(|err| err.to_string());
            
            log_1(&"end".into());
            
            match res {
                Ok(data) => log_1(&format!("{:?}", data).into()),
                Err(err) => {
                    log_1(&"lol".into());

                    log_1(&"here".into());

                    error_state_setter.reduce(|_| ErrorState { show_error: true, error_message: err });
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
        <button onclick={button_onclick} disabled={async_state.loading} type="button" class="btn ms-auto login-button">
            { "Login with Twitch" }
        </button>
    }
}