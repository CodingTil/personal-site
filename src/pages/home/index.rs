use yew::prelude::*;
use crate::layouts::base::BaseLayout;

#[function_component]
pub fn Home() -> Html {
	let counter = use_state(|| 0);
	let onclick = {
		let counter = counter.clone();
		move |_| {
			let value = *counter + 1;
			counter.set(value);
		}
	};

	html! {
		<BaseLayout>
			<button class={classes!("bg-rainbow-4", "text-xl", "text-sky-400")} {onclick}>{ "+1" }</button>
			<p class={classes!("text-rainbow-3")}>{ *counter }</p>
		</BaseLayout>
	}
}
