use crate::components::text_button::TextButton;
use crate::components::text_button::TextButtonProps;
use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let text_values = vec!["💻 Dashboard", "🗃️ Projects", "📜 Issues"];
    let button_props = text_values
        .iter()
        .map(|text_value| TextButtonProps {
            text_value: text_value.to_string(),
        })
        .collect::<Vec<TextButtonProps>>();

    let buttons = button_props
        .iter()
        .map(move |button_props| {
            html! {<TextButton text_value={ button_props.text_value.clone() } />}
        })
        .collect::<Html>();

    let user_settings_button = html! {
        <TextButton text_value="🔧 User Settings" />
    };

    html! {
        <div class="sidebar">
            <div class="overflow"></div>
            { buttons }
            <div class="sidebar-divider"></div>
            { user_settings_button }
        </div>
    }
}
