use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TextButtonProps {
    pub text_value: String,
}

#[function_component(TextButton)]
pub fn text_button(TextButtonProps { text_value }: &TextButtonProps) -> Html {
    html! {
        <li class="text-button">
            { text_value }
        </li>
    }
}
