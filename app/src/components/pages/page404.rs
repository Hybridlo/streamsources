use yew::prelude::*;

#[function_component(Page404)]
pub fn page_404() -> Html {
    html! {
        <>
            <div class="text-center">
                { "Page not found!" }
            </div>
        </>
    }
}