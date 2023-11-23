use std::cell::RefCell;
use std::rc::Rc;
use chrono::{DateTime, Utc};

use yew::UseStateSetter;
use gloo_timers::callback::{Interval, Timeout};

use crate::{FPS, GLOBAL_DELAY_VALUE, GLOBAL_DELAY_VALUE_SECONDS, common_data::eventsub_msgs::{EventSubData, EventSubMessage}};
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
    pub top_predictors: Vec<UserPredictionState>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum PredictionStatus {
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
    pub status: PredictionStatus,
}

impl PredictionState {
    pub fn new(
        id: String,
        title: String,
        winning_outcome_id: Option<String>,
        outcomes: Vec<PredictionOutcomeState>,
        lock_time: DateTime<Utc>,
        status: PredictionStatus
    ) -> Self {
        PredictionState {
            id,
            title,
            winning_outcome_id,
            outcomes,
            lock_time,
            status,
        }
    }

    pub fn clone_with_empty_outcomes(&self) -> Self {
        return PredictionState {
            id: self.id.clone(),
            title: self.title.clone(),
            winning_outcome_id: self.winning_outcome_id.clone(),
            outcomes: Default::default(),
            lock_time: self.lock_time.clone(),
            status: self.status.clone(),
        }
    }

    pub fn get_outcome_percents(&self, outcome_id: &str) -> i64 {
        let sum = self.outcomes.iter().fold(0, |curr, val| curr + val.channel_points);

        // outcome_id should always come from the vec itself, otherwise something wrong is going on
        let outcome = self.outcomes.iter().find(|v| v.id == outcome_id).unwrap();

        return (((outcome.channel_points as f64) / (sum as f64)) * 100.0).round() as _;
    }

    pub fn get_winner_idx(&self) -> Option<usize> {
        if let Some(outcome_id) = &self.winning_outcome_id {
            return self.outcomes.iter().position(|item| &item.id == outcome_id)
        }

        return None;
    }
}

impl TryFrom<EventSubMessage> for PredictionState {
    type Error = anyhow::Error;

    fn try_from(value: EventSubMessage) -> Result<Self, Self::Error> {
        match value.data {
            EventSubData::ChannelPredictionBegin(data) => Ok(Self { 
                id: data.id,
                title: data.title,
                winning_outcome_id: None,
                outcomes: data.outcomes.into_iter().map(|outcome| PredictionOutcomeState {
                    id: outcome.id,
                    title: outcome.title,
                    color: outcome.color,
                    users: outcome.users,
                    channel_points: outcome.channel_points,
                    top_predictors: outcome.top_predictors.into_iter().map(|predictor| UserPredictionState {
                        user_name: predictor.user_name,
                        channel_points_used: predictor.channel_points_used,
                    }).collect(),
                }).collect(),
                lock_time: DateTime::parse_from_rfc3339(&data.locks_at)?.with_timezone(&Utc),
                status: PredictionStatus::InProgress
            }),
            EventSubData::ChannelPredictionProgress(data) => Ok(Self { 
                id: data.id,
                title: data.title,
                winning_outcome_id: None,
                outcomes: data.outcomes.into_iter().map(|outcome| PredictionOutcomeState {
                    id: outcome.id,
                    title: outcome.title,
                    color: outcome.color,
                    users: outcome.users,
                    channel_points: outcome.channel_points,
                    top_predictors: outcome.top_predictors.into_iter().map(|predictor| UserPredictionState {
                        user_name: predictor.user_name,
                        channel_points_used: predictor.channel_points_used,
                    }).collect(),
                }).collect(),
                lock_time: DateTime::parse_from_rfc3339(&data.locks_at)?.with_timezone(&Utc),
                status: PredictionStatus::InProgress
            }),
            EventSubData::ChannelPredictionLock(data) => Ok(Self { 
                id: data.id,
                title: data.title,
                winning_outcome_id: None,
                outcomes: data.outcomes.into_iter().map(|outcome| PredictionOutcomeState {
                    id: outcome.id,
                    title: outcome.title,
                    color: outcome.color,
                    users: outcome.users,
                    channel_points: outcome.channel_points,
                    top_predictors: outcome.top_predictors.into_iter().map(|predictor| UserPredictionState {
                        user_name: predictor.user_name,
                        channel_points_used: predictor.channel_points_used,
                    }).collect(),
                }).collect(),
                lock_time: DateTime::parse_from_rfc3339(&data.locked_at)?.with_timezone(&Utc),
                status: PredictionStatus::Locked
            }),
            EventSubData::ChannelPredictionEnd(data) => Ok(Self { 
                id: data.id,
                title: data.title,
                winning_outcome_id: data.winning_outcome_id,
                outcomes: data.outcomes.into_iter().map(|outcome| PredictionOutcomeState {
                    id: outcome.id,
                    title: outcome.title,
                    color: outcome.color,
                    users: outcome.users,
                    channel_points: outcome.channel_points,
                    top_predictors: outcome.top_predictors.into_iter().map(|predictor| UserPredictionState {
                        user_name: predictor.user_name,
                        channel_points_used: predictor.channel_points_used,
                    }).collect(),
                }).collect(),
                lock_time: DateTime::parse_from_rfc3339(&data.ended_at)?.with_timezone(&Utc),
                status: PredictionStatus::Finished
            }),
            _ => Err(anyhow::anyhow!("Could not understand the message, please report this error")),
        }
    }
}


