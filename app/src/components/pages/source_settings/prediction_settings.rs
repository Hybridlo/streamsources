use std::str::FromStr;
use std::{rc::Rc, cell::RefCell};

use chrono::Duration;
use wasm_bindgen::{JsValue, JsCast};
use gloo_timers::callback::Timeout;
use yew::use_state;
use yew::prelude::*;
use yew_hooks::prelude::use_clipboard;
use chrono::offset::Utc;

use web_sys::{HtmlSelectElement, HtmlOptionElement};
use web_sys::console::log_1;

use twitch_sources_rework::front_common::SourceColor;
use twitch_sources_rework::front_common::predictions::*;

#[derive(Default, Clone, PartialEq, Debug)]
pub enum PredictionSkins {
    #[default]
    List
}

impl ToString for PredictionSkins {
    fn to_string(&self) -> String {
        match self {
            PredictionSkins::List => "List".to_string()            
        }
    }
}

impl FromStr for PredictionSkins {
    type Err = JsValue;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "List" => Ok(Self::List),
            _ => Err("Select contained invalid value".into())
        }
    }
}

impl PredictionSkins {
    fn all_options() -> Vec<Self> {
        vec![Self::List]
    }

    fn to_source_name(&self) -> String {
        match self {
            PredictionSkins::List => "list".to_string()
        }
    }
}

#[derive(Default)]
pub struct PredictionsSettingsState {
    options: PredictionsSourceOptions,
    copy_timer: Rc<RefCell<Option<Timeout>>>,
    copy_button_text: String,
    skin: PredictionSkins
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
            skin: self.skin.clone(),
            ..*self
        }
    }

    pub fn with_is_expanded(&self, is_expanded: bool) -> Self {
        Self {
            options: self.options.with_is_expanded(is_expanded),
            copy_timer: self.copy_timer.clone(),
            copy_button_text: self.copy_button_text.clone(),
            skin: self.skin.clone(),
            ..*self
        }
    }

    pub fn with_copy_timer(&self, copy_timer: Timeout) -> Self {
        Self {
            copy_timer: Rc::new(RefCell::new(Some(copy_timer))),
            copy_button_text: self.copy_button_text.clone(),
            skin: self.skin.clone(),
            ..*self
        }
    }

    pub fn with_copy_button_text(&self, new_text: String) -> Self {
        Self {
            copy_button_text: new_text,
            copy_timer: self.copy_timer.clone(),
            skin: self.skin.clone(),
            ..*self
        }
    }

    pub fn with_skin(&self, skin: PredictionSkins) -> Self {
        Self {
            copy_timer: self.copy_timer.clone(),
            copy_button_text: self.copy_button_text.clone(),
            skin,
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

    fn render_skin_options(state: &UseStateHandle<PredictionsSettingsState>) -> Html {
        let select_ref = use_node_ref();

        let onchange = {
            let state = state.clone();
            let select_ref = select_ref.clone();

            Callback::from(move |_| {
                let select = select_ref.cast::<HtmlSelectElement>();

                if let Some(select) = select {
                    let selected_skin = select
                        .selected_options()
                        .item(0)
                        .expect("msg")
                        .unchecked_into::<HtmlOptionElement>()
                        .value()
                        .parse::<PredictionSkins>();

                    if let Ok(selected_skin) = selected_skin {
                        log_1(&format!("{:?}", selected_skin).into());
                        state.with_skin(selected_skin);
                    }
                }
            })
        };

        html! {
            <>
                <h5 class="container text-center my-2">{ "Choose source appearance" }</h5>
                <select class="form-select" ref={select_ref} {onchange}>
                    {
                        PredictionSkins::all_options().into_iter().map(|skin| {
                            html! {
                                <option selected={skin == state.skin}>{ skin.to_string() }</option>
                            }
                        }).collect::<Html>()
                    }
                </select>
            </>
        }
    }

    fn render_copy_button(state: &UseStateHandle<PredictionsSettingsState>) -> Result<Html, JsValue> {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("Could not get window"))?;
        let location = window.location();
        let href = location.protocol()? + "//" + &location.host()?;

        let clipboard_handle = use_clipboard();

        let copy_click_callback = {
            let state = state.clone();
            let href = href.clone();

            Callback::from(move |_| {
                let href = href.clone();
                
                clipboard_handle.write_text(
                        href + "/sources"
                        + "/" + PredictionsSettingsState::get_source_endpoint()
                        + "/" + &state.skin.to_source_name()
                        + "?" + &state.options.item_to_params()
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
                            href.clone() + "/sources"
                            + "/" + PredictionsSettingsState::get_source_endpoint()
                            + "/" + &state.skin.to_source_name()
                            + "?" + &state.options.item_to_params()
                        }
                    />
                    <button class="btn btn-outline-secondary" type="button" id="srcCopy"
                        onclick={copy_click_callback}
                    >{ &*state.copy_button_text }</button>
                </div>
            </>
        })
    }

    fn test_predictions_state() -> Html {
        let source_state = use_state(|| PredictionState {
            id: "".to_string(),
            title: "Test title".to_string(),
            winning_outcome_id: None,
            outcomes: vec![
                PredictionOutcomeState {
                    id: "1".to_string(),
                    title: "Very complicated long title to do a test".to_string(),
                    color: "blue".to_string(),
                    users: 1,
                    channel_points: 10_000,
                    top_predictors: vec![],
                }, PredictionOutcomeState {
                    id: "2".to_string(),
                    title: "Title2".to_string(),
                    color: "pink".to_string(),
                    users: 1,
                    channel_points: 20_000,
                    top_predictors: vec![],
                }
            ],
            lock_time: Utc::now() + Duration::seconds(20),
            status: PreditionStatus::Locked
        });

        let show_element_state = use_state(|| false);
        let show_status_state = use_state(|| true);
        let status_state = use_state(|| PreditionStatus::default());

        let animator = use_mut_ref(|| PredictionStateAnimator::new(
            source_state.setter(),
            &source_state,
            show_element_state.setter(),
            show_status_state.setter(),
            status_state.setter()
        ));

        let change_state_callback = {
            let animator = animator.clone();
            let source_state = source_state.clone();

            Callback::from(move |_| {
                let mut animator_borrow = animator.borrow_mut();

                let opt1_p = source_state.outcomes[0].channel_points;
                let opt2_p = source_state.outcomes[1].channel_points;

                (*animator_borrow).set_state(PredictionState::new(
                    "".to_string(),
                    "Test title".to_string(),
                    None,
                    vec![
                        PredictionOutcomeState {
                            id: "1".to_string(),
                            title: "Very complicated long title to do a test".to_string(),
                            color: "blue".to_string(),
                            users: 1,
                            channel_points: opt1_p + 10_000,
                            top_predictors: vec![],
                        },
                        PredictionOutcomeState {
                            id: "2".to_string(),
                            title: "Title2".to_string(),
                            color: "pink".to_string(),
                            users: 1,
                            channel_points: opt2_p + 10_000,
                            top_predictors: vec![],
                        }
                    ],
                    Utc::now() + Duration::seconds(20),
                    PreditionStatus::Finished
                ),
                &source_state)
            })
        };

        html! {
            <>
                <div style="height: 500px">
                    <components::PredictionsList state={source_state} is_white={true} {show_status_state} {show_element_state} {status_state}/>
                </div>
                <button onclick={change_state_callback}>{"Test"}</button>
            </>
        }
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
                        { PredictionsSettingsState::render_skin_options(&state) }
                    </div>
                </div>
                { PredictionsSettingsState::render_copy_button(&state)? }
                { PredictionsSettingsState::test_predictions_state() }
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