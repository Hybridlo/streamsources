use twitch_sources_rework::{front_common::{hypetrain::*, SourceColor}, common_data::eventsub_msgs::ContributionType, enclose};

use strum::{EnumIter, Display, IntoEnumIterator};
use twitch_sources_rework::front_common::hypetrain::HypetrainState;
use yew::{UseStateHandle, html, Html, use_state, function_component, Properties, use_state_eq, use_effect_with_deps};

use crate::components::widgets::Carousel;
use crate::components::widgets::TestButton;
use crate::components::widgets::source_settings::Checkbox;
use crate::components::widgets::source_settings::NumberInput;
use crate::components::widgets::source_settings::SourceColorChooser;
use crate::components::widgets::source_settings::StaticSourceLink;
use crate::components::widgets::source_settings::LoginSourceLink;
use crate::util::login_gate;

#[derive(Default, Clone, PartialEq, Debug, EnumIter, Display)]
#[strum(serialize_all="snake_case")]
pub enum HypetrainSkins {
    #[default]
    Normal
}

impl HypetrainSkins {
    fn to_html(
        &self,
        state: UseStateHandle<HypetrainState>,
        settings: HypetrainSourceOptions,
    ) -> Html {
        match self {
            HypetrainSkins::Normal => html! {
                <components::HypetrainNormal {settings} last_message={state} />
            },
        }
    }
}


#[derive(Properties, PartialEq)]
struct HypetrainCarouselProps {
    settings: HypetrainSourceOptions,
    carousel_state: UseStateHandle<usize>
}

#[function_component(HypetrainCarousel)]
fn hypetrain_carousel(props: &HypetrainCarouselProps) -> Html {
    let source_state = use_state(|| HypetrainState {
        id: "".to_string(),
        curr_level: 3,
        curr_points: 3000,
        needed_points: 5000,
        top_gifter: Some(HypetrainContributionState {
            user_name: "User1".to_string(),
            type_: ContributionType::Subscription,
            total: 5000,
        }),
        top_bit_donator: Some(HypetrainContributionState {
            user_name: "User2".to_string(),
            type_: ContributionType::Bits,
            total: 3000,
        }),
        // Not intended to have more than 1 in here, but it works
        last_contributions: vec![
            HypetrainContributionState {
                user_name: "User3".to_string(),
                type_: ContributionType::Bits,
                total: 100,
            },
            HypetrainContributionState {
                user_name: "User4".to_string(),
                type_: ContributionType::Subscription,
                total: 500,
            },
            HypetrainContributionState {
                user_name: "User5".to_string(),
                type_: ContributionType::Subscription,
                total: 2500,
            },
            HypetrainContributionState {
                user_name: "User6".to_string(),
                type_: ContributionType::Other,
                total: 1000,
            },
            HypetrainContributionState {
                user_name: "User7".to_string(),
                type_: ContributionType::Subscription,
                total: 500,
            },
        ],
        ends_at: Default::default(),
        cooldown_ends_at: Default::default(),
    });

    let skins: Vec<Html> = HypetrainSkins::iter().map(|skin| skin.to_html(
        source_state.clone(),
        props.settings,
    )).collect();

    html! {
        <Carousel active_item={props.carousel_state.clone()} carousel_size={3} items={skins} height={300} />
    }
}

#[function_component(HypetrainSettings)]
pub fn hypetrain_settings() -> Html {
    let source_color = use_state_eq(|| SourceColor::default());
    let max_last_contributions = use_state_eq(|| 1i64);
    let show_cooldown = use_state_eq(|| true);
    let show_ready = use_state_eq(|| false);

    let carousel_state = use_state_eq(|| 0);
    let chosen_skin = HypetrainSkins::iter().get(*carousel_state).expect("No way carousel gets out of bounds of the iter");

    use_effect_with_deps(enclose! { (show_ready) move |show_cooldown: &UseStateHandle<bool>| {
        if !**show_cooldown {
            show_ready.set(false);
        }

        || ()
    }}, show_cooldown.clone());

    let collected_options = HypetrainSourceOptions {
        last_events_shown_count: *max_last_contributions as usize,
        color: *source_color,
        show_cooldown: *show_cooldown,
        show_ready: *show_ready,
    };

    html! {
        <>
            <h4 class="text-center">{ "Settings" }</h4>
            <div class="container mb-3">
                <div class="row gx-3 gy-4">

                    <div class="col-6">
                        <SourceColorChooser color={source_color.clone()} />
                    </div>

                    <div class="col-6">
                        <NumberInput
                            title={"Amount of last contributions shown"}
                            number_state={max_last_contributions.clone()}
                            min_val={0} max_val={5}
                        />
                    </div>

                    <div class="col-4">
                    </div>
                    <div class="col-4 p-3 border border-dark border-2">
                        <h5 class="text-center">{ "Cooldowns" }</h5>
                        <Checkbox
                            checked_state={show_cooldown.clone()}
                            is_active={true}
                            id="showCooldown"
                            label="Show hype train cooldown after finish"
                        />
                        <Checkbox
                            checked_state={show_ready}
                            is_active={*show_cooldown}
                            id="showReady"
                            label="Show hype train is ready after cooldown"
                        />
                    </div>
                    <div class="col-4">
                    </div>

                    <HypetrainCarousel settings={collected_options} {carousel_state} />

                    <div>
                        <h5 class="text-center mb-2">{ "Link to source (will require to login to Twitch in OBS)" }</h5>
                        // can't not have the generic, unfortunately
                        <StaticSourceLink<HypetrainSourceOptions> options={collected_options.clone()} source_name={"hype_train"} skin={chosen_skin.to_string()} />
                    </div>
                    <div>
                        <h5 class="text-center mb-2">{ "User-specific link" }</h5>
                        { 
                            login_gate(html!{
                                <LoginSourceLink<HypetrainSourceOptions> options={collected_options.clone()} source_name={"hype_train"} skin={chosen_skin.to_string()} />
                            })
                        }
                    </div>
                    
                    <div class="d-flex flex-column justify-content-center align-items-center">
                        <div>
                            <h5>{"Start a test event"}</h5>
                        </div>
                        <div>
                            { login_gate(html!{ <TestButton test_name={"hype_train"} timeout_secs={13} /> }) }
                        </div>
                    </div>

                </div>
            </div>
        </>
    }
}