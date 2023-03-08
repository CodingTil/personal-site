use stylist::style;
use yew::prelude::*;

use stylist::yew::styled_component;

use crate::theme::use_theme;

use crate::components::logo::Logo;

#[styled_component]
pub fn Header() -> Html {
	let theme = use_theme();

	let outter_css = style!(
		r#"
		background-color: ${bg_1};
		border-color: ${bg_2};
		"#,
		bg_1 = theme.background_primary.clone(),
		bg_2 = theme.background_secondary.clone(),
	)
	.unwrap();

	html! {
		<div class={String::from("sticky top-0 z-50 w-full border-solid border-b-2 h-20") + " " + outter_css.get_class_name()}>
			<nav class={String::from("container mx-auto max-w-7xl pt-6 pb-4 px-4 xl:px-0 flex content-center justify-between items-center")}>
				<Logo />
				<div class={String::from("justify-self-end inline-flex space-x-2 content-center items-center")}>

				</div>
			</nav>
		</div>
	}
}
