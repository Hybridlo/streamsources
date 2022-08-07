use yew::prelude::*;

use crate::components::widgets::SourceLink;
use crate::components::BaseRoute;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <h4 class="text-center p-2">{ "Available sources:" }</h4>
            <div class="list-group p-2">
                <SourceLink href={BaseRoute::Predictions} name="Predictions" />
                <SourceLink href={BaseRoute::NotFound} name="...with more coming soon!" disabled=true />
            </div>
        </>
    }
}