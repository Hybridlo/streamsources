use wasm_bindgen::JsCast as _;
use web_sys::{InputEvent, Event};
use yew::{UseStateHandle, Properties, function_component, html, Callback};

#[derive(PartialEq, Properties)]
pub struct NumberInputProps {
    pub title: &'static str,
    pub number_state: UseStateHandle<i64>,
    pub min_val: i64,
    pub max_val: i64,
}

#[function_component(NumberInput)]
pub fn number_input(props: &NumberInputProps) -> Html {
    html!{
        <div class="p-3 border border-dark border-2 h-100">
            <h5 class="text-center">{ props.title }</h5>
            <div class="form-check">
                <input
                    type="number"
                    class="form-control text-center"
                    min={props.min_val.to_string()}
                    max={props.max_val.to_string()}
                    value={props.number_state.to_string()}
                    onchange={
                        let number_state = props.number_state.clone();
                        Callback::from(move |ev: Event| {
                            let input = ev
                                .target()
                                .unwrap()
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .unwrap();

                            number_state.set(
                                input.value()
                                    .parse().ok()
                                    .unwrap_or(1)
                            )
                        }
                    )}
                />
            </div>
        </div>
    }
}