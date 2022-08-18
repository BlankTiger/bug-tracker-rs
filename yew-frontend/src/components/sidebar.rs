use crate::components::text_button::TextButton;
use crate::components::text_button::TextButtonProps;
use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let text_values: Vec<String> = vec![
        "💻 Dashboard".to_string(),
        "🗃️ Projects".to_string(),
        "📜 Issues".to_string(),
    ];

    let button_props = text_values
        .iter()
        .map(|text_value| {
            let on_click_msg = text_value.clone().split_at(1).1.to_string();
            TextButtonProps {
                text_value: text_value.clone(),
                on_click: Callback::from(move |_| {
                    println!("{}", on_click_msg);
                }),
            }
        })
        .collect::<Vec<TextButtonProps>>();

    let buttons = button_props
        .iter()
        .map(move |button_props| {
            html! {<TextButton text_value={ button_props.text_value.clone() } on_click={ button_props.on_click.clone() } />}
        })
        .collect::<Html>();

    let user_settings_props = TextButtonProps {
        text_value: "🔧 User Settings".to_string(),
        on_click: Callback::from(|_| {}),
    };

    let user_settings_button = html! {
        <TextButton text_value={user_settings_props.text_value} on_click={user_settings_props.on_click} />
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
