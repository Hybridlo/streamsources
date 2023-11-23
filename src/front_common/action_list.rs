use std::{rc::Rc, cell::RefCell, collections::VecDeque};

use gloo_timers::callback::Timeout;
use yew::use_mut_ref;

#[derive(Clone)]
pub struct ActionList {
    actions: Rc<RefCell<VecDeque<(Box<dyn FnOnce() -> u32>, Box<dyn FnOnce()>)>>>,
    action_handle: Rc<RefCell<Option<Timeout>>>
}

impl ActionList {
    fn new() -> Self {
        let actions = use_mut_ref(Default::default);
        let action_handle = use_mut_ref(|| None);

        Self {
            actions,
            action_handle,
        }
    }

    fn perform_next_action(&self) {
        let action_list = self.clone();
        if let Some(action) = self.actions.borrow_mut().pop_front() {
            self.action_handle.replace(Some(Timeout::new((action.0)(), move || {
                (action.1)();
                action_list.perform_next_action();
            })));
        }
    }

    pub fn start(&self, actions: Vec<(Box<dyn FnOnce() -> u32>, Box<dyn FnOnce()>)>) {
        self.stop();

        self.actions.replace(actions.into());

        self.perform_next_action();
    }

    pub fn stop(&self) {
        if let Some(action_handle) = self.action_handle.take() {
            action_handle.cancel();
        }
    }
}

pub fn use_action_list() -> ActionList {
    ActionList::new()
}
