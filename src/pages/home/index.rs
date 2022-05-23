use yew::prelude::*;

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
		<div>
			<button class={classes!("bg-red-900")} {onclick}>{ "+1" }</button>
			<p class={classes!("bg-red-900")}>{ *counter }</p>
		</div>
	}
}
