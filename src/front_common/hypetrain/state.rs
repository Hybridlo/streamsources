use chrono::{DateTime, Utc, ParseError};
use thiserror::Error;

use crate::common_data::eventsub_msgs::{ContributionType, EventSubMessage, EventSubData, SubType, Contribution};

#[derive(Clone, Debug, PartialEq)]
pub struct HypetrainContributionState {
    pub user_name: String,
    pub type_: ContributionType,
    pub total: u64
}

impl HypetrainContributionState {
    pub fn contrib_amount(&self) -> u64 {
        match self.type_ {
            ContributionType::Bits => self.total,
            ContributionType::Subscription => self.total / 500,
            ContributionType::Other => self.total,
        }
    }
}

impl From<Contribution> for HypetrainContributionState {
    fn from(contribution: Contribution) -> Self {
        Self {
            user_name: contribution.user_name,
            type_: contribution.type_,
            total: contribution.total,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HypetrainState {
    pub id: String,
    pub curr_level: u64,
    pub curr_points: u64,
    pub needed_points: u64,
    pub top_gifter: Option<HypetrainContributionState>,
    pub top_bit_donator: Option<HypetrainContributionState>,
    pub last_contributions: Vec<HypetrainContributionState>,
    pub ends_at: DateTime<Utc>,
    pub cooldown_ends_at: Option<DateTime<Utc>>,
}

impl Default for HypetrainState {
    fn default() -> Self {
        Self {
            id: Default::default(),
            curr_level: 0,
            curr_points: Default::default(),
            needed_points: 1,
            top_gifter: Default::default(),
            top_bit_donator: Default::default(),
            last_contributions: Default::default(),
            ends_at: Default::default(),
            cooldown_ends_at: Default::default(),
        }
    }
}

impl HypetrainState {
    pub fn new(
        id: String,
        curr_level: u64,
        curr_points: u64,
        needed_points: u64,
        top_gifter: Option<HypetrainContributionState>,
        top_bit_donator: Option<HypetrainContributionState>,
        last_contributions: Vec<HypetrainContributionState>,
        ends_at: DateTime<Utc>,
        cooldown_ends_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            curr_level,
            curr_points,
            needed_points,
            top_gifter,
            top_bit_donator,
            last_contributions,
            ends_at,
            cooldown_ends_at,
        }
    }
}

impl TryFrom<EventSubMessage> for HypetrainState {
    type Error = HypetrainFromEventSubMessageError;

    fn try_from(value: EventSubMessage) -> Result<Self, Self::Error> {
        match value.data {
            EventSubData::HypeTrainBegin(data) => Ok(Self {
                id: data.data.id,
                curr_level: data.data.level,
                curr_points: data.data.progress,
                needed_points: data.data.goal,
                top_gifter: data.data.top_contributions
                    .iter()
                    .find(|contribution| contribution.type_ == ContributionType::Subscription)
                    .cloned()
                    .map(Into::into),
                top_bit_donator: data.data.top_contributions
                    .iter()
                    .find(|contribution| contribution.type_ == ContributionType::Bits)
                    .cloned()
                    .map(Into::into),
                last_contributions: vec![data.data.last_contribution.into()],
                ends_at: DateTime::parse_from_rfc3339(&data.expires_at)?.with_timezone(&Utc),
                cooldown_ends_at: None,
            }),
            EventSubData::HypeTrainProgress(data) => Ok(Self {
                id: data.data.id,
                curr_level: data.data.level,
                curr_points: data.data.progress,
                needed_points: data.data.goal,
                top_gifter: data.data.top_contributions
                    .iter()
                    .find(|contribution| contribution.type_ == ContributionType::Subscription)
                    .cloned()
                    .map(Into::into),
                top_bit_donator: data.data.top_contributions
                    .iter()
                    .find(|contribution| contribution.type_ == ContributionType::Bits)
                    .cloned()
                    .map(Into::into),
                last_contributions: vec![data.data.last_contribution.into()],
                ends_at: DateTime::parse_from_rfc3339(&data.expires_at)?.with_timezone(&Utc),
                cooldown_ends_at: None,
            }),
            EventSubData::HypeTrainEnd(data) => Ok(Self {
                id: data.data.id,
                curr_level: data.data.level,
                curr_points: data.data.progress,
                needed_points: data.data.goal,
                top_gifter: data.data.top_contributions
                    .iter()
                    .find(|contribution| contribution.type_ == ContributionType::Subscription)
                    .cloned()
                    .map(Into::into),
                top_bit_donator: data.data.top_contributions
                    .iter()
                    .find(|contribution| contribution.type_ == ContributionType::Bits)
                    .cloned()
                    .map(Into::into),
                last_contributions: vec![data.data.last_contribution.into()],
                ends_at: DateTime::parse_from_rfc3339(&data.ended_at)?.with_timezone(&Utc),
                cooldown_ends_at: Some(DateTime::parse_from_rfc3339(&data.cooldown_ends_at)?.with_timezone(&Utc)),
            }),
            wrong_sub_data => Err(HypetrainFromEventSubMessageError::WrongTypeSubData(wrong_sub_data.sub_type()))
        }
    }
}

#[derive(PartialEq)]
pub enum HypetrainStatus {
    Hypetrain,
    Finished,
    Cooldown,
    Ready
}

/* pub struct HypetrainStateAnimator {
    prev_state: HypetrainState,
    next_state: HypetrainState,

    pub state_setter: UseStateHandle<HypetrainState>,
    pub show_level: UseStateHandle<bool>,
    pub curr_level_setter: UseStateHandle<u64>,
    pub show_top_gifter: UseStateHandle<bool>,
    pub show_top_bit_donator: UseStateHandle<bool>,

    timer: Rc<RefCell<f64>>,

    show_level_handle: Rc<RefCell<Option<Timeout>>>,
    show_top_gifter_handle: Rc<RefCell<Option<Timeout>>>,
    show_top_bit_donator_handle: Rc<RefCell<Option<Timeout>>>,
}

impl HypetrainStateAnimator {
    pub fn new(
        state_setter: UseStateHandle<HypetrainState>,
        curr_state: &HypetrainState,
        show_level: UseStateHandle<bool>,
        curr_level_setter: UseStateHandle<u64>,
        show_top_gifter: UseStateHandle<bool>,
        show_top_bit_donator: UseStateHandle<bool>,
    ) -> Self {
        Self {
            prev_state: curr_state.clone(),
            next_state: Default::default(),
            state_setter,
            show_level,
            curr_level_setter,
            show_top_gifter,
            show_top_bit_donator,
            timer: Default::default(),
            show_level_handle: Default::default(),
            show_top_gifter_handle: Default::default(),
            show_top_bit_donator_handle: Default::default(),
        }
    }

    pub fn update_state(&mut self, new_state: HypetrainState, curr_state: &HypetrainState) {
        if new_state.id != curr_state.id {
            self.prev_state = new_state.clone();

            // If current progress state is greater than that of a new state - the new state is old,
            // and should be ignored.
            //
            // Because level requirements never decrease - this formula is sufficient enough.
        } else if curr_state.curr_level * curr_state.needed_points + curr_state.curr_points
                > new_state.curr_level * new_state.needed_points + new_state.curr_points {
            return;
        }

        self.next_state = new_state;

        if curr_state.curr_level < self.next_state.curr_level {
            self.update_level(self.next_state.curr_level);
        }

        let top_gifter_newly_set_or_changed = self.next_state.top_gifter
            .is_some_and(|top_gifter| {
                // checks if a top gifter changed or if there were no top gifters before
                //          effectively - .is_none_or
                curr_state.top_gifter.map_or(
                    true,
                    |curr_top_gifter| curr_top_gifter.user_name != top_gifter.user_name
                )
            });

        let top_bit_donator_newly_set_or_changed = self.next_state.top_bit_donator
            .is_some_and(|top_bit_donator| {
                // checks if a top bit_donator changed or if there were no top bit_donators before
                //               effectively - .is_none_or
                curr_state.top_bit_donator.map_or(
                    true,
                    |curr_top_bit_donator| curr_top_bit_donator.user_name != top_bit_donator.user_name
                )
            });

        let intermediate_state = curr_state.clone();
        
        if top_gifter_newly_set_or_changed {
            if let Some(show_top_gifter_handle) = self.show_top_gifter_handle.take() {
                show_top_gifter_handle.cancel();
            }

            self.show_top_gifter.set(false);
            
            let show_top_gifter = self.show_top_gifter.clone();
            self.show_top_gifter_handle.replace(Some(Timeout::new(500, move || {
                show_top_gifter.set(true)
            })))
        }
    }

    pub fn update_level(&mut self, next_level: u64) {
        if let Some(show_level_handle) = self.show_level_handle.take() {
            show_level_handle.cancel();
        }

        self.show_level.set(false);

        let show_level = self.show_level.clone();
        let curr_level_setter = self.curr_level_setter.clone();
        self.show_level_handle.replace(Some(Timeout::new(500, move || {
            show_level.set(true);
            curr_level_setter.set(next_level);
        })));
    }
} */

#[derive(Debug, Error)]
pub enum HypetrainFromEventSubMessageError {
    #[error("Wrong type of sub data received: {0}")]
    WrongTypeSubData(SubType),
    #[error("Expiration time parsing failed: {0}")]
    ParseError(#[from] ParseError),
}