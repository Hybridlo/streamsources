use std::{rc::Rc, cell::RefCell};

use wasm_bindgen::JsValue;
use gloo_timers::callback::Timeout;
use yew::use_state;
use yew::prelude::*;
use yew_hooks::prelude::use_clipboard;

use twitch_sources_rework::front_common::source_options::{PredictionsSourceOptions, SourceColor};

#[derive(Default)]
pub struct PredictionsSettingsState {
    options: PredictionsSourceOptions,
    copy_timer: Rc<RefCell<Option<Timeout>>>,
    copy_button_text: String,
}

impl PredictionsSettingsState {
    pub fn new() -> Self {
        Self {
            copy_button_text: "Copy".to_string(),
            ..Default::default()
        }
    }

    // i could not make a new State
    // then i'd need to put options behind a RefCell
    // and mutate it there, but it feels wrong somehow
    pub fn with_color(&self, color: SourceColor) -> Self {
        Self {
            options: self.options.with_color(color),
            copy_timer: self.copy_timer.clone(),
            copy_button_text: self.copy_button_text.clone(),
            ..*self
        }
    }

    pub fn with_is_expanded(&self, is_expanded: bool) -> Self {
        Self {
            options: self.options.with_is_expanded(is_expanded),
            copy_timer: self.copy_timer.clone(),
            copy_button_text: self.copy_button_text.clone(),
            ..*self
        }
    }

    pub fn with_copy_timer(&self, copy_timer: Timeout) -> Self {
        Self {
            copy_timer: Rc::new(RefCell::new(Some(copy_timer))),
            copy_button_text: self.copy_button_text.clone(),
            ..*self
        }
    }

    pub fn with_copy_button_text(&self, new_text: String) -> Self {
        Self {
            copy_button_text: new_text,
            copy_timer: self.copy_timer.clone(),
            ..*self
        }
    }

