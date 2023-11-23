use std::{cell::RefCell, rc::Rc, ops::Deref};

use gloo_timers::callback::Interval;
use yew::{UseStateSetter, UseStateHandle, use_mut_ref, use_state_eq};

use crate::FPS;

pub struct AnimatedF64<F> {
    prev_state: f64,
    next_state: f64,

    pub state_setter: UseStateSetter<f64>,

    transition_time: f64,
    curr_time: Rc<RefCell<f64>>,
    interpolation_func: F,

    animation_handle: Rc<RefCell<Option<Interval>>>,
}

impl<F> AnimatedF64<F>
where
    F: Fn(f64, f64, f64, f64) -> f64 + 'static + Copy
{
    fn new(transition_time: f64, interpolation_func: F, curr_state: UseStateSetter<f64>) -> Self {
        Self {
            prev_state: 0.0,
            next_state: 0.0,
            state_setter: curr_state,
            transition_time,
            curr_time: Default::default(),
            interpolation_func,
            animation_handle: Default::default(),
        }
    }

    fn set_value(&mut self, curr_value: f64, new_value: f64) {
        *(self.curr_time.borrow_mut()) = 0.0;
        self.prev_state = curr_value;
        self.next_state = new_value;
        if let Some(animation_handle) = self.animation_handle.take() {
            animation_handle.cancel();
        }

        let curr_state = self.state_setter.clone();
        let curr_time = self.curr_time.clone();
        let prev_state = self.prev_state;
        let next_state = self.next_state;
        let animation_handle = self.animation_handle.clone();
        let interpolation_func = self.interpolation_func;
        let transition_time = self.transition_time;

        self.animation_handle.replace(Some(Interval::new(
            (transition_time * 1000.0 / (FPS as f64)) as _,
            move || {
                let mut time_borrow = curr_time.borrow_mut();
                *time_borrow += transition_time / (FPS as f64);

                // if time over the animation time - we're done
                if *time_borrow > transition_time {
                    curr_state.set(next_state);
                    if let Some(animation_handle) = animation_handle.take() {
                        animation_handle.cancel();
                    };
                    return;
                }
                
                curr_state.set(interpolation_func(
                    *time_borrow,
                    prev_state,
                    next_state - prev_state,
                    transition_time
                ));
            }
        )));
    }

    fn immediate_set_value(&mut self, new_value: f64) {
        if let Some(animation_handle) = self.animation_handle.take() {
            animation_handle.cancel();
        }

        self.state_setter.set(new_value);
        self.prev_state = new_value;
        self.next_state = new_value;
    }
}

pub struct AnimatedF64Handle<F> {
    state: UseStateHandle<f64>,
    animator: Rc<RefCell<AnimatedF64<F>>>
}

impl<F> AnimatedF64Handle<F>
where
    F: Fn(f64, f64, f64, f64) -> f64 + 'static + Copy
{
    fn new(transition_time: f64, interpolation_func: F) -> Self {
        let state = use_state_eq(|| 0.0);
        let animator = use_mut_ref(|| AnimatedF64::new(transition_time, interpolation_func, state.setter()));
        Self {
            state,
            animator,
        }
    }

    pub fn set_value(&self, new_value: f64) {
        self.animator.borrow_mut().set_value(*self.state, new_value);
    }

    pub fn immediate_set_value(&self, new_value: f64) {
        self.animator.borrow_mut().immediate_set_value(new_value);
    }
}

impl<F> Deref for AnimatedF64Handle<F> {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &*self.state
    }
}

impl<F> Clone for AnimatedF64Handle<F> {
    fn clone(&self) -> Self {
        Self { state: self.state.clone(), animator: self.animator.clone() }
    }
}

pub fn use_animated_f64<F>(transition_time: f64, interpolation_func: F) -> AnimatedF64Handle<F>
where
    F: Fn(f64, f64, f64, f64) -> f64 + 'static + Copy
{
    AnimatedF64Handle::new(transition_time, interpolation_func)
}