pub struct PredictionStateAnimator {
    prev_state: PredictionState,
    next_state: PredictionState,

    pub state_setter: UseStateSetter<PredictionState>,
    pub show_element_setter: UseStateSetter<bool>,
    pub show_status_setter: UseStateSetter<bool>,
    pub status_setter: UseStateSetter<PredictionStatus>,

    timer: Rc<RefCell<f64>>,

    animation_handle: Rc<RefCell<Option<Interval>>>,
    hide_element_handle: Rc<RefCell<Option<Timeout>>>,
    show_status_handle: Rc<RefCell<Option<Timeout>>>
}

impl PredictionStateAnimator {
    pub fn new(
        state_setter: UseStateSetter<PredictionState>,
        curr_state: &PredictionState,
        show_element_setter: UseStateSetter<bool>,
        show_status_setter: UseStateSetter<bool>,
        status_setter: UseStateSetter<PredictionStatus>,
    ) -> Self {
        Self {
            prev_state: curr_state.clone(),
            next_state: Default::default(),
            state_setter,
            show_element_setter,
            show_status_setter,
            status_setter,
            timer: Default::default(),
            animation_handle: Default::default(),
            hide_element_handle: Default::default(),
            show_status_handle: Default::default()
        }
    }

    // we can't rely on UseState value, because it doesn't update without use_state func
    // so we need curr_state from the called, that uses use_state
    pub fn set_state(&mut self, new_state: PredictionState, curr_state: &PredictionState) {
        // we got a new prediction, so gotta adjust for that
        // just copying target state, because there's no
        // previous state to animate from
        if new_state.id != curr_state.id {
            self.prev_state = new_state.clone();
            // but i can't leave prediction status the same... maybe there's a more right way to this
            self.prev_state.status = curr_state.status.clone();
        } else {
            self.prev_state = curr_state.clone();
        }

        self.next_state = new_state;
        self.next_state.outcomes.sort_by(|a, b| b.channel_points.cmp(&a.channel_points));

        *(self.timer.borrow_mut()) = 0.0;

        // TODO: handle prediction status change

        if let Some(anim_handle) = self.animation_handle.take() {
            anim_handle.cancel();
        };

        {         
            // clear this timeout and show the element, since we've got a new state   
            if let Some(element_handle) = self.hide_element_handle.take() {
                element_handle.cancel();
            }

            self.show_element_setter.set(true);

            // neatly transition react to status change
            if self.prev_state.status != self.next_state.status {
                if let Some(status_handle) = self.show_status_handle.take() {
                    status_handle.cancel();
                }

                self.show_status_setter.set(false);

                let show_status_setter = self.show_status_setter.clone();
                let status_setter = self.status_setter.clone();
                let next_status = self.next_state.status.clone();

                self.show_status_handle.replace(Some(Timeout::new(500, move || {
                    show_status_setter.set(true);
                    status_setter.set(next_status);
                })));
            }

            // gotta hide the element when prediction is finished, but only after a while
            if self.next_state.status == PredictionStatus::Finished {
                let show_element_setter = self.show_element_setter.clone();

                self.hide_element_handle.replace(Some(Timeout::new(10_000, move || {
                    show_element_setter.set(false);
                })));

                // also get the winner up top, make other options black(? we have other colors tho too, maybe can be improved)
                if let Some(win_idx) = self.next_state.get_winner_idx() {
                    self.next_state.outcomes.as_mut_slice()[0..win_idx].rotate_right(1);

                    self.next_state.outcomes.iter_mut()
                        .enumerate()
                        .for_each(|(i, item)| if i != 0 { item.color = "black".to_string() } )
                } else {
                    self.next_state.outcomes.iter_mut()
                        .for_each(|item| item.color = "black".to_string() )
                }
            }

            // and this is the part that manipulates stuff to animate from one value to new one
            let state_setter = self.state_setter.clone();
            let timer_ref = self.timer.clone();
            let anim_handle_ref = self.animation_handle.clone();
            let prev_state = self.prev_state.clone();
            let next_state = self.next_state.clone();

            self.animation_handle.replace(Some(Interval::new(GLOBAL_DELAY_VALUE / FPS, move || {
                let mut timer_borrow = timer_ref.borrow_mut();
                *timer_borrow += (GLOBAL_DELAY_VALUE_SECONDS as f64) / (FPS as f64);

                // if time over the animation time - we're done
                if *timer_borrow > (GLOBAL_DELAY_VALUE_SECONDS as f64) {
                    state_setter.set(next_state.clone());
                    if let Some(anim_handle) = anim_handle_ref.take() {
                        anim_handle.cancel();
                    };
                    return;
                }

                // intermediate step will take everything from the next step, except stuff that
                // has points in it, which is animated, calculated from the time, prev_state and next_state
                let mut intermediate_state = next_state.clone_with_empty_outcomes();

                for outcome in next_state.outcomes.iter() {
                    // outcomes will never be different; we're taking care of that before
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