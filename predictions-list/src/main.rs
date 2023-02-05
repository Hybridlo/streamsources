use twitch_sources_rework::common_data::EventSubMessage;
use twitch_sources_rework::front_common::predictions::PredictionState;
use twitch_sources_rework::front_common::predictions::PredictionStateAnimator;
use twitch_sources_rework::front_common::predictions::PredictionsSourceOptions;
use twitch_sources_rework::front_common::predictions::PredictionStatus;
use twitch_sources_rework::front_common::predictions::components::PredictionsList;
use yew::prelude::*;

use yew_hooks::use_web_socket_with_options;
use yew_hooks::UseWebSocketOptions;

#[function_component(App)]
fn app() -> Html {
    // gather data
    let window = web_sys::window().expect("Gotta have a window object");
    let location = window.location();
    let path = location.host().expect("Gotta have a location origin");
    let protocol = location.protocol().expect("Gotta have a location protocol");

    let query_string = location.search()
        .unwrap_or_default()
        .trim_start_matches('?')
        .to_string();

    let query_data = serde_urlencoded::de::from_str::<PredictionsSourceOptions>(&query_string)
        .unwrap_or_default();

    let ws_protocol = if protocol.contains("https") { "wss".to_string() } else { "ws".to_string() };
    let ws = use_web_socket_with_options(
        ws_protocol + "://" + &path + "/ws/sources/predictions",
        UseWebSocketOptions { reconnect_limit: Some(1000), reconnect_interval: Some(10000), ..Default::default() }
    );

    // state setup
    let error_state: UseStateHandle<Option<String>> = use_state(|| None);

    let source = use_state(|| PredictionState::default());
    let show_element = use_state(|| false);
    let show_status = use_state(|| true);
    let status = use_state(|| PredictionStatus::default());

    let animator = use_mut_ref(|| PredictionStateAnimator::new(
        source.setter(),
        &source,
        show_element.setter(),
        show_status.setter(),
        status.setter()
    ));

    {
        let ws = ws.clone();
        let state = source.clone();
        let animator = animator.clone();
        let error_state = error_state.clone();

        use_effect_with_deps(move |message| {
            if let Some(message) = &**message {
                match serde_json::de::from_str::<EventSubMessage>(message) {
                    Ok(parsed) => {

                        match parsed.try_into() {
                            Ok(res) => {animator.borrow_mut().set_state(res, &*state)},
                            Err(err) => error_state.set(Some(err.to_string())),
                        }
                    },
                    Err(err) => error_state.set(Some(err.to_string())),
                }
            }
            || ()
        }, ws.message)
    }

    if let Some(err_text) = &*error_state {
        return html! {
            <>
                <div>{ "Error!" }</div>
                <div>{ err_text }</div>
                <div>{ "Try refreshing, and if error occurs again, please let us know" }</div>
            </>
        }
    }

    match *ws.ready_state {
        yew_hooks::UseWebSocketReadyState::Connecting => html! {
            <h4 class="text-center">{ "Connecting..." }</h4>
        },
        yew_hooks::UseWebSocketReadyState::Open => html! {
            <PredictionsList
                color={query_data.color}
                is_maximized={query_data.is_maximized}
                state={source}
                show_element_state={show_element}
                show_status_state={show_status}
                status_state={status}
            />
        },
        yew_hooks::UseWebSocketReadyState::Closing
      | yew_hooks::UseWebSocketReadyState::Closed => html! {
            <h4 class="text-center">{ "Connection lost, reconnecting in 10 seconds..." }</h4>
        },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}