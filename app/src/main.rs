mod components;

use anyhow::Result;
use twitch_sources_rework::MyTestStruct;
use yew::prelude::*;
use yew_router::prelude::*;
use components::*;

#[derive(Clone, Routable, PartialEq, Copy)]
pub enum MainRoute {
    #[at("/")]
    Index,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &MainRoute) -> Html {
    match routes {
        MainRoute::Index => html! { <Index /> },
        MainRoute::NotFound => html! { <Page404 /> },
    }
}

#[function_component(Model)]
fn model() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}