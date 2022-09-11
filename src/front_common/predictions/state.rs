use std::cell::RefCell;
use std::rc::Rc;

use yew::UseStateHandle;
use gloo_timers::callback::Interval;

use crate::{FPS, GLOBAL_DELAY_VALUE, GLOBAL_DELAY_VALUE_SECONDS};
use super::super::transition_funcs::ease_in_out_formula;

#[derive(Clone, Debug)]
pub struct UserPredictionState {
    pub user_name: String,
    pub channel_points_used: i64
}

#[derive(Clone, Debug)]
pub struct PredictionOutcomeState {
    pub id: String,
    pub title: String,
    pub color: String,
    pub users: i64,
    pub channel_points: i64,
    pub top_predictors: Vec<UserPredictionState>
}

#[derive(Clone, Default, Debug)]
pub struct PredictionState {
    pub id: String,
    pub title: String,
    pub winning_outcome_id: Option<String>,
    pub outcomes: Vec<PredictionOutcomeState>
}

impl PredictionState {
    pub fn new() -> Self {
        return Default::default();
    }
}

pub struct PredictionStateAnimator {
    prev_state: PredictionState,
    next_state: PredictionState,

    pub state: UseStateHandle<PredictionState>,

    timer: Rc<RefCell<f64>>,
    animation_handle: Rc<RefCell<Option<Interval>>>
}

impl PredictionStateAnimator {
    pub fn new(state_handle: &UseStateHandle<PredictionState>) -> Self {
        Self {
            prev_state: (**state_handle).clone(),
            next_state: Default::default(),
            state: state_handle.clone(),
            timer: Default::default(),
            animation_handle: Default::default()
        }
    }

    pub fn set_state(&mut self, new_state: PredictionState) {
        self.next_state = new_state;

        *(self.timer.borrow_mut()) = 0.0;

        if let Some(anim_handle) = self.animation_handle.take() {
            anim_handle.cancel();
        };

        {
            let state_ref = self.state.clone();
            let timer_ref = self.timer.clone();
            let anim_handle_ref = self.animation_handle.clone();
            let prev_state = self.prev_state.clone();
            let next_state = self.next_state.clone();

            self.animation_handle = Rc::new(RefCell::new(Some(Interval::new(GLOBAL_DELAY_VALUE / FPS, move || {
                let mut timer_borrow = timer_ref.borrow_mut();
                *timer_borrow += (GLOBAL_DELAY_VALUE_SECONDS as f64) / (FPS as f64);

                if *timer_borrow > (GLOBAL_DELAY_VALUE_SECONDS as f64) {
                    state_ref.set(next_state.clone());
                    if let Some(anim_handle) = anim_handle_ref.take() {
                        anim_handle.cancel();
                    };
                    return;
                }

                let mut intermediate_state = PredictionState::new();

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

                state_ref.set(intermediate_state);
            }))));
        }
    }
}