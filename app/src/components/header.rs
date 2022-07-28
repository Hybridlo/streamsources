use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="border-bottom">
            <div class="container text-center">
                <span class="site-title">{ "Twitch sources (beta)" }</span>
            </div>
        </div>
    }
}