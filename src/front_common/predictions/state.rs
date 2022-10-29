use std::cell::RefCell;
use std::rc::Rc;
use chrono::{DateTime, Utc};

use yew::UseStateSetter;
use gloo_timers::callback::Interval;

use crate::{FPS, GLOBAL_DELAY_VALUE, GLOBAL_DELAY_VALUE_SECONDS};
use super::super::transition_funcs::ease_in_out_formula;

#[derive(Clone, Debug, PartialEq)]
pub struct UserPredictionState {
    pub user_name: String,
    pub channel_points_used: i64
}

#[derive(Clone, Debug, PartialEq)]
pub struct PredictionOutcomeState {
    pub id: String,
    pub title: String,
    pub color: String,
    pub users: i64,
    pub channel_points: i64,
    pub top_predictors: Vec<UserPredictionState>
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum PreditionStatus {
    InProgress,
    Locked,
    #[default]
    Finished
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct PredictionState {
    pub id: String,
    pub title: String,
    pub winning_outcome_id: Option<String>,
    pub outcomes: Vec<PredictionOutcomeState>,
    pub lock_time: DateTime<Utc>,
    pub status: PreditionStatus,
    pub show_element: bool,
    pub show_status: bool
}

impl PredictionState {
    pub fn clone_with_empty_outcomes(&self) -> Self {
        return PredictionState {
            id: self.id.clone(),
            title: self.title.clone(),
            winning_outcome_id: self.winning_outcome_id.clone(),
            outcomes: Default::default(),
            lock_time: self.lock_time.clone(),
            status: self.status.clone(),
            show_element: self.show_element,
            show_status: self.show_status
        }
    }

    pub fn get_outcome_percents(&self, outcome_id: &str) -> i64 {
        let sum = self.outcomes.iter().fold(0, |curr, val| curr + val.channel_points);

        // outcome_id should always come from the vec itself, otherwise something wrong is going on
        let outcome = self.outcomes.iter().find(|v| v.id == outcome_id).unwrap();

        return (((outcome.channel_points as f64) / (sum as f64)) * 100.0).round() as _;
    }
}

pub struct PredictionStateAnimator {
    prev_state: PredictionState,
    next_state: PredictionState,

    pub state_setter: UseStateSetter<PredictionState>,

    timer: Rc<RefCell<f64>>,
    animation_handle: Rc<RefCell<Option<Interval>>>
}

impl PredictionStateAnimator {
    pub fn new(state_setter: UseStateSetter<PredictionState>, curr_state: &PredictionState) -> Self {
        Self {
            prev_state: curr_state.clone(),
            next_state: Default::default(),
            state_setter,
            timer: Default::default(),
            animation_handle: Default::default()
        }
    }

    pub fn set_state(&mut self, new_state: PredictionState, curr_state: &PredictionState) {
        self.prev_state = curr_state.clone();
        self.next_state = new_state;
        self.next_state.outcomes.sort_by(|a, b| b.channel_points.cmp(&a.channel_points));

        *(self.timer.borrow_mut()) = 0.0;

        // TODO: handle prediction status change

        if let Some(anim_handle) = self.animation_handle.take() {
            anim_handle.cancel();
        };

        {
            let state_setter = self.state_setter.clone();
            let timer_ref = self.timer.clone();
            let anim_handle_ref = self.animation_handle.clone();
            let prev_state = self.prev_state.clone();
            let next_state = self.next_state.clone();

            self.animation_handle.replace(Some(Interval::new(GLOBAL_DELAY_VALUE / FPS, move || {
                let mut timer_borrow = timer_ref.borrow_mut();
                *timer_borrow += (GLOBAL_DELAY_VALUE_SECONDS as f64) / (FPS as f64);

                if *timer_borrow > (GLOBAL_DELAY_VALUE_SECONDS as f64) {
                    state_setter.set(next_state.clone());
                    if let Some(anim_handle) = anim_handle_ref.take() {
                        anim_handle.cancel();
                    };
                    return;
                }

                let mut intermediate_state = next_state.clone_with_empty_outcomes();

                for outcome in next_state.outcomes.iter() {
                    // outcomes can never be different; if a new event is fired, there must be a new animator
                    let prev_outcome = prev_state.outcomes.iter().find(
                        |&elem| elem.id == outcome.id
                    ).unwrap();

                    let mut intermediate_outcome = PredictionOutcomeState {
                        id: outcome.id.clone(),
                        title: outcome.title.clone(),
                        color: outcome.color.clone(),
                        channel_points: ease_in_out_formula(
                            *timer_borrow,
                            prev_outcome.channel_points as _,
                            (outcome.channel_points - prev_outcome.channel_points) as _,
                            GLOBAL_DELAY_VALUE_SECONDS as _
                        ) as _,
                        users: ease_in_out_formula(
                            *timer_borrow,
                            prev_outcome.users as _, 
                            (outcome.users - prev_outcome.users) as _, 
                            GLOBAL_DELAY_VALUE_SECONDS as _
                        ) as _,
                        top_predictors: vec![]
                    };

                    for predictor in outcome.top_predictors.iter() {
                        let prev_predictor = prev_outcome.top_predictors.iter().find(
                            |&elem| elem.user_name == predictor.user_name
                        );

                        let prev_predictor = {
                            if let Some(actual_prev_predictor) = prev_predictor {
                                actual_prev_predictor.clone()
                            } else {
                                UserPredictionState {
                                    user_name: predictor.user_name.clone(),
                                    channel_points_used: 0
                                }
                            }
                        };

                        let intermediate_predictor_state = UserPredictionState {
                            user_name: predictor.user_name.clone(),
                            channel_points_used: ease_in_out_formula(
                                *timer_borrow,
                                prev_predictor.channel_points_used as _,
                                (predictor.channel_points_used - prev_predictor.channel_points_used) as _,
                                GLOBAL_DELAY_VALUE_SECONDS as _
                            ) as _
                        };

                        intermediate_outcome.top_predictors.push(intermediate_predictor_state);
                    }

                    intermediate_state.outcomes.push(intermediate_outcome);
                }

                state_setter.set(intermediate_state);
            })));
        }
    }
}