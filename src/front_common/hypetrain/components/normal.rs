use std::{rc::Rc, cell::RefCell};

use chrono::Utc;
use gloo_render::{AnimationFrame, request_animation_frame};
use uuid::Uuid;
use yew::{function_component, html, Html, UseStateHandle, use_effect_with_deps, Properties, use_state, use_mut_ref, classes, use_state_eq};
use yew_style_in_rs::style;

use crate::common_data::eventsub_msgs::ContributionType;
use crate::front_common::hypetrain::HypetrainStatus;
use crate::front_common::{scalable_wrapper, SourceColor, enclose, use_action_list};
use crate::front_common::animated_values::use_animated_f64;
use crate::front_common::transition_funcs::ease_in_out_formula;
use crate::front_common::hypetrain::{options::HypetrainProps, HypetrainState, HypetrainContributionState};
use crate::front_common::components::CountdownTimer;

#[derive(PartialEq, Properties)]
struct LastMsgProp {
    last_message: UseStateHandle<HypetrainState>
}
#[function_component(Title)]
fn title(props: &LastMsgProp) -> Html {
    let level = use_state(|| 0u64);
    let show_title = use_state(|| false);
    let actions = use_action_list();

    use_effect_with_deps(enclose! { (level, show_title) move |last_message: &UseStateHandle<HypetrainState>| {
        if *level != last_message.curr_level {
            show_title.set(false);
            
            let curr_level = last_message.curr_level;

            actions.start(vec![
                (Box::new(|| 500), Box::new(enclose! { (show_title) move || {
                    level.set(curr_level);
                    show_title.set(true);
                }})),
            ]);
        }

        || ()
    }}, props.last_message.clone());

    style! {
        let css = dyn css! {r#"
            & .title {
                transition: opacity 200ms ease-out;
                font-size: 30px;
                font-weight: bold;
            }

            & .title.show {
                opacity: 1;
            }

            & .title.hide {
                opacity: 0;
            }
        "#};
    }

    html! {
        <div class={classes!(css)}>
            <span
                class={if *show_title { "mx-auto title show" } else { "mx-auto title hide" }}
            >
                { "Hype train level " }{ level.to_string() }
            </span>
        </div>
    }
}

fn progress_bar_color(level: u64) -> String {
    match (level.saturating_sub(1)) % 5 {
        0 => "#0000bf",
        1 => "#00bfbf",
        2 => "#00bf00",
        3 => "#bf6000",
        4 => "#bf0000",
        _ => unreachable!()
    }.to_string()
}

#[derive(PartialEq, Properties)]
struct ProgressBarProps {
    last_message: UseStateHandle<HypetrainState>,
    color: SourceColor,
}
#[function_component(ProgressBar)]
fn progress_bar(props: &ProgressBarProps) -> Html {
    let progress_state = use_animated_f64(1.0, ease_in_out_formula);
    let color_state = use_state_eq(|| progress_bar_color(1));

    use_effect_with_deps(enclose!{ (progress_state, color_state) move |last_message: &UseStateHandle<HypetrainState>| {
        progress_state.set_value(last_message.curr_points as f64 / last_message.needed_points as f64);
        color_state.set(progress_bar_color(last_message.curr_level));

        || ()
    }}, props.last_message.clone());

    const TOTAL_WIDTH: f64 = 500.0;
    const ZERO_VAL: f64 = 20.0;
    const STROKE_WIDTH: f64 = 4.0;

    let box_width = ZERO_VAL - STROKE_WIDTH + (TOTAL_WIDTH - ZERO_VAL) * *progress_state;
    let progress_width = ZERO_VAL - 1.0 + (TOTAL_WIDTH - ZERO_VAL) * *progress_state;

    style!{
        let css = dyn css! {r#"
            animation: ##animate-train-bar## 3s infinite linear;
        "#};
        let css2 = dyn css! {r#"
            font-size: 24px;
            font-weight: bold;
        "#};
        dyn keyframes! {r#"
            @keyframes animate-train-bar {
                from { transform: translateX(0px); }
                to { transform: translateX(98.4px); }
            }
        "#}
    }

    html! {
        <>
            <div class={classes!(css2)}>
                {format!("{}%", (*progress_state * 100.0) as u64)}
            </div>
            <div
                style="width: 500px;"
                class={classes!(
                    "text-start",
                    "border",
                    { if props.color == SourceColor::White {"border-light"} else {"border-dark"} },
                    "border-3"
                )
            }>
                <svg
                    viewBox="0 0 500 200"
                    preserveAspectRatio="xMinYMin slice"
                    width={progress_width.to_string()}
                    height="200"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <defs></defs>
                    <rect
                        width={box_width.to_string()}
                        height="196"
                        style={"stroke: ".to_string() + &*color_state + "; fill: " + &*color_state + "; stroke-width: 6px; fill-opacity: 0.2;"}
                        x="1" y="2"
                    ></rect>
                    <path
                        class={classes!(css)}
                        style={"stroke: ".to_string() + &*color_state + "; stroke-width: 4px; fill-opacity: 0; transform-box: fill-box; transform-origin: 50% 50%;"}
                        d="M -97.2 100 C -92.2 70 -52.6 70 -47.6 100 C -42.6 130 -3 130 2 100 C 7 70 46.6 70 51.6 100 C 56.6 130 96.2 130 101.2 100 C 106.2 70 145.8 70 150.8 100 C 155.8 130 195.4 130 200.4 100 C 205.4 70 245 70 250 100 C 255 130 294.6 130 299.6 100 C 304.6 70 344.2 70 349.2 100 C 354.2 130 393.8 130 398.8 100 C 403.8 70 443.4 70 448.4 100 C 453.4 130 493 130 498 100">
                    </path>
                </svg>
            </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct TopBlockProps {
    last_message: UseStateHandle<HypetrainState>,
    contribution_type: ContributionType,
}

#[function_component(TopBlock)]
fn top_block(props: &TopBlockProps) -> Html {
    let curr_top_contrib_username = use_state_eq(|| "".to_string());
    let curr_top_contrib_points = use_animated_f64(1.0, ease_in_out_formula);
    let show_contributor = use_state_eq(|| false);
    let actions = use_action_list();

    let contrib_type = props.contribution_type.clone();
    use_effect_with_deps(enclose!{
        (curr_top_contrib_username, curr_top_contrib_points, show_contributor)
        move |last_msg: &UseStateHandle<HypetrainState>| {
            let contrib = match contrib_type {
                ContributionType::Bits => last_msg.top_bit_donator.clone(),
                ContributionType::Subscription => last_msg.top_gifter.clone(),
                ContributionType::Other => unreachable!(),
            };

            if let Some(contrib) = contrib {
                if contrib.user_name == *curr_top_contrib_username {
                    curr_top_contrib_points.set_value(contrib.total as _);
                } else {
                    show_contributor.set(false);

                    actions.start(vec![
                        (Box::new(|| 500), Box::new(move || {
                            show_contributor.set(true);
                            curr_top_contrib_username.set(contrib.user_name);
                            curr_top_contrib_points.immediate_set_value(contrib.total as _);
                        })),
                    ])
                }
            }

            || ()
        }
    }, props.last_message.clone());

    style! {
        let css = dyn css! {r#"
            transition: opacity 200ms ease-out;
            font-size: 21px;
            font-weight: 500;

            &.show {
                opacity: 1;
            }

            &.hide {
                opacity: 0;
            }
        "#};
    }

    html! {
        <div class={classes!(css, {if *show_contributor {"show"} else {"hide"}})}>
            <div>{ format!("Top {}", props.contribution_type.name()) }</div>
            <div>{ format!("{}", (*curr_top_contrib_username).clone())}</div>
            <div>
                {
                    (match props.contribution_type {
                        ContributionType::Bits => *curr_top_contrib_points,
                        ContributionType::Subscription => *curr_top_contrib_points / 500.0,
                        ContributionType::Other => *curr_top_contrib_points,
                    } as u64).to_string()
                }
                {
                    if let ContributionType::Subscription = props.contribution_type {
                        format!(" ({})", *curr_top_contrib_points as u64)
                    } else {
                        "".to_string()
                    }
                }
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct LatestBlockProps {
    last_message: UseStateHandle<HypetrainState>,
    max_last_contributions: usize
}
#[function_component(LastestBlock)]
fn latest_block(props: &LatestBlockProps) -> Html {
    let latest_contributions: UseStateHandle<Vec<(i64, Uuid, HypetrainContributionState)>> = use_state_eq(|| Vec::new());
    let next_frame_change_order: Rc<RefCell<Option<AnimationFrame>>> = use_mut_ref(|| None);

    let shown_contributions = props.max_last_contributions;
    
    let latest_contributions = latest_contributions.clone();
    let next_frame_change_order = next_frame_change_order.clone();

    use_effect_with_deps(enclose! { (latest_contributions, next_frame_change_order) move |last_msg: &UseStateHandle<HypetrainState>| {
        let mut contributions = last_msg.last_contributions
            .clone()
            .into_iter()
            .map(|contribution| (Uuid::new_v4(), contribution))
            .collect::<Vec<_>>();

        contributions.extend(
            (*latest_contributions)
                .clone()
                .into_iter()
                .map(|(_, uuid, contribution)| (uuid, contribution))
                .take(shown_contributions + 1)
        );

        let contributions: Vec<(i64, Uuid, HypetrainContributionState)> = contributions
            .into_iter()
            .enumerate()
            .map(|(idx, (uuid, contribution))| (idx as i64 - 1, uuid, contribution))
            .collect();

        latest_contributions.set(contributions.clone());

        let latest_contribution = latest_contributions.clone();
        next_frame_change_order.replace(Some(request_animation_frame(move |_| {
            let contributions = contributions
                .clone()
                .into_iter()
                .map(|(_, uuid, contribution)| (uuid, contribution))
                .enumerate()
                .map(|(idx, (uuid, contribution))| (idx as i64, uuid, contribution))
                .collect();

            latest_contribution.set(contributions);
        })));

        || ()
    }}, props.last_message.clone());

    style! {
        let css = dyn css! {r#"
            & .hidden-item {
                opacity: 0;
                font-size: 18px;
            }

            & .item {
                position: absolute;
                top: 0;
                font-size: 18px;
                font-weight: 500;

                transition: transform 0.3s ease;
                transform: translateY(calc(var(--order) * 100%));
            }
        "#};
    }

    html! {
        <div class={classes!(css, "position-relative", "overflow-hidden")}>
            {
                (0..props.max_last_contributions).map(|_| {
                    html! {
                        <div class="hidden-item">
                            {"Placeholder"}
                        </div>
                    }
                }).collect::<Html>()
            }
            {
                (*latest_contributions).clone().into_iter().map(|(idx, uuid, contribution)| {
                    html! {
                        <div class="item w-100" style={format!("--order: {}", idx)} key={uuid.to_string()}>
                            { 
                                format!(
                                    "{} contributed {} {}",
                                    contribution.user_name,
                                    contribution.contrib_amount(),
                                    contribution.type_.name()
                                )
                            }
                            {
                                if let ContributionType::Subscription = contribution.type_ {
                                    format!(" ({})", contribution.total)
                                } else {
                                    "".to_string()
                                }
                            }
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}

#[function_component(HypetrainNormal)]
pub fn hypetrain_normal(props: &HypetrainProps) -> Html {
    let shown = use_state_eq(|| false);
    let status = use_state_eq(|| HypetrainStatus::Hypetrain);
    let actions = use_action_list();

    use_effect_with_deps(enclose! {
        (shown, status)
        move |last_msg: &UseStateHandle<HypetrainState>| {
            if last_msg.curr_level > 0 {
                // Reset animations, since they'll be re-set later
                actions.stop();

                if last_msg.cooldown_ends_at.is_some() {
                    shown.set(false);
    
                    let cd_ends_at = last_msg.cooldown_ends_at.unwrap();

                    actions.start(vec![
                        (Box::new(|| 500), Box::new(enclose! { (shown, status) move || {
                            shown.set(true);
                            status.set(HypetrainStatus::Finished);
                        }})),
                        (Box::new(|| 10_000), Box::new(enclose! { (shown) move || {
                            shown.set(false);
                        }})),
                        (Box::new(|| 500), Box::new(enclose! { (shown, status) move || {
                            shown.set(true);
                            status.set(HypetrainStatus::Cooldown);
                        }})),
                        (
                            Box::new(move || (cd_ends_at - Utc::now()).num_milliseconds().max(500).try_into().unwrap()),
                            Box::new(enclose! { (shown) move || {
                                shown.set(false);
                            }})
                        ),
                        (Box::new(|| 500), Box::new(enclose! { (shown, status) move || {
                            shown.set(true);
                            status.set(HypetrainStatus::Ready);
                        }})),
                    ])
                } else {
                    // Hype train not done but status not in `Hypetrain` state (should be impossible, but just in case)
                    if *status != HypetrainStatus::Hypetrain {
                        shown.set(false);

                        actions.start(vec![
                            (Box::new(|| 500), Box::new(enclose! { (shown, status) move || {
                                status.set(HypetrainStatus::Hypetrain);
                                shown.set(true);
                            }})),
                        ])
                    } else {
                        // Hype train not done and status is `Hypetrain` (should be shown anyway already, but still "just in case")
                        shown.set(true);
                    }
                }
            }
            || ()
        }
    }, props.last_message.clone());

    let outline_color = if props.settings.color == SourceColor::White { "white" } else { "black" };
    let outline_color_opposite = if props.settings.color == SourceColor::White { "black" } else { "white" };

    style! {
        let css = dyn css! {r#"
            --outlines: ${outline_color};
            --outlines-opposite: ${outline_color_opposite};
            color: var(--outlines);
            -webkit-text-stroke: 0.4px var(--outlines-opposite);
            transition: opacity 200ms ease-out;

            &.hide {
                opacity: 0;
            }

            &.show {
                opacity: 1;
            }

            & .hide {
                opacity: 0;
            }

            & .show {
                opacity: 1;
            }

            & .timer {
                font-size: 18px;
                font-weight: 500;
            }
        "#};
    }

    scalable_wrapper(html! {
        <div class={classes!(css, "text-center", { if *shown {"show"} else {"hide"} } )}>
        {
            match *status {
                HypetrainStatus::Hypetrain => html! {
                    <>
                        <Title last_message={props.last_message.clone()} />
                        <div class="timer">
                            {" Ends in: "}
                            <CountdownTimer target_time={props.last_message.ends_at} />
                        </div>
                        <ProgressBar last_message={props.last_message.clone()} color={props.settings.color} />
                        <div class="row m-0">
                            <div class="col-6">
                                <TopBlock
                                    last_message={props.last_message.clone()}
                                    contribution_type={ContributionType::Subscription}
                                />
                            </div>
                            <div class="col-6">
                                <TopBlock
                                    last_message={props.last_message.clone()}
                                    contribution_type={ContributionType::Bits}
                                />
                            </div>
                        </div>
                        <LastestBlock last_message={props.last_message.clone()} max_last_contributions={props.settings.last_events_shown_count} />
                    </>
                },
                HypetrainStatus::Finished => html! {
                    <div style="font-size: 30px; font-weight: bold;">
                        {
                            format!(
                                "Hype train finished at level {} and {}%",
                                props.last_message.curr_level,
                                (props.last_message.curr_points as f64 * 100.0 / props.last_message.needed_points as f64) as u64
                            )
                        }
                    </div>
                },
                HypetrainStatus::Cooldown => html! {
                    <div class={classes!("timer", { if props.settings.show_cooldown {"show"} else {"hide"} })}>
                        {"Hype train can be launched again in: "}
                        <CountdownTimer target_time={props.last_message.cooldown_ends_at} />
                    </div>
                },
                HypetrainStatus::Ready => html! {
                    <div style="font-size: 30px; font-weight: bold;" class={if props.settings.show_ready {"show"} else {"hide"}} >
                        {"Hype train is ready!"}
                    </div>
                },
            }
        }
        
        </div>
    })
}