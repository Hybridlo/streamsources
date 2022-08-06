use yew::prelude::*;

use crate::components::containers::Header;
use crate::components::widgets::SourceLink;
use crate::MainRoute;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <Header />
            <div class="container shadow bg-light border border-primary border-2 p-2 rounded">
                <h4 class="text-center p-2">{ "Available sources:" }</h4>
                <div class="list-group p-2">
                    <SourceLink href={MainRoute::NotFound} name="Predictions" />
                    <SourceLink href={MainRoute::NotFound} name="...with more coming soon!" disabled=true />
                </div>
            </div>
        </>
    }
}