use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

use crate::router::Route;
use crate::theme::use_theme;

#[styled_component]
pub fn Logo() -> Html {
	let theme = use_theme();

	html! {
		<Link<Route> to={Route::Home}>
			<div class="justify-self-start inline-flex content-center items-center space-x-2">
				<div class={css!(r#"
					color: ${rainbow_1};
					filter: drop-shadow(1px 1px 1px black);
				"#,rainbow_1 = theme.rainbow_1.clone())}>
					<i class="fa-solid fa-trademark text-3xl"></i>
				</div>
				<div class={css!(r#"
					color: ${fg};
				"#, fg = theme.foreground_primary.clone())}>
					{"Home"}
				</div>
			</div>
		</Link<Route>>
	}
}
