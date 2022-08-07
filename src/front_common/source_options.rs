use yew::{prelude::{Html, html, UseStateHandle}};

#[derive(Clone, Copy)]
pub enum SourceColor {
    White,
    Black
}

impl Default for SourceColor {
    fn default() -> Self {
        Self::Black
    }
}

#[derive(Default)]
pub struct NormalSourceOptions {
    pub color: SourceColor,
    pub is_expanded: bool
}

impl NormalSourceOptions {
    pub fn with_color(&self, color: SourceColor) -> Self {
        Self {
            color,
            is_expanded: self.is_expanded
        }
    }

    pub fn with_is_expanded(&self, is_expanded: bool) -> Self {
        Self {
            color: self.color,
            is_expanded
        }
    }

    fn render_color_option(state: &UseStateHandle<NormalSourceOptions>, color: SourceColor) -> Html {
        // makes sure to fill all options if there'll be more
        match color {
            SourceColor::White => html! {
                <div class="form-check">
                    <input
                        class="form-check-input" type="radio" name="textColor" id="textColorWhite"
                        onclick={
                                let state = state.clone();
                                move |_| state.set((*state).with_color(color))
                            }
                        />
                        <label class="form-check-label" for="textColorWhite">
                        { "While" }
                    </label>
                </div>
            },
            SourceColor::Black => html! {
                <div class="form-check">
                    <input
                        class="form-check-input" type="radio" name="textColor" id="textColorBlack" checked=true
                        onclick={
                                let state = state.clone();
                                move |_| state.set((*state).with_color(color))
                            }
                        />
                        <label class="form-check-label" for="textColorBlack">
                        { "Black" }
                    </label>
                </div>
            },
        }
    }

    fn render_color_options(state: &UseStateHandle<NormalSourceOptions>) -> Html {
        html! {
            <div class="col-6">
                <div class="p-3 border border-dark border-2 h-100">
                    <h5 class="text-center">{ "Text color" }</h5>
                    { NormalSourceOptions::render_color_option(state, SourceColor::Black) }
                    { NormalSourceOptions::render_color_option(state, SourceColor::White) }
                </div>
            </div>
        }
    }

    fn render_expand_option(state: &UseStateHandle<NormalSourceOptions>, is_expanded: bool) -> Html {
        match is_expanded {
            true => html! {
                <div class="col-12">
                    <div class="form-check">
                        <input
                            class="form-check-input" type="radio" name="expandedOption" id="expandedOptionTrue"
                            onclick={
                                let state = state.clone();
                                move |_| state.set((*state).with_is_expanded(is_expanded))
                            }
                        />
                            <label class="form-check-label" for="expandedOptionTrue">
                            { "Maximized" }
                        </label>
                    </div>
                </div>
            },
            false => html! {
                <div class="col-12">
                    <div class="form-check">
                        <input
                            class="form-check-input" type="radio" name="expandedOption" id="expandedOptionFalse" checked=true
                            onclick={
                                let state = state.clone();
                                move |_| state.set((*state).with_is_expanded(is_expanded))
                            }
                        />
                        <label class="form-check-label" for="expandedOptionFalse">
                            { "Minimized" }
                        </label>
                        </div>
                </div>
            },
        }
    }

    fn render_expand_options(state: &UseStateHandle<NormalSourceOptions>) -> Html {
        html! {
            <div class="col-6">
                <div class="p-3 border border-dark border-2 h-100">
                    <h5 class="text-center">{ "Size options" }</h5>
                    <div class="row">
                        { NormalSourceOptions::render_expand_option(state, false) }
                        { NormalSourceOptions::render_expand_option(state, true) }
                    </div>
                </div>
            </div>
        }
    }
    
    pub fn to_html(state: &UseStateHandle<NormalSourceOptions>) -> Html {
        html! {
            <>
                <h4 class="text-center">{ "Settings" }</h4>
                <div class="container mb-3">
                    <div class="row gx-3">
                        { NormalSourceOptions::render_color_options(state) }
                        { NormalSourceOptions::render_expand_options(state) }
                    </div>
                </div>
            </>
        }
    }
}