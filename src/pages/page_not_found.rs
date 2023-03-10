use std::borrow::Cow;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
	localization::{use_localization, Localization},
	router::Route,
};

struct Translations {
	sorry: Cow<'static, str>,
	we_couldnt_find: Cow<'static, str>,
	back_home: Cow<'static, str>,
}

const EN_TRANSLATIONS: Translations = Translations {
	sorry: Cow::Borrowed("Sorry, we couldn't find this page."),
	we_couldnt_find: Cow::Borrowed(
		"But dont worry, you can find plenty of other things on the homepage.",
	),
	back_home: Cow::Borrowed("Back to homepage"),
};

const DE_TRANSLATIONS: Translations = Translations {
	sorry: Cow::Borrowed("Entschuldigung, wir konnten diese Seite nicht finden."),
	we_couldnt_find: Cow::Borrowed(
		"Aber keine Sorge, auf der Homepage finden Sie viele andere Dinge.",
	),
	back_home: Cow::Borrowed("Zurück zur Startseite"),
};

#[function_component]
pub fn PageNotFound() -> Html {
	let localization = use_localization();

	let translation = match localization.get() {
		Localization::EN => &EN_TRANSLATIONS,
		Localization::DE => &DE_TRANSLATIONS,
	};

	html! {
		<section id="404-page" class="flex items-center h-full p-16">
			<div class="container flex flex-col items-center justify-center px-5 mx-auto my-8">
				<div class="max-w-md text-center">
					<h2 class="mb-8 font-extrabold text-9xl text-foreground-tertiary">
						<span class="sr-only">
							{"Error"}
						</span>
						{"404"}
					</h2>
					<p class="text-2xl font-semibold md:text-3xl text-foreground-primary">
						{translation.sorry.clone()}
					</p>
					<p class="mt-4 mb-8 text-foreground-secondary">
						{translation.we_couldnt_find.clone()}
					</p>
					<Link<Route> to={Route::Home}>
						<div rel="noopener noreferrer" href="#" class="px-8 py-3 font-semibold rounded bg-violet-400">
							{translation.back_home.clone()}
						</div>
					</Link<Route>>
				</div>
			</div>
		</section>
	}
}