    pub fn get_source_endpoint() -> &'static str {
        return "predictions";
    }

    fn render_color_option(state: &UseStateHandle<PredictionsSettingsState>, color: SourceColor) -> Html {
        // makes sure to fill all options if there'll be more
        match color {
            SourceColor::White => html! {
                <div class="form-check">
                    <input
                        class="form-check-input" type="radio" name="textColor" id="textColorWhite" checked={state.options.color == SourceColor::White}
                        oninput={
                                let state = state.clone();
                                move |_| state.set((*state).with_color(color))
                            }
                        />
                        <label class="form-check-label" for="textColorWhite">
                        { "While" }
                    </label>
                </div>
            },
            SourceColor::Black => html! {
                <div class="form-check">
                    <input
                        class="form-check-input" type="radio" name="textColor" id="textColorBlack" checked={state.options.color == SourceColor::Black}
                        oninput={
                                let state = state.clone();
                                move |_| state.set((*state).with_color(color))
                            }
                        />
                        <label class="form-check-label" for="textColorBlack">
                        { "Black" }
                    </label>
                </div>
            },
        }
    }

    fn render_color_options(state: &UseStateHandle<PredictionsSettingsState>) -> Html {
        html! {
            <div class="col-6">
                <div class="p-3 border border-dark border-2 h-100">
                    <h5 class="text-center">{ "Text color" }</h5>
                    { PredictionsSettingsState::render_color_option(state, SourceColor::Black) }
                    { PredictionsSettingsState::render_color_option(state, SourceColor::White) }
                </div>
            </div>
        }
    }

    fn render_expand_option(state: &UseStateHandle<PredictionsSettingsState>, is_expanded: bool) -> Html {
        match is_expanded {
            true => html! {
                <div class="col-12">
                    <div class="form-check">
                        <input
                            class="form-check-input" type="radio" name="expandedOption" id="expandedOptionTrue" checked={state.options.is_expanded}
                            oninput={
                                let state = state.clone();
                                move |_| state.set((*state).with_is_expanded(is_expanded))
                            }
                        />
                            <label class="form-check-label" for="expandedOptionTrue">
                            { "Maximized" }
                        </label>
                    </div>
                </div>
            },
            false => html! {
                <div class="col-12">
                    <div class="form-check">
                        <input
                            class="form-check-input" type="radio" name="expandedOption" id="expandedOptionFalse" checked={!state.options.is_expanded}
                            oninput={
                                let state = state.clone();
                                move |_| state.set((*state).with_is_expanded(is_expanded))
                            }
                        />
                        <label class="form-check-label" for="expandedOptionFalse">
                            { "Minimized" }
                        </label>
                        </div>
                </div>
            },
        }
    }

    fn render_expand_options(state: &UseStateHandle<PredictionsSettingsState>) -> Html {
        html! {
            <div class="col-6">
                <div class="p-3 border border-dark border-2 h-100">
                    <h5 class="text-center">{ "Size options" }</h5>
                    <div class="row">
                        { PredictionsSettingsState::render_expand_option(state, false) }
                        { PredictionsSettingsState::render_expand_option(state, true) }
                    </div>
                </div>
            </div>
        }
    }

    fn render_copy_button(state: &UseStateHandle<PredictionsSettingsState>) -> Result<Html, JsValue> {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("Could not get window"))?;
        let location = window.location();
        let href = location.href()?;

        let clipboard_handle = use_clipboard();

        let copy_click_callback = {
            let state = state.clone();
            let href = href.clone();

            Callback::from(move |_| {
                let href = href.clone();
                
                clipboard_handle.write_text(
                        href
                        + "/" + PredictionsSettingsState::get_source_endpoint()
                        + "?" + state.options.item_to_params().as_str()
                );

                if let Some(copy_timer) = (*state.copy_timer).take() {
                    copy_timer.cancel();
                }

                let timeout = {
                    let state = state.clone();
                    Timeout::new(2_000, move || {
                        state.set((*state).with_copy_button_text("Copy".to_string()));
                    })
                };

                state.set(
                    (*state)
                        .with_copy_button_text("Copied!".to_string())
                        .with_copy_timer(timeout)
                );
            })
        };

        Ok(html! {
            <>
                <h5 class="container text-center mb-2">{ "Link to source" }</h5>
                <div class="container input-group mb-3">
                    <input id="srcLink" type="text" class="form-control text-center" readonly=true aria-label="Source link" aria-describedby="button-addon2"
                        placeholder={
                            href.clone()
                            + "/" + PredictionsSettingsState::get_source_endpoint()
                            + "?" + state.options.item_to_params().as_str()
                        }
                    />
                    <button class="btn btn-outline-secondary" type="button" id="srcCopy"
                        onclick={copy_click_callback}
                    >{ &*state.copy_button_text }</button>
                </div>
            </>
        })
    }
    
    pub fn to_html() -> Result<Html, JsValue> {
        let state: UseStateHandle<PredictionsSettingsState> = use_state(|| PredictionsSettingsState::new());

        Ok(html! {
            <>
                <h4 class="text-center">{ "Settings" }</h4>
                <div class="container mb-3">
                    <div class="row gx-3">
                        { PredictionsSettingsState::render_color_options(&state) }
                        { PredictionsSettingsState::render_expand_options(&state) }
                    </div>
                </div>
                { PredictionsSettingsState::render_copy_button(&state)? }
            </>
        })
    }
}

#[function_component(PredictionsSettings)]
pub fn predictions_settings() -> Html {
    match PredictionsSettingsState::to_html() {
        Ok(html_res) => html! {
            <>
                {
                    html_res
                }
            </>
        },
        Err(js_err) => html! {
            // TODO: proper html for error
            <>
                <h4 class="text-center p-2">{ "Normal source options render failed" }</h4>
                { format!{"Returned error: {:?}", js_err} }
            </>
        },
    }
}