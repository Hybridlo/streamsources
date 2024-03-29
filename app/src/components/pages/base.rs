use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::containers::Header;
use super::Index;
use super::Page404;
use super::PredictionsSettings;
use super::HypetrainSettings;
//use twitch_sources_rework::front_common::predictions::components::PredictionsPie;

#[derive(Clone, Routable, PartialEq, Copy)]
pub enum BaseRoute {
    #[at("/")]
    Index,
    #[at("/predictions")]
    Predictions,
    #[at("/hype_train")]
    HypeTrain,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &BaseRoute) -> Html {
    match routes {
        BaseRoute::Index => html! { <Index /> },
        BaseRoute::NotFound => html! { <Page404 /> },
        BaseRoute::Predictions => html! { <PredictionsSettings /> },
        BaseRoute::HypeTrain => html! { <HypetrainSettings /> },
    }
}

#[function_component(Base)]
pub fn base() -> Html {

    html! {
        <>
            <Header />
            <div class="container shadow bg-light border border-primary border-2 p-2 rounded">
                <Switch<BaseRoute> render={Switch::render(switch)} />
            </div>
            //<div style={"height: 500px;"}>
                //<PredictionsPie />
            //</div>
        </>
    }
}