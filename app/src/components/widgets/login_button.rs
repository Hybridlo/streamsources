use yew::prelude::*;

#[function_component(LoginButton)]
pub fn login_button() -> Html {
    html! {
        <button type="button" class="btn ms-auto login-button">
            { "Login with Twitch" }
        </button>
    }
}