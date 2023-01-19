use strum::IntoEnumIterator;
use twitch_sources_rework::front_common::SourceColor;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChooserProps {
    pub color: UseStateHandle<SourceColor>
}

#[function_component(SourceColorChooser)]
pub fn source_color_chooser(props: &ChooserProps) -> Html {
    html! {
        <div class="p-3 border border-dark border-2 h-100">
            <h5 class="text-center">{ "Text color" }</h5>

            {
                SourceColor::iter().map(
                    |color| html! {
                        <div class="form-check">
                            <input
                                class="form-check-input"
                                type="radio"
                                name="textColor"
                                id={"textColor".to_string() + &color.to_string()}
                                checked={*props.color == color}
                                oninput={
                                    let state = props.color.clone();
                                    move |_| state.set(color)
                                }
                            />
                                <label class="form-check-label" for={"textColor".to_string() + &color.to_string()}>
                                { color.to_string() }
                            </label>
                        </div>
                    }
                ).collect::<Html>()
            }

        </div>
    }
}