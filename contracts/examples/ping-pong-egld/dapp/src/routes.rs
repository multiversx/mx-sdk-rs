use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::HomePage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
    }
}
