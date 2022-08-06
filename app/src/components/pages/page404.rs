use yew::prelude::*;
use crate::components::containers::Header;

#[function_component(Page404)]
pub fn page_404() -> Html {
    html! {
        <>
            <Header />
            <div class="container shadow bg-light border border-primary border-2 p-2 rounded text-center">
                { "Page not found!" }
            </div>
        </>
    }
}