use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::BaseRoute;
use crate::components::widgets::LoginState;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="border-bottom mb-3">
            <div class="container">
                <div class="d-flex flex-row p-2 align-items-center">
                    <span class="site-title"><Link<BaseRoute> to={BaseRoute::Index}>{ "StreamWidgets (beta)" }</Link<BaseRoute>></span>
                    <LoginState />
                </div>
            </div>
        </div>
    }
}