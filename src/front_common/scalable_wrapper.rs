use yew::prelude::*;
use yew_hooks::use_measure;

pub fn scalable_wrapper(content: Html) -> Html {
    let parent_node = use_node_ref();
    let node = use_node_ref();
    let parent_size = use_measure(parent_node.clone());
    let node_size = use_measure(node.clone());
    let width_scale = parent_size.width / (node_size.width as f64);
    let height_scale = parent_size.height / (node_size.height as f64);

    html! {
        <div ref={parent_node} style={"height: 100%; width: 100%;"}>
            <div
                ref={node}
                style={
                    format!(
                        "display: inline-block; transform-origin: top left; transform: scale({}) translateX(-50%); position: relative; left: 50%;",
                        width_scale.min(height_scale)
                    )
                }
            >
                { content }
            </div>
        </div>
    }
}