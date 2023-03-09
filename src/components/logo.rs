use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

use crate::router::Route;

#[styled_component]
pub fn Logo() -> Html {
	html! {
		<Link<Route> to={Route::Home}>
			<div class="justify-self-start inline-flex content-center items-center space-x-2">
				<i class="fa-solid fa-trademark text-3xl text-rainbow-1" style="filter:drop-shadow(1px 1px 1px black)"></i>
				<span class="text-foreground-primary">
					{"Home"}
				</span>
			</div>
		</Link<Route>>
	}
}
