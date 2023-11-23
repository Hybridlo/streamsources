use std::{rc::Rc, cell::RefCell};

use chrono::{DateTime, Utc};
use gloo_timers::callback::Interval;
use web_sys::console::log_1;
use yew::{Properties, function_component, use_mut_ref, use_state, html};

#[derive(PartialEq, Properties)]
pub struct CountdownTimerProps {
    pub target_time: Option<DateTime<Utc>>
}

fn get_time_left_seconds(target_time: Option<DateTime<Utc>>) -> Option<i64> {
    let time_now = Utc::now();

    target_time.map(|target_time| (target_time - time_now).num_seconds().max(0))
}

fn get_timer_string(time_left: Option<i64>) -> String {
    if let Some(time_left) = time_left {
        let minutes = time_left.checked_div(60).unwrap_or_default();
        let seconds = time_left.checked_rem(60).unwrap_or_default();
    
        format!("{minutes:0>2}:{seconds:0>2}")
    } else {
        "--:--".to_string()
    }
}

#[function_component(CountdownTimer)]
pub fn countdown_timer(props: &CountdownTimerProps) -> Html {
    let timer_animation: Rc<RefCell<Option<Interval>>> = use_mut_ref(|| None);
    let timer = use_state(|| get_timer_string(get_time_left_seconds(props.target_time)));
    log_1(&format!("{:?}", props.target_time).into());

    let timer_not_running = timer_animation.borrow().is_none();

    if timer_not_running {
        let target_time = props.target_time;
        let timer = timer.clone();
        let timer_animation_clone = timer_animation.clone();
        timer_animation.replace(Some(Interval::new(1000, move || {
            let time_left = get_time_left_seconds(target_time);
            timer.set(get_timer_string(time_left));

            if time_left.map_or(true, |time_left| time_left <= 0) {
                if let Some(timer_animation) = timer_animation_clone.take() {
                    timer_animation.cancel();
                }
            }
        })));
    }

    html!{ { &*timer } }
}