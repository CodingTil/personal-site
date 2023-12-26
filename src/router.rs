use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home::Home;
use crate::pages::page_not_found::PageNotFound;
use crate::pages::projects::eiuie::Eiuie;
use crate::pages::projects::flappyking::FlappyKing;
use crate::pages::projects::fractal::Fractal;
use crate::pages::projects::py_css::Pycss;
use crate::pages::projects::simplechat::SimpleChat;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
	#[at("/")]
	Home,
	#[at("/flappyking")]
	FlappyKing,
	#[at("/simplechat")]
	SimpleChat,
	#[at("/fractal")]
	Fractal,
	#[at("/eiuie")]
	Eiuie,
	#[at("/py_css")]
	Pycss,
	#[not_found]
	#[at("/404")]
	NotFound,
}

fn scroll_to_top() {
	if let Some(window) = web_sys::window() {
		let mut options = web_sys::ScrollToOptions::new();
		options.top(0.0);
		window.scroll_with_scroll_to_options(&options);
	}
}

pub fn switch(route: Route) -> Html {
	scroll_to_top();

	match route {
		Route::Home => html! { <Home /> },
		Route::FlappyKing => html! { <FlappyKing /> },
		Route::SimpleChat => html! { <SimpleChat /> },
		Route::Fractal => html! { <Fractal /> },
		Route::Eiuie => html! { <Eiuie /> },
		Route::Pycss => html! { <Pycss /> },
		Route::NotFound => html! { <PageNotFound /> },
	}
}
