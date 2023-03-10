use yew::prelude::*;

use stylist::yew::styled_component;

use crate::localization::{use_localization, Localization};

#[styled_component]
pub fn LanguageSwitcher() -> Html {
	let localization = use_localization();

	let current_locale = localization.get().clone();

	let other_locale = match current_locale {
		Localization::DE => Localization::EN,
		Localization::EN => Localization::DE,
	};

	let other_locale_str = other_locale.to_string();

	let switch_locale = Callback::from(move |_| localization.set(other_locale.clone()));

	html! {
		<button onclick={switch_locale} class="text-foreground-secondary">
			<span class="sr-only">{"Switch language to "}</span>
			{other_locale_str}
		</button>
	}
}
