use yew::function_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SkewPorps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SkewHeader)]
pub fn skew_header(props: &SkewPorps) -> Html {
    html!(
    <div class={classes!("self-start", "relative", "h-2", props.class.clone())}>
        <h3 class="absolute -skew-y-2 -top-2 -left-2 bg-slate-100 px-2">{for props.children.iter()}</h3>
    </div>
    )
}
