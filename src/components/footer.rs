use yew::prelude::*;

use stylist::yew::styled_component;

use crate::theme::use_theme;

#[styled_component]
pub fn Footer() -> Html {
	let theme = use_theme();
	let colors = [
		theme.rainbow_1.clone(),
		theme.rainbow_2.clone(),
		theme.rainbow_3.clone(),
		theme.rainbow_4.clone(),
		theme.rainbow_5.clone(),
		theme.rainbow_6.clone(),
	];
	html! {
		<footer class={css!(r#"
			margin-top: auto;
			margin-bottom: 0;
			display: flex;
			flex-wrap: nowrap;
			min-width: 100%;
			height: 0.75rem;
		"#)}>
			{
				colors.into_iter().map(|color| {
					html! {
						<div class={css!(r#"
							min-height: 100%;
							width: 16.7%;
							background-color: ${bg};
						"#, bg = color)} />
					}
				}).collect::<Html>()
			}
		</footer>
	}
}
