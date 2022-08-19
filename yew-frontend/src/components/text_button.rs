use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TextButtonProps {
    pub text_value: String,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(TextButton)]
pub fn text_button(TextButtonProps { text_value, on_click }: &TextButtonProps) -> Html {
    html! {
        <div class="text-button" onclick={ on_click }>
            { text_value }
        </div>
    }
}
