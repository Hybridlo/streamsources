use std::{cell::RefCell, rc::Rc};

use gloo_timers::callback::Timeout;
use yew::prelude::*;
use yew_hooks::use_async;
use yewdux::prelude::use_store;

use twitch_sources_client::apis::default_api::api_test_get;

use crate::state::{ClientConfig, ErrorState};

#[derive(Properties, PartialEq)]
pub struct TestButtonProps {
    pub timeout_secs: u32,
    pub test_name: &'static str
}

#[function_component(TestButton)]
pub fn test_button(props: &TestButtonProps) -> Html {
    let (client_config, _) = use_store::<ClientConfig>();
    let (_, error_state_setter) = use_store::<ErrorState>();

    let button_active = use_state(|| true);
    let reenable_button_timeout: Rc<RefCell<Option<Timeout>>> = use_mut_ref(|| None);

    let async_state = {
        let error_state_setter = error_state_setter.clone();
        let test_name = props.test_name;
        let button_active = button_active.clone();

        use_async::<_, (), ()>(async move {
            // test_name could have been an enum, but the clientgen i use doesn't support it i think
            let res = api_test_get(&client_config.config, test_name).await;

            // we don't handle the Ok, because it's empty
            if let Err(err) = res {
                error_state_setter.reduce(|_| ErrorState { show_error: true, error_message: err.to_string() });
            }

            button_active.set(false);

            Ok(())
        })
    };

    let mut reenable_button_timeout_borrow = reenable_button_timeout.borrow_mut();
    if *button_active == false && reenable_button_timeout_borrow.is_none() {
        let button_active = button_active.clone();
        let reenable_button_timeout = reenable_button_timeout.clone();

        *reenable_button_timeout_borrow = Some(Timeout::new(props.timeout_secs * 1000, move || {
            button_active.set(true);
            reenable_button_timeout.replace(None);
        }));
    }

    let button_onclick = {
        let state = async_state.clone();
        Callback::from(move |_| {
            state.run();
        })
    };

    html! {
        <button onclick={button_onclick} disabled={async_state.loading || !*button_active} type="button" class="btn btn-outline-primary test-button">
            {
                if *button_active { "Test" } else {
                    if async_state.loading {"Starting..."} else {"Running..."}
                }
            }
        </button>
    }
}