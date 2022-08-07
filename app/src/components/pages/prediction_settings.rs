use yew::prelude::*;

use twitch_sources_rework::front_common::source_options::NormalSourceOptions;

#[function_component(PredictionsSettings)]
pub fn predictions_settings() -> Html {
    html! {
        <>
            { NormalSourceOptions::to_html() }
        </>
    }
}