use yew::prelude::*;

use twitch_sources_rework::front_common::source_options::NormalSourceOptions;

#[function_component(PredictionsSettings)]
pub fn predictions_settings() -> Html {
    let options: UseStateHandle<NormalSourceOptions> = use_state(Default::default);

    html! {
        <>
            { NormalSourceOptions::to_html(&options) }
        </>
    }
}