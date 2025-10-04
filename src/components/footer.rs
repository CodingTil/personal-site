use yew::prelude::*;

use stylist::yew::styled_component;

#[styled_component]
pub fn Footer() -> Html {
	html! {
		<footer class="mt-auto">
			// Copyright notice
			<div class={css!(r#"
				background: #0F172A;
				padding: 2rem 1rem;
				text-align: center;
				border-top: 1px solid rgba(203, 213, 225, 0.1);
			"#)}>
				<p class={css!(r#"
					color: rgba(203, 213, 225, 0.6);
					font-size: 0.875rem;
					line-height: 1.5;
				"#)}>
					{"© 2025 Til Mohr"}
					{" · "}
					<a href="https://github.com/CodingTil/personal-site" class={css!(r#"
						color: rgba(203, 213, 225, 0.6);
						transition: color 0.2s ease;
						&:hover {
							color: #60A5FA;
						}
					"#)}>
						{"Source Code"}
					</a>
				</p>
			</div>

			// Rainbow bar
			<div class="flex flex-nowrap min-w-full h-3">
				{
					(1..7).into_iter().map(|color| {
						html! {
							<div class={format!("min-h-full w-1/6 bg-rainbow-{}", color)} />
						}
					}).collect::<Html>()
				}
			</div>
		</footer>
	}
}
