use yew::prelude::*;

use stylist::yew::styled_component;

use crate::theme::{use_theme, ThemeKind};

#[styled_component]
pub fn ColorThemePicker() -> Html {
	let theme = use_theme();

	let this_theme = theme.kind().clone();

	let other_theme = match this_theme {
		ThemeKind::Dark => ThemeKind::Dark,
	};

	let switch_theme = Callback::from(move |_| theme.set(other_theme.clone()));

	match this_theme {
		ThemeKind::Dark => html! {
			<button class="btn outline-none focus:outline-none" onclick={switch_theme}>
				<span class="sr-only">{"Switch to light theme"}</span>
				<i class="fa-solid fa-sun text-3xl text-yellow-300 hover:text-yellow-100" style="filter:drop-shadow(1px 1px 1px black)"></i>
			</button>
		},
	}
}
