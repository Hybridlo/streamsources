use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SourceLinkProps {
    pub href: &'static str,
    pub name: &'static str,
    #[prop_or(false)]
    pub disabled: bool
}

#[function_component(SourceLink)]
pub fn source_link(props: &SourceLinkProps) -> Html {
    html! {
        <a href={ props.href } class={
            if props.disabled
            { "list-group-item list-group-item-action mb-2 disabled" }
            else
            { "list-group-item list-group-item-action mb-2" }
        } style="font-size: 1.25em;">{ props.name }</a>
    }
}