use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::projects::flappyking::FlappyKing;
use crate::pages::projects::simplechat::SimpleChat;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/flappyking")]
	FlappyKing,
	#[at("/simplechat")]
	SimpleChat,
	#[not_found]
	#[at("/404")]
	NotFound,
}

pub fn switch(route: Route) -> Html {
	match route {
		Route::Home => html! { <Home /> },
		Route::FlappyKing => html! { <FlappyKing /> },
		Route::SimpleChat => html! { <SimpleChat /> },
		Route::NotFound => html! { <PageNotFound /> },
	}
}
