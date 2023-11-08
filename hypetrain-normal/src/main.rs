use twitch_sources_rework::front_common::hypetrain::components::HypetrainNormal;
use yew::{function_component, html};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <HypetrainNormal />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
