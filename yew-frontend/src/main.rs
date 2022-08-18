use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod pages;
use components::sidebar::Sidebar;
use pages::dashboard::Dashboard;

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/projects")]
    Projects,
    #[at("/issues")]
    Issues,
    #[at("/user-settings")]
    UserSettings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <h1>{ "Home" }</h1>
        },
        Route::Dashboard => html! {
            <Dashboard />
        },
        Route::Projects => html! {
            <h1>{ "Projects" }</h1>
        },
        Route::Issues => html! {
            <h1>{ "Issues" }</h1>
        },
        Route::UserSettings => html! {
            <h1>{ "User Settings" }</h1>
        },
        Route::NotFound => html! {
            <h1>{ "404" }</h1>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="container">
                <Sidebar />
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
