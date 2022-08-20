use crate::components::text_button::TextButton;
use crate::components::text_button::TextButtonProps;
use crate::components::topbar::Topbar;
use crate::Route;
use yew::classes;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct Navbar {
    classes: Vec<&'static str>,
}
pub enum NavbarMsg {
    Toggle,
}

impl Component for Navbar {
    type Message = NavbarMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            classes: vec!["navbar"],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavbarMsg::Toggle => {
                if self.classes.contains(&"active") {
                    self.classes.pop();
                } else {
                    self.classes.push("active");
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
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

        let link = ctx.link();
        let on_click = link.callback(|_| NavbarMsg::Toggle);
        let onclick = link.callback(|_| NavbarMsg::Toggle);

        html! {
            <>
                <Topbar {on_click} />
                <nav class={classes!(self.classes.clone())}>
                    <div class="navbar-close-button" onclick={onclick.clone()}>{"❌"}</div>
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
            </>
        }
    }
}
