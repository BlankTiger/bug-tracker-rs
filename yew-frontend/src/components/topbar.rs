use yew::prelude::*;
use crate::components::navbar::NavbarMsg;

#[derive(Properties, PartialEq, Clone)]
pub struct TopbarProps {
    pub on_click: Callback<NavbarMsg>
}

#[function_component(Topbar)]
pub fn topbar(props: &TopbarProps) -> Html {
    let onclick = props.on_click.clone();
    let onclick = move |_| {
        onclick.emit(NavbarMsg::Toggle)
    };

    html! {
        <div class="topbar">
            <div class="topbar-open-navbar-button" {onclick}>{"✨"}</div>
        </div>
    }
}
