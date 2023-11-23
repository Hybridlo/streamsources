use twitch_sources_rework::{front_common::hypetrain::{components::HypetrainNormal, HypetrainState, HypetrainSourceOptions}, common_data::eventsub_msgs::EventSubMessage};
use yew::{function_component, html, use_effect_with_deps, UseStateHandle, use_state_eq};
use yew_hooks::{use_web_socket_with_options, UseWebSocketOptions};

#[function_component(App)]
pub fn app() -> Html {
    // gather data
    let window = web_sys::window().expect("Gotta have a window object");
    let location = window.location();
    let path = location.host().expect("Gotta have a location origin");
    let protocol = location.protocol().expect("Gotta have a location protocol");

    let query_string = location.search()
        .unwrap_or_default()
        .trim_start_matches('?')
        .to_string();

    let query_data = serde_urlencoded::de::from_str::<HypetrainSourceOptions>(&query_string)
        .unwrap_or_default();

    let ws_protocol = if protocol.contains("https") { "wss".to_string() } else { "ws".to_string() };
    let ws = use_web_socket_with_options(
        ws_protocol + "://" + &path + "/ws/sources/hype_train",
        UseWebSocketOptions { reconnect_limit: Some(1000), reconnect_interval: Some(10000), ..Default::default() }
    );

    // state setup
    let error_state: UseStateHandle<Option<String>> = use_state_eq(|| None);
    let last_message = use_state_eq(|| HypetrainState::default());

    {
        let ws = ws.clone();
        let error_state = error_state.clone();
        let last_message = last_message.clone();

        use_effect_with_deps(move |ws_message| {
            if let Some(message) = &**ws_message {
                match serde_json::de::from_str::<EventSubMessage>(message) {
                    Ok(parsed) => {

                        match HypetrainState::try_from(parsed) {
                            Ok(res) => {
                                let id_changed = res.id != last_message.id;
                                let higher_level = res.curr_level * res.needed_points + res.curr_points
                                    > last_message.curr_level * last_message.needed_points + last_message.curr_points;

                                let same_level_but_has_cooldown_time = (
                                    res.curr_level * res.needed_points + res.curr_points
                                    == last_message.curr_level * last_message.needed_points + last_message.curr_points
                                ) && res.cooldown_ends_at.is_some();
                                
                                if id_changed || higher_level || same_level_but_has_cooldown_time {
                                    last_message.set(res)
                                }
                            },
                            Err(err) => error_state.set(Some(err.to_string())),
                        }
                    },
                    Err(err) => error_state.set(Some(err.to_string())),
                }
            }
            
            || ()
        }, ws.message);
    }

    html! {
        <>
            <HypetrainNormal
                settings={query_data}
                last_message={last_message.clone()}
            />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
