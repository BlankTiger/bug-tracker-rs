use yew::prelude::*;

#[function_component(Issues)]
pub fn issues() -> Html {
    html! {
        <div class="issues-fragment">
            <h1>{ "Issues" }</h1>
            <p>{ "This is the issues page" }</p>
        </div>
    }
}
