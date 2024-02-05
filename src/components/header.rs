use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

use crate::localization::{use_localization, Localization};
use crate::router::Route;

#[styled_component]
pub fn Header() -> Html {
	let localization = use_localization();

	let current_locale = localization.get().clone();

	let other_locale = match current_locale {
		Localization::DE => Localization::EN,
		Localization::EN => Localization::DE,
	};

	let other_locale_str = other_locale.to_string();

	let switch_locale = Callback::from(move |_| localization.set(other_locale.clone()));

	html! {
		<div class="h-20 sticky top-0 z-50 w-full border-solid border-b-2 bg-background-primary border-background-secondary">
			<nav class="container mx-auto pt-5 pb-3 max-w-7xl px-4 xl:px-0 flex justify-between">
				<Link<Route> to={Route::Home} classes="group flex items-center space-x-2">
					<div class="text-rainbow-1 group-hover:brightness-125">
						<i class="fa-solid fa-trademark text-3xl" style="filter:drop-shadow(1px 1px 1px black)"></i>
					</div>
					<span class="text-foreground-secondary text-lg font-bold">{"Home"}</span>
				</Link<Route>>
				<button onclick={switch_locale} class="group flex items-center space-x-2">
					<span class="sr-only text-foreground-secondary text-lg font-bold">{"Switch language to "}</span>
					<span class="text-foreground-secondary text-lg font-bold">{other_locale_str}</span>
					<div class="text-rainbow-1 group-hover:brightness-125">
						<i class="fa-solid fa-language text-3xl" style="filter:drop-shadow(1px 1px 1px black)"></i>
					</div>
				</button>
			</nav>
		</div>
	}
}
