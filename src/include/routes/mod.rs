use yew::prelude::*;
use yew_router::prelude::*;



mod pages;
use pages::{
    login::Login,
    home::Home,
    not_found::NotFound
};

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/")] Home,
    #[at("/login")] Login,
    #[not_found] #[at("/404")] NotFound,
}

#[allow(dead_code)]
pub fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Home     => html! {<Home />},
        AppRoute::Login    => html! {<Login />},
        AppRoute::NotFound => html! {<NotFound />}
    }
}