use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod pages;
// use components::navbar::Navbar;
use components::navbar::Navbar;
use pages::dashboard::Dashboard;

#[derive(Routable, PartialEq, Eq, Clone)]
pub enum Route {
    #[at("/dashboard/home")]
    Home,
    #[at("/dashboard/projects")]
    Projects,
    #[at("/dashboard/issues")]
    Issues,
    #[at("/dashboard/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        // TODO: Extract out into components
        Route::Home => html! {
            <Dashboard />
        },
        Route::Projects => html! {
            <center>
            <h1>{ "Projects" }</h1>
            </center>
        },
        Route::Issues => html! {
            <center>
            <h1>{ "Issues" }</h1>
            </center>
        },
        Route::Settings => html! {
            <center>
            <h1>{ "Settings" }</h1>
            </center>
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
    html! {
        <BrowserRouter>
            <div class="container">
                <Navbar />
                <Switch<Route> render={Switch::render(switch)} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
