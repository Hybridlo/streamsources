use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::state::ErrorState;
use crate::util::bootstrap::get_modal_by_id;

#[derive(Properties, PartialEq)]
pub struct ErrorModalProps {
    pub elem_id: String
}

#[function_component(ErrorModal)]
pub fn error_modal(props: &ErrorModalProps) -> Html {
    let modal_state = use_state(|| None);

    let (error_state, error_state_setter) = use_store::<ErrorState>();

    let on_close = {
        let setter = error_state_setter.clone();

        Callback::from(move |_| {
            setter.reduce(|_| ErrorState { show_error: false, error_message: "".to_string() })
        })
    };

    {
        let error_state = error_state.clone();
        let elem_id = props.elem_id.clone();
        use_effect(move || {
            if modal_state.is_none() {
                modal_state.set(Some(get_modal_by_id(&elem_id)));
            }

            if let Some(modal) = &*modal_state {
                if error_state.show_error {
                    modal.show();
                } else {
                    modal.hide();
                }
            }

            || ()
        })
    }

    html! {
        // modal is static, so that users can't click on the backdrop and bypass state change to hide the modal
        <div class="modal" id={props.elem_id.clone()} data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1">
            <div class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <div class="modal-header">
                        <h5 class="modal-title">{ "Error!" }</h5>
                    </div>
                    <div class="modal-body">
                        <p>{ &error_state.error_message }</p>
                    </div>
                    <div class="modal-footer">
                        <button type="button"
                            class="btn btn-primary"
                            onclick={on_close}>{ "Close" }</button>
                    </div>
                </div>
            </div>
        </div>
    }
}