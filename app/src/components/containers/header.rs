use yew::prelude::*;
use crate::components::widgets::LoginButton;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="border-bottom mb-3">
            <div class="container">
                <div class="d-flex flex-row p-2 align-items-center">
                    <span class="site-title">{ "Twitch sources (beta)" }</span>
                    <LoginButton />
                </div>
            </div>
        </div>
    }
}