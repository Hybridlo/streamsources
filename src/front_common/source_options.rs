use yew::prelude::{Html, html};

pub enum SourceColor {
    White,
    Black
}

pub struct NormalSourceOptions {
    pub color: SourceColor,
    pub is_expanded: bool
}

impl NormalSourceOptions {
    fn render_color_option(color: &SourceColor) -> Html {
        // makes sure to fill all options if there'll be more
        match color {
            SourceColor::White => html! {
                <div class="form-check">
                    <input class="form-check-input" type="radio" name="textColor" id="textColorWhite" />
                    <label class="form-check-label">
                        { "While" }
                    </label>
                </div>
            },
            SourceColor::Black => html! {
                <div class="form-check">
                    <input class="form-check-input" type="radio" name="textColor" id="textColorBlack" checked=true />
                    <label class="form-check-label">
                        { "Black" }
                    </label>
                </div>
            },
        }
    }

    fn render_color_options() -> Html {
        html! {
            <div class="col-6">
                <div class="p-3 border border-dark border-2 h-100">
                    <h5 class="text-center">{ "Text color" }</h5>
                    { NormalSourceOptions::render_color_option(&SourceColor::Black) }
                    { NormalSourceOptions::render_color_option(&SourceColor::White) }
                </div>
            </div>
        }
    }

    fn render_expand_option(is_expanded: bool) -> Html {
        match is_expanded {
            true => html! {
                <div class="col-12">
                    <div class="form-check">
                        <input class="form-check-input" type="radio" name="expandedOption" id="expandedOptionTrue" />
                        <label class="form-check-label">
                            { "Maximized" }
                        </label>
                    </div>
                </div>
            },
            false => html! {
                <div class="col-12">
                    <div class="form-check">
                        <input class="form-check-input" type="radio" name="expandedOption" id="expandedOptionFalse" checked=true />
                        <label class="form-check-label">
                            { "Minimized" }
                        </label>
                    </div>
                </div>
            },
        }
    }

    fn render_expand_options() -> Html {
        html! {
            <div class="col-6">
                <div class="p-3 border border-dark border-2 h-100">
                    <h5 class="text-center">{ "Size options" }</h5>
                    <div class="row">
                        { NormalSourceOptions::render_expand_option(false) }
                        { NormalSourceOptions::render_expand_option(true) }
                    </div>
                </div>
            </div>
        }
    }
    
    pub fn to_html() -> Html {
        html! {
            <>
                <h4 class="text-center">{ "Settings" }</h4>
                <div class="container mb-3">
                    <div class="row gx-3">
                        { NormalSourceOptions::render_color_options() }
                        { NormalSourceOptions::render_expand_options() }
                    </div>
                </div>
            </>
        }
    }
}