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

	let header_css = css!(
		r#"
		backdrop-filter: blur(12px) saturate(180%);
		-webkit-backdrop-filter: blur(12px) saturate(180%);
		background-color: rgba(15, 23, 42, 0.85);
		border-bottom: 1px solid rgba(203, 213, 225, 0.1);
		box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
		"#
	);

	html! {
		<div class={classes!("h-20", "sticky", "top-0", "z-50", "w-full", header_css)}>
			<nav class="container mx-auto pt-5 pb-3 max-w-7xl px-4 xl:px-0 flex justify-between items-center">
				<Link<Route> to={Route::Home} classes={css!(r#"
					display: flex;
					align-items: center;
					gap: 0.75rem;
					transition: all 0.2s ease;
				"#)}>
					<div class={css!(r#"
						transition: all 0.2s ease;
						&:hover {
							transform: scale(1.1);
						}
					"#)}>
						<i class="fa-solid fa-trademark text-3xl text-rainbow-1" style="filter:drop-shadow(2px 2px 4px rgba(96, 165, 250, 0.3))"></i>
					</div>
					<span class="text-foreground-primary text-lg font-semibold">{"Home"}</span>
				</Link<Route>>
				<button onclick={switch_locale} class={css!(r#"
					display: flex;
					align-items: center;
					gap: 0.75rem;
					padding: 0.5rem 1rem;
					border-radius: 0.5rem;
					background-color: rgba(51, 65, 85, 0.5);
					transition: all 0.2s ease;
					border: 1px solid rgba(203, 213, 225, 0.1);
					&:hover {
						background-color: rgba(71, 85, 105, 0.6);
						transform: translateY(-2px);
						box-shadow: 0 4px 6px rgba(0, 0, 0, 0.2);
					}
				"#)}>
					<span class="sr-only">{"Switch language to "}</span>
					<span class="text-foreground-primary text-lg font-semibold">{other_locale_str}</span>
					<i class="fa-solid fa-language text-2xl text-rainbow-1"></i>
				</button>
			</nav>
		</div>
	}
}
