use yew::prelude::*;

use stylist::yew::styled_component;

#[styled_component]
pub fn Footer() -> Html {
	html! {
		<footer class="mt-auto flex flex-nowrap min-w-full h-3 z-40">
			{
				(1..7).into_iter().map(|color| {
					html! {
						<div class={format!("min-h-full w-1/6 bg-rainbow-{}", color)} />
					}
				}).collect::<Html>()
			}
		</footer>
	}
}
