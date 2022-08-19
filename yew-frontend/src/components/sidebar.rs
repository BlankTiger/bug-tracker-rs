use crate::components::text_button::TextButton;
use crate::components::text_button::TextButtonProps;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let dashboard_button = {
        let props = TextButtonProps {
            text_value: "💻 Dashboard".to_string(),
        };
        html! {
            <Link<Route> to={Route::Dashboard}>
                <TextButton text_value={ props.text_value } />
            </Link<Route>>
        }
    };

    let projects_button = {
        let props = TextButtonProps {
            text_value: "🗃️ Projects".to_string(),
        };
        html! {
            <Link<Route> to={Route::Projects}>
                <TextButton text_value={ props.text_value } />
            </Link<Route>>
        }
    };

    let issues_button = {
        let props = TextButtonProps {
            text_value: "📜 Issues".to_string(),
        };
        html! {
            <Link<Route> to={Route::Issues}>
                <TextButton text_value={ props.text_value } />
            </Link<Route>>
        }
    };

    let user_settings_button = {
        let props = TextButtonProps {
            text_value: "🔧 User Settings".to_string(),
        };
        html! {
            <Link<Route> to={Route::UserSettings}>
                <TextButton text_value={ props.text_value } />
            </Link<Route>>
        }
    };

    html! {
        <div class="sidebar">
            <div class="overflow"></div>
            { dashboard_button }
            { projects_button }
            { issues_button }
            <div class="sidebar-divider"></div>
            { user_settings_button }
        </div>
    }
}
