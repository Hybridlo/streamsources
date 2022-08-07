use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::BaseRoute;

#[derive(Properties, PartialEq)]
pub struct SourceLinkProps {
    pub href: BaseRoute,
    pub name: &'static str,
    #[prop_or(false)]
    pub disabled: bool
}

#[function_component(SourceLink)]
pub fn source_link(props: &SourceLinkProps) -> Html {
    let link_active = {
        if props.disabled
        { "source-button list-group-item list-group-item-action mb-2 disabled" }
        else
        { "source-button list-group-item list-group-item-action mb-2" }
    };

    html! {
        <Link<BaseRoute> to={props.href} classes={ classes!(link_active) }>{ props.name }</Link<BaseRoute>>
    }
}