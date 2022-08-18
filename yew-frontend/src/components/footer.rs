use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <p>{ "Copyright © 2022, Maciej Urban" }</p>
        </footer>
    }
}
