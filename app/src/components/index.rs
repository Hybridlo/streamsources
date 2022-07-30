use yew::prelude::*;
use super::header::Header;

#[function_component(Index)]
pub fn index() -> Html {
    html! {
        <>
            <Header />
            <div class="container shadow bg-light border border-primary border-2 p-2 rounded">
                {"Sum txt"}
            </div>
            <div>
                <button>{"+1"}</button>
                <button>{"+2"}</button>
                <button>{"Fetch"}</button>
                <button>{"Fetch"}</button>
            </div>
        </>
    }
}