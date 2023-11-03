use std::{cell::RefCell, rc::Rc};

use twitch_sources_rework::{GLOBAL_DELAY_VALUE, FPS, GLOBAL_DELAY_VALUE_SECONDS};
use twitch_sources_rework::front_common::transition_funcs::ease_in_out_formula;
use web_sys::Element;
use yew::prelude::*;

use gloo_timers::callback::Interval;
use yew_style_in_rs::style;

const CAROUSEL_SPEEDUP: u32 = 4;
const CAROUSEL_DELAY_VALUE: u32 = GLOBAL_DELAY_VALUE / CAROUSEL_SPEEDUP;
const CAROUSEL_DELAY_VALUE_SECONDS: f64 = GLOBAL_DELAY_VALUE_SECONDS as f64 / CAROUSEL_SPEEDUP as f64;

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    pub active_item: UseStateHandle<usize>,
    pub carousel_size: usize,
    pub items: Vec<Html>,
    pub height: usize,
    #[prop_or_default]
    pub bg_color_not_chosen: Option<String>,
    #[prop_or_default]
    pub bg_color_chosen: Option<String>
}

#[function_component(Carousel)]
pub fn carousel(props: &CarouselProps) -> Html {
    if props.carousel_size % 2 != 1 {
        panic!("Carousel mus have an odd amount of items");
    }
    
    let main_div_ref = use_node_ref();
    let animation: Rc<RefCell<Option<Interval>>> = use_mut_ref(|| None);
    let timer = use_mut_ref(|| 0.0);
    let offset_previous = use_mut_ref(|| 0);
    let offset_target = use_mut_ref(|| 0);
    
    
    let current_carousel_size = use_state(|| props.carousel_size);
    let item_previous = use_state(|| *props.active_item);
    let current_move = use_state(|| 0);
    let offset = use_state(|| 0);
    
    let mut rendered_items: Vec<(Html, i32)> = Vec::new();
    
    // this silly thing is needed because on wrapping the vector of items using min() would return the wrong result
    // like min item being 0 when you click back on (10 0 1) when for our use case the 10 is the min
    let min_item = if *current_move < 0 { *props.active_item } else { *item_previous };
    
    let start_underflow_protected = min_item + props.items.len();
    let start = start_underflow_protected - ((props.carousel_size - 1) / 2) % props.items.len();

    for idx in 0..*current_carousel_size {
        let position = (start+idx) % props.items.len();
        let mut offset = idx as i32 - (props.carousel_size as i32 / 2);

        if *current_move > 0 {
            // if div's are added to the end of the container, our offset 0 must be farther than
            // where it is during not expanded state, it does not happen when we go back,
            // because then these two align
            offset -= *current_move;
        }

        rendered_items.push((props.items[position].clone(), offset));
    }
    
    let partial_click_cb = {
        |item_offset: i32| {
            // i really don't like these long cloning blocks, i wonder if there's a better way to do this
            let timer_ref = timer.clone();
            let active_item = props.active_item.clone();
            let offset_previous = offset_previous.clone();
            let offset_target = offset_target.clone();
            let offset = offset.clone();
            let main_div_ref = main_div_ref.clone();
            let items_length = props.items.len();
            let main_div_items = props.carousel_size;
            let animation = animation.clone();
            let item_previous = item_previous.clone();
            let next_item = (*props.active_item as i32 + item_offset + items_length as i32) as usize % items_length;
            let current_carousel_size = current_carousel_size.clone();
            let current_move = current_move.clone();
            
            Callback::from(move |_| {
                if item_offset == 0 || animation.borrow().is_some() {
                    // i could not provide a callback for 0, but this is easier
                    return;
                }
                
                // since this is a callback - main_div can't not be mounted at the point this is called
                let main_div = main_div_ref.cast::<Element>().expect("Main div should be mounted");
                if item_offset > 0 {
                    offset_previous.replace(0);
                    offset_target.replace(main_div.client_width() / main_div_items as i32 * item_offset);
                } else {
                    offset_previous.replace(-main_div.client_width() / main_div_items as i32 * item_offset);
                    // set scroll at the same moment as the another element added to main div
                    // otherwise there's a jitter frame
                    offset.set(-main_div.client_width() / main_div_items as i32 * item_offset);
                    offset_target.replace(0);
                }

                active_item.set((*active_item as i32 + item_offset + items_length as i32) as usize % items_length);
                current_carousel_size.set(main_div_items + item_offset.abs() as usize);
                current_move.set(item_offset);
                *(timer_ref.borrow_mut()) = 0.0;

                let timer_ref = timer_ref.clone();
                let animation_clone = animation.clone();
                let item_previous = item_previous.clone();
                let offset_previous = offset_previous.clone();
                let offset_target = offset_target.clone();
                let current_carousel_size = current_carousel_size.clone();
                let offset = offset.clone();
                let current_move = current_move.clone();
                animation.replace(Some(Interval::new(CAROUSEL_DELAY_VALUE / FPS, move || {
                    let mut timer_borrow = timer_ref.borrow_mut();
                    *timer_borrow += (CAROUSEL_DELAY_VALUE_SECONDS as f64) / (FPS as f64);

                    let offset_previous_borrow = offset_previous.borrow();
                    let offset_target_borrow = offset_target.borrow();

                    // if time over the animation time - we're done
                    if *timer_borrow > (CAROUSEL_DELAY_VALUE_SECONDS as f64) {
                        animation_clone.replace(None);

                        item_previous.set(next_item);
                        current_carousel_size.set(main_div_items);
                        offset.set(0);
                        current_move.set(0);
                        return;
                    }

                    offset.set(ease_in_out_formula(
                        *timer_borrow,
                        *offset_previous_borrow as _,
                        (*offset_target_borrow - *offset_previous_borrow) as _,
                        CAROUSEL_DELAY_VALUE_SECONDS
                    ) as _);
                })));
            })
        }
    };


    let back_arrow = html! {
        <svg viewBox="0 0 32 32" width="32" height="32" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M 67.594 47.494 L 83.594 75.494 L 51.594 75.494 L 67.594 47.494 Z"
                style="stroke: rgb(0, 0, 0); stroke-width: 0px;"
                transform="matrix(0.000066, -1, 1, 0.000066, -47.496864, 83.590866)"
            />
        </svg>
    };

    let forward_arrow = html! {
        <svg viewBox="0 0 32 32" width="32" height="32" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M 67.594 47.494 L 83.594 75.494 L 51.594 75.494 L 67.594 47.494 Z"
                style="stroke: rgb(0, 0, 0); stroke-width: 0px;"
                transform="matrix(-0.000066, 1, -1, -0.000066, 79.499519, -51.58902)"
            />
        </svg>
    };

    
    let bg_color_chosen = props.bg_color_chosen.clone().unwrap_or("gray".to_string());
    let bg_color_not_chosen = props.bg_color_not_chosen.clone().unwrap_or("silver".to_string());
    let width_percents = 100.0 / (*current_carousel_size as f64);
    let scroller_width = 100.0 * (*current_carousel_size as f64) / props.carousel_size as f64;
    style! {
        let css = dyn css! {r#"
            & {
                position: relative;
            }

            & .scroller {
                position: relative;
                height: 100%;
                width: ${scroller_width}%;
                padding: 0 20px;
            }

            & .carousel-item {
                width: ${width_percents}%;
                margin: 5px;
            }
            
            & .carousel-item.chosen {
                background-color: ${bg_color_chosen};
            }
            
            & .carousel-item.not-chosen {
                background-color: ${bg_color_not_chosen};
            }

            & .arrow-button {
                z-index: 9999;
                position: absolute;
                transform: translateY(-50%);
                top: 50%;
                width: 64px;
                height: 64px;
            }
            
            & .arrow-button.arrow-back {
                left: 0%;
            }
            
            & .arrow-button.arrow-forward {
                transform: translate(-100%, -50%);
                left: 100%;
            }
        "#};
    }
    
    let carousel_item_classes = vec![
        "d-flex",
        "flex-grow-1",
        "flex-basis-0",

        "carousel-item",

        "justify-content-center",
        "align-items-center",

        "border",
        "border-dark",
        "rounded"
    ];

    html! {
        <div ref={main_div_ref.clone()} class={classes!(css, "overflow-hidden")} style={format!("height: {}px;", props.height)}>
            <button
                class="btn btn-light rounded-circle border-dark arrow-button arrow-back"
                onclick={partial_click_cb(-1)}
                disabled={animation.borrow().is_some()}
            >{back_arrow}</button>
            <button
                class="btn btn-light rounded-circle border-dark arrow-button arrow-forward"
                onclick={partial_click_cb(1)}
                disabled={animation.borrow().is_some()}
            >{forward_arrow}</button>
            <div class="d-flex scroller" style={format!("left: -{}px;", *offset)}>
                {
                    rendered_items.into_iter().map(|(item, offset)| {
                        html! {
                            <div
                                class={classes!(carousel_item_classes.clone(), if offset == 0 {"chosen"} else {"not-chosen"} )}
                                onclick={partial_click_cb(offset)}
                                data-offset={offset.to_string()}
                                disabled={offset!=0}
                            >{item}</div>
                        }
                    }).collect::<Html>()
                }
            </div>
            <p>{ format!("Active item: {}", *props.active_item) }</p>
        </div>
    }
}