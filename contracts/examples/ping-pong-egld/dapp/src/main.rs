mod components;
mod context;
mod interactor;
mod pages;
mod requests;
mod routes;

use components::Footer;
use context::ConfigProvider;
use log::Level;
use routes::{Route, switch};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <ConfigProvider>
                <Switch<Route> render={switch} />
                <Footer />
            </ConfigProvider>
        </BrowserRouter>
    }
}

fn main() {
    // initialize the logger
    console_log::init_with_level(Level::Debug).expect("Failed to initialize logger");

    // render the app
    yew::Renderer::<App>::new().render();
}
