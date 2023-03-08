use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BaseLayoutProps {
	#[prop_or_default]
	pub children: Children,
}

#[function_component]
pub fn BaseLayout(props: &BaseLayoutProps) -> Html {
	html! {
		<div>
			<div>{ for props.children.iter() }</div>
			<footer class={classes!("mt-auto", "flex", "flex-nowrap", "min-w-full", "h-3", "z-40")}>
				<div class={classes!("min-h-full", "w-1/6", "bg-rainbow-1")} />
				<div class={classes!("min-h-full", "w-1/6", "bg-rainbow-2")} />
				<div class={classes!("min-h-full", "w-1/6", "bg-rainbow-3")} />
				<div class={classes!("min-h-full", "w-1/6", "bg-rainbow-4")} />
				<div class={classes!("min-h-full", "w-1/6", "bg-rainbow-5")} />
				<div class={classes!("min-h-full", "w-1/6", "bg-rainbow-6")} />
			</footer>
		</div>
	}
}
