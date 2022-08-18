use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct CardProps {
    pub title: String,
}

#[function_component(Card)]
pub fn card(card_props: &CardProps) -> Html{
    html! {
        <div class="card">
            <div class="card-title">{&card_props.title}</div>
        </div>
    }
}
