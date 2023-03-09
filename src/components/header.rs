use yew::prelude::*;

use stylist::yew::styled_component;

use crate::components::{
	color_theme_picker::ColorThemePicker, language_switcher::LanguageSwitcher, logo::Logo,
};

#[styled_component]
pub fn Header() -> Html {
	html! {
		<div class="sticky top-0 z-50 w-full border-solid border-b-2 h-20 bg-background-primary border-background-secondary">
			<nav class="container mx-auto max-w-7xl pt-6 pb-4 px-4 xl:px-0 flex content-center justify-between items-center">
				<Logo />
				<div class="justify-self-end inline-flex space-x-2 content-center items-center">
					<LanguageSwitcher />
					<ColorThemePicker />
				</div>
			</nav>
		</div>
	}
}
