use yew::prelude::*;

#[function_component(NotFound)]
pub fn notfound() -> Html {
    html! {
        <center>
            <h1>{ "404 page not found" }</h1>
        </center>
    }
}
