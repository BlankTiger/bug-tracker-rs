use crate::components::text_button::TextButton;
use crate::components::text_button::TextButtonProps;
use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Navbar)]
pub fn navbar() -> Html {
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

    let settings_button = {
        let props = TextButtonProps {
            text_value: "🔧 Settings".to_string(),
        };
        html! {
            <Link<Route> to={Route::Settings}>
                <TextButton text_value={ props.text_value } />
            </Link<Route>>
        }
    };

    html! {
        <nav class="navbar">
            <div class="overflow"></div>
            <ul class="navbar-route-links">
                { dashboard_button }
                { projects_button }
                { issues_button }
            </ul>
            <div class="navbar-divider"></div>
            <ul class="navbar-route-links">
                { settings_button }
            </ul>
        </nav>
    }
}
