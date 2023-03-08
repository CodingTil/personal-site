use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;

#[derive(Properties, PartialEq)]
pub struct BaseLayoutProps {
	#[prop_or_default]
	pub children: Children,
}

#[function_component]
pub fn BaseLayout(props: &BaseLayoutProps) -> Html {
	html! {
		<div>
			<Header />
			<div>{ for props.children.iter() }</div>
			<Footer />
		</div>
	}
}
