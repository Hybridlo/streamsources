use http::StatusCode;
use twitch_sources_client::apis::Error;
use twitch_sources_client::apis::ResponseContent;
use yew::prelude::*;
use yew_hooks::use_async;
use yewdux::prelude::use_store;

use twitch_sources_client::apis::default_api::api_login_check_get;

use crate::state::LoginInfo;
use crate::state::LoginState as LoginStateData;
use crate::state::ClientConfig;
use crate::state::ErrorState;

use crate::components::widgets::LoginButton;

const CHECK_TIMEOUT: i64 = 3600;

/// This is probably the only component that will check and update
/// the login state, it's always on the page so it works well for that,
/// although i'm not sure it's the best way to go about that
#[function_component(LoginState)]
pub fn login_state() -> Html {
    let (login_state, login_state_setter) = use_store::<LoginStateData>();
    let (client_config, _) = use_store::<ClientConfig>();
    let (_, error_state_setter) = use_store::<ErrorState>();

    let async_state = {
        let login_state_setter = login_state_setter.clone();
        use_async::<_, (), ()>(async move {
            // so this is needed, because otherwise this will be called in a loop
            // of state change -> component rerender -> state change -> ...
            let res = api_login_check_get(&client_config.config).await;

            match res {
                Ok(data) => login_state_setter.reduce(|_| LoginStateData {
                    info: Some(LoginInfo { username: data.username }),
                    last_check: chrono::offset::Utc::now().naive_utc()
                }),
                // this is quite a cumbersome way to do this, but the best way (i think)
                Err(Error::ResponseError(ResponseContent { status: StatusCode::FORBIDDEN, content: _, entity: _ })) => {
                    login_state_setter.reduce(|_| LoginStateData {
                        info: None,
                        last_check: chrono::offset::Utc::now().naive_utc()
                    })
                },
                Err(_) => {
                    error_state_setter.reduce(|_| ErrorState {
                        show_error: true,
                        error_message: "Unknown error while trying to resolve user status".to_string()
                    });

                    login_state_setter.reduce(|_| LoginStateData {
                        info: None,
                        last_check: chrono::offset::Utc::now().naive_utc()
                    })
                }
            }

            Ok(())
        })
    };

    if chrono::offset::Utc::now().naive_utc() - chrono::Duration::seconds(CHECK_TIMEOUT) > login_state.last_check {
        login_state_setter.reduce(|_| LoginStateData {
            info: None,
            last_check: chrono::offset::Utc::now().naive_utc()
        });
        
        async_state.run();
    }

    html! {
        <div class="ms-auto login-state">
        {
            if async_state.loading {
                html! {
                    <>
                        <span>{ "Loading..." }</span>
                    </>
                }
            } else {
                match &login_state.info {
                    Some(data) => {
                        html! {
                            <span>{ "Welcome, "}{ data.username.clone() }</span>
                        }
                    },
                    None => html! {
                        <LoginButton />
                    },
                }
            }
        }
        </div>
    }
}