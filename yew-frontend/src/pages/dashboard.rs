use yew::prelude::*;
use crate::components::card::Card;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="card-container">
            <Card title="" />
            <Card title="" />
            <Card title="" />
            <Card title="" />
        </div>
    }
}
