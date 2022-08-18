use yew::prelude::*;
use crate::components::card::Card;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
        <div class="container">
            <div class="main">
                <div class="card-container">
                    <Card title="Card 1" />
                    <Card title="Card 2" />
                    <Card title="Card 3" />
                    <Card title="Card 4" />
                </div>
            </div>
        </div>
    }
}
