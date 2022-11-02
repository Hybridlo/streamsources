use yew::prelude::*;
use yew_style_in_rs::style;

use super::super::state::{PredictionState, PreditionStatus};
use crate::front_common::scalable_wrapper;

fn title(title: &str) -> Html {
    html! {
        <span class="mx-auto title">{ title }</span>
    }
}

fn status(status: &PreditionStatus) -> Html {
    match status {
        PreditionStatus::InProgress => html!{ <span class="timer">{"Should be a timer here"}</span> },
        PreditionStatus::Locked => html!{ <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20"><path fill="#C38000" d="m3,9h1V6a5,5 0 0,1 12,0V9h1v11H3M14,9V6a4,4 0 1,0-8,0v3" /></svg> },
        PreditionStatus::Finished => html!{ <svg xmlns="http://www.w3.org/2000/svg" height="20" width="20" viewBox="0 0 26 26"><path fill="var(--outlines)" d="M 13.03125 0 C 7.574219 0 4.34375 3.46875 4.34375 3.46875 C 4.105469 3.742188 4.03125 4.125 4.15625 4.46875 L 8.65625 17.03125 C 8.769531 17.359375 9.042969 17.601563 9.378906 17.679688 C 9.71875 17.753906 10.070313 17.652344 10.3125 17.40625 C 10.3125 17.40625 12.855469 15 15 15 C 16.359375 15 17.160156 15.222656 18 15.46875 C 18.839844 15.714844 19.761719 16 21 16 C 23.480469 16 25.46875 14.875 25.46875 14.875 C 25.792969 14.703125 25.996094 14.367188 26 14 L 26 1 C 26 0.617188 25.78125 0.265625 25.433594 0.0976563 C 25.085938 -0.0703125 24.675781 -0.0234375 24.375 0.21875 C 24.375 0.21875 22.035156 2 20.03125 2 C 19.027344 2 18.222656 1.601563 17.15625 1.09375 C 16.089844 0.585938 14.78125 0 13.03125 0 Z M 13.03125 2 C 13.304688 2 13.566406 2.03125 13.8125 2.0625 L 14.65625 5.28125 C 15.5625 5.28125 18.09375 6.53125 18.09375 6.53125 L 17.375 3.375 C 18.148438 3.707031 18.992188 4 20.03125 4 C 20.558594 4 21.050781 3.921875 21.53125 3.8125 L 21.78125 6.90625 C 21.78125 6.90625 22.753906 6.648438 24 5.90625 L 24 10.0625 C 22.949219 10.511719 22.09375 10.65625 22.09375 10.65625 L 22.34375 13.84375 C 21.9375 13.925781 21.507813 14 21 14 C 20.546875 14 20.15625 13.957031 19.78125 13.875 L 19.03125 10.625 C 18.089844 10.378906 16.570313 9.941406 15.875 9.84375 L 16.75 13.125 C 16.230469 13.046875 15.671875 13 15 13 C 14.269531 13 13.625 13.132813 13.03125 13.3125 L 11.84375 10.1875 C 10.742188 10.613281 9.683594 11.414063 8.96875 12 L 7.34375 7.46875 C 8.765625 6.398438 10.1875 5.875 10.1875 5.875 L 8.96875 2.75 C 10.015625 2.324219 11.355469 2 13.03125 2 Z M 10.1875 5.875 L 11.84375 10.1875 C 11.84375 10.1875 13.1875 9.71875 14.09375 9.71875 C 15 9.71875 15.875 9.84375 15.875 9.84375 L 14.65625 5.28125 C 14.65625 5.28125 14.332031 5.21875 13.90625 5.21875 C 11.875 5.21875 10.1875 5.875 10.1875 5.875 Z M 19.03125 10.625 C 19.03125 10.625 19.980469 10.8125 20.625 10.8125 C 21.269531 10.8125 22.09375 10.65625 22.09375 10.65625 L 21.78125 6.90625 C 21.78125 6.90625 21.164063 7.09375 20.40625 7.09375 C 19.226563 7.09375 18.09375 6.53125 18.09375 6.53125 Z M 2.59375 2.65625 C 2.464844 2.640625 2.320313 2.648438 2.1875 2.6875 L 1.625 2.84375 C 1.492188 2.859375 1.367188 2.902344 1.25 2.96875 L 0.8125 3.09375 C 0.28125 3.246094 -0.0273438 3.816406 0.125 4.34375 C 0.257813 4.800781 0.699219 5.089844 1.15625 5.0625 L 8.375 25.0625 C 8.457031 25.4375 8.75 25.734375 9.128906 25.820313 C 9.503906 25.90625 9.894531 25.769531 10.132813 25.46875 C 10.375 25.164063 10.417969 24.753906 10.25 24.40625 L 3.0625 4.46875 C 3.386719 4.226563 3.554688 3.785156 3.4375 3.375 C 3.324219 2.976563 2.980469 2.703125 2.59375 2.65625 Z"></path></svg> },
    }
}

fn options_list(state: &PredictionState) -> Html {
    html! {
        <div>
            {
                state.outcomes.iter().map(
                    |option| html! {
                        <>
                            <div class="option d-grid" style={format!("--opt-color: {};", option.color)}>
                                <div class="option-title vh-align">{ &option.title }</div>
                                <div class="option-percents vh-align">{ state.get_outcome_percents(&option.id) }{"%"}</div>
                                <div class="option-points vh-align">{ option.channel_points }</div>
                            </div>
                        </>
                    }
                ).collect::<Html>()
            }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct ListProps {
    pub is_white: bool,
    pub state: UseStateHandle<PredictionState>,
    pub show_element_state: UseStateHandle<bool>,
    pub show_status_state: UseStateHandle<bool>,
    pub status_state: UseStateHandle<PreditionStatus>,
}

#[function_component(PredictionsList)]
pub fn predictions_list(props: &ListProps) -> Html {
    let outline_color = if props.is_white { "white" } else { "black" };
    let outline_color_opposite = if props.is_white { "black" } else { "white" };

    style! {
        let css = dyn css! {r#"
            --outlines: ${outline_color};
            --outlines-opposite: ${outline_color_opposite};

            & .element {
                transition: opacity 400ms ease-out;
            }

            & .element.hide {
                opacity: 0;
            }

            & .element.show {
                opacity: 1;
            }

            & .status {
                transition: opacity 200ms ease-out;
            }

            & .status.hide {
                opacity: 0;
            }

            & .status.show {
                opacity: 1;
            }

            & .title {
                color: var(--outlines);
                -webkit-text-stroke: 0.6px var(--outlines-opposite);
                font-size: 16px;
                font-weight: bold;
            }
            
            & .timer {
                color: var(--outlines);
                -webkit-text-stroke: 0.6px var(--outlines-opposite);
            }
            
            & .option {
                transition: color 400ms ease-out;
                color: var(--opt-color);
                -webkit-text-stroke: 0.6px var(--outlines);
                font-size: 16px;
                font-weight: bold;
                grid-template-columns: 3fr 1fr 2fr;
                vertical-align: middle;
            }

            & .option-title {
                max-width: 200px;
            }

            & .option-percents {
                font-size: 20px;
            }

            & .option-points {
                text-align: left;
            }

            & .vh-align {
                display: flex;
                justify-content: center;
                align-items: center;
            }
        "#};
    }

    scalable_wrapper(html! {
        <div class={classes!(css)}>
            <div class={if *props.show_element_state {"element show"} else {"element hide"}}>
                <div>
                    { title(&props.state.title) }
                </div>
                <div class={if *props.show_status_state {"status show"} else {"status hide"}}>
                    { status(&props.status_state) }
                </div>
                { options_list(&props.state) }
            </div>
        </div>
    })
}