use yew::prelude::*;
use yew_hooks::use_measure;
use web_sys::HtmlElement;

pub fn scalable_wrapper(content: Html) -> Html {
    let parent_node = use_node_ref();
    let node = use_node_ref();
    let state = use_measure(parent_node.clone());
    let mut width_scale = 1.0;
    let mut height_scale = 1.0;

    if let Some(node) = node.cast::<HtmlElement>() {
        width_scale = state.width / (node.offset_width() as f64);
        height_scale = state.height / (node.offset_height() as f64);
    }

    html! {
        <div ref={parent_node} style={"height: 100%; text-align: center;"}>
            <div ref={node} style={format!("display: inline-block; transform-origin: top center; transform: scale({})", width_scale.min(height_scale))}>
                { content }
            </div>
        </div>
    }
}