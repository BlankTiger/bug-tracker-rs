use yew::prelude::*;
use yew_router::prelude::*;
mod components;
mod pages;
use components::navbar::Navbar;
use pages::{dashboard, issues, notfound, projects, settings};

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
        Route::Home => html! {
            <dashboard::Dashboard />
        },
        Route::Projects => html! {
            <projects::Projects />
        },
        Route::Issues => html! {
            <issues::Issues />
        },
        Route::Settings => html! {
            <settings::Settings />
        },
        Route::NotFound => html! {
            <notfound::NotFound />
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
