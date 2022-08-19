use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod pages;
use components::sidebar::Sidebar;
use pages::dashboard::Dashboard;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/home")]
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
            <center>
            <h1>{ "404" }</h1>
            </center>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    let current_route = use_state(|| Route::Dashboard);
    let change_route = {
        // let route = current_route.clone();
        Callback::from(move |route: Route| {
            current_route.set(route);
        })
    };

    html! {
        <BrowserRouter>
            <div class="container">
                <Sidebar on_click={change_route} />
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
