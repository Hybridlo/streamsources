use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChooserProps {
    pub title: &'static str,
    pub true_text: &'static str,
    pub false_text: &'static str,
    pub bool_state: UseStateHandle<bool>
}

#[function_component(BooleanChooser)]
pub fn boolean_chooser(props: &ChooserProps) -> Html {
    html! {
        <div class="p-3 border border-dark border-2 h-100">
            <h5 class="text-center">{ props.title }</h5>
            <div class="row">

                <div class="col-12">
                    <div class="form-check">
                        <input
                            class="form-check-input" type="radio" name="expandedOption" id="expandedOptionTrue" checked={*props.bool_state}
                            oninput={
                                let state = props.bool_state.clone();
                                move |_| state.set(true)
                            }
                        />
                            <label class="form-check-label" for="expandedOptionTrue">
                            { props.true_text }
                        </label>
                    </div>
                </div>

                <div class="col-12">
                    <div class="form-check">
                        <input
                            class="form-check-input" type="radio" name="expandedOption" id="expandedOptionFalse" checked={!*props.bool_state}
                            oninput={
                                let state = props.bool_state.clone();
                                move |_| state.set(false)
                            }
                        />
                            <label class="form-check-label" for="expandedOptionFalse">
                            { props.false_text }
                        </label>
                    </div>
                </div>
            </div>
        </div>
    }
}