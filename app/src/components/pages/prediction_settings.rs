use strum::Display;
use strum::{EnumIter, IntoEnumIterator};
use yew::prelude::*;
use chrono::offset::Utc;

use twitch_sources_rework::front_common::SourceColor;
use twitch_sources_rework::front_common::predictions::*;

use crate::components::widgets::Carousel;
use crate::components::widgets::TestButton;
use crate::components::widgets::source_settings::BooleanChooser;
use crate::components::widgets::source_settings::SourceColorChooser;
use crate::components::widgets::source_settings::StaticSourceLink;
use crate::components::widgets::source_settings::LoginSourceLink;
use crate::util::login_gate;

#[derive(Default, Clone, PartialEq, Debug, EnumIter, Display)]
#[strum(serialize_all="snake_case")]
pub enum PredictionSkins {
    #[default]
    List
}

impl PredictionSkins {
    fn to_html(
        &self,
        color: SourceColor,
        is_maximized: bool,
        state: UseStateHandle<PredictionState>,
        show_element_state: UseStateHandle<bool>,
        show_status_state: UseStateHandle<bool>,
        status_state: UseStateHandle<PredictionStatus>
    ) -> Html {
        match self {
            PredictionSkins::List => html! {
                <components::PredictionsList {state} {color} {is_maximized} {show_status_state} {show_element_state} {status_state}/>
            },
        }
    }
}

#[derive(Properties, PartialEq)]
struct PredictionsCarouselProps {
    color: SourceColor,
    is_maximized: bool,
    carousel_state: UseStateHandle<usize>
}

#[function_component(PredictionsCarousel)]
fn predictions_carousel(props: &PredictionsCarouselProps) -> Html {
    let source_state = use_state(|| PredictionState {
        id: "".to_string(),
        title: "Test title".to_string(),
        winning_outcome_id: None,
        outcomes: vec![
            PredictionOutcomeState {
                id: "1".to_string(),
                title: "Very complicated long title to test this".to_string(),
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
        lock_time: Utc::now(),
        status: PredictionStatus::Locked
    });

    let show_element_state = use_state(|| true);
    let show_status_state = use_state(|| true);
    let status_state = use_state(|| PredictionStatus::Locked);

    // this can be expanded to have some animations periodically to showcase better
    /* let animator = use_mut_ref(|| PredictionStateAnimator::new(
        source_state.setter(),
        &source_state,
        show_element_state.setter(),
        show_status_state.setter(),
        status_state.setter()
    )); */

    let skins: Vec<Html> = PredictionSkins::iter().map(|skin| skin.to_html(
        props.color,
        props.is_maximized,
        source_state.clone(),
        show_element_state.clone(),
        show_status_state.clone(),
        status_state.clone()
    )).collect();

    html! {
        <Carousel active_item={props.carousel_state.clone()} carousel_size={3} items={skins} height={300} />
    }
}

#[function_component(PredictionsSettings)]
pub fn predictions_settings() -> Html {
    let source_color = use_state(|| SourceColor::default());
    let is_maximized = use_state(|| false);

    let carousel_state = use_state_eq(|| 0);
    let chosen_skin = PredictionSkins::iter().get(*carousel_state).expect("No way carousel gets out of bounds of the iter");

    let collected_options = PredictionsSourceOptions { color: *source_color, is_maximized: *is_maximized };
    
    html! {
        <>
            <h4 class="text-center">{ "Settings" }</h4>
            <div class="container mb-3">
                <div class="row gx-3 gy-4">
                
                    <div class="col-6">
                        <SourceColorChooser color={source_color.clone()} />
                    </div>
                    
                    <div class="col-6">
                        <BooleanChooser title={"Size options"} true_text={"Maximized"} false_text={"Minimized"} bool_state={is_maximized.clone()} />
                    </div>

                    <PredictionsCarousel color={*source_color} is_maximized={*is_maximized} {carousel_state} />

                    <div>
                        <h5 class="text-center mb-2">{ "Link to source" }</h5>
                        // can't not have the generic, unfortunately
                        <StaticSourceLink<PredictionsSourceOptions> options={collected_options.clone()} source_name={"predictions"} skin={chosen_skin.to_string()} />
                    </div>
                    <div>
                        <h5 class="text-center mb-2">{ "User-specific link (better to use in OBS)" }</h5>
                        { 
                            login_gate(html!{
                                <LoginSourceLink<PredictionsSourceOptions> options={collected_options.clone()} source_name={"predictions"} skin={chosen_skin.to_string()} />
                            })
                        }
                    </div>
                    
                    <div class="d-flex flex-column justify-content-center align-items-center">
                        <div>
                            <h5>{"Start a test event"}</h5>
                        </div>
                        <div>
                            { login_gate(html!{ <TestButton test_name={"predictions"} timeout_secs={13} /> }) }
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}