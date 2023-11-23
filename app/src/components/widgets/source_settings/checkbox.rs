use yew::{UseStateHandle, function_component, html, Properties};

#[derive(PartialEq, Properties)]
pub struct CheckboxProps {
    pub checked_state: UseStateHandle<bool>,
    pub is_active: bool,
    pub label: &'static str,
    pub id: &'static str,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    html! {
        <div class="form-check">
            <input 
                type="checkbox"
                id={props.id}
                class="form-check-input"
                checked={*props.checked_state}
                oninput={
                    let checked_state = props.checked_state.clone();
                    move |_| checked_state.set(!*checked_state)
                }
                disabled={!props.is_active}
            />
            <label class="form-label" for={props.id}>{props.label}</label>
        </div>
    }
}