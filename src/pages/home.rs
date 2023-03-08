use stylist::yew::styled_component;
use yew::prelude::*;

use crate::theme::use_theme;

#[styled_component]
pub fn Home() -> Html {
	let theme = use_theme();

	html! {
		<div>
		</div>
	}
}
