use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::state::LoginState;

pub fn login_gate(gated: Html) -> Html {
    let (login_state, _) = use_store::<LoginState>();

    if let Some(_) = login_state.info {
        gated
    } else {
        html! {
            <p class="text-center"><strong>{"This feature requires you to be logged in"}</strong></p>
        }
    }
}