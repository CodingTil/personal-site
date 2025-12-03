use yew::prelude::*;

use stylist::{style, yew::styled_component};

use comrak::{format_html, parse_document, Arena, Options};

use crate::{safehtml::SafeHtml, theme::use_theme};

#[derive(Debug, PartialEq, Properties)]
pub struct ContentEducationProps {
	pub main_header: String,
	pub right_header: String,
	pub subtitle: String,
	#[prop_or_default]
	pub link: Option<String>,
	pub markdown: String,
	pub border_color: String,
}

#[styled_component]
pub fn ContentItem(props: &ContentEducationProps) -> Html {
	let theme = use_theme();

	let md = props.markdown.clone();

	// Render html
	let arena = Arena::new();
	let mut options = Options::default();
	options.extension.front_matter_delimiter = Some("---".to_string());
	options.render.r#unsafe = true;
	options.extension.table = true;
	options.extension.strikethrough = true;
	options.extension.tasklist = true;
	let root = parse_document(&arena, &md, &options);
	let mut md_html = String::new();
	format_html(root, &options, &mut md_html).unwrap();

	let prose_content_css = style!(
		r#"
	width: 100%;
	margin-top: 0.75rem;
	margin-bottom: 0.75rem;
	min-height: fit-content;
	--tw-text-opacity: 1;
	color: ${fg} !important;

	@media (min-width: 640px) {
		max-width: 640px;
	}

	@media (min-width: 768px) {
		max-width: 768px;
	}

	@media (min-width: 1024px) {
		max-width: 1024px;
	}

	@media (min-width: 1280px) {
		max-width: 1280px;
	}

	@media (min-width: 1536px) {
		max-width: 1536px;
	}

	img {
		position: relative;
		margin-left: auto;
		margin-right: auto;
		max-width: 100%;
		overflow: hidden;
		padding-left: 0px;
		padding-right: 0.5rem;
	}

	h1, h2, h3, h4, h5, h6, th, td, a, p {
		--tw-text-opacity: 1;
		color: ${fg} !important;
		-webkit-text-decoration-line: none;
		text-decoration-line: none;
	}

	h1 {
		margin-top: 0rem;
		margin-bottom: 0px;
		border-bottom-width: 1px;
		--tw-border-opacity: 1;
		border-color: ${fg};
		padding-bottom: 0.25rem;
	}

	h2 {
		margin-top: 0rem;
		margin-bottom: -0.25rem;
	}

	h3 {
		margin-top: 0rem;
		margin-bottom: 0rem;
	}

	h4 {
		margin-top: 0rem;
		margin-bottom: 0rem;
	}

	hr {
		--tw-border-opacity: 1;
		border-color: ${fg};
		margin-top: 0rem;
		margin-bottom: 0rem;
	}

	a.my-a {
		-webkit-text-decoration-line: none;
				text-decoration-line: none;
	}

	p {
		margin-top: 0.75rem;
		margin-bottom: 0.75rem;
		line-height: 1.5rem;
	}

	ul {
		margin-top: -0.25rem;
		padding-left: 1.75rem;
		padding-right: 1.75rem;
	}

	li {
		margin-bottom: 0.25rem;
		line-height: 1.5rem;
	}

	sup {
		--tw-text-opacity: 1;
		color:${fg2};
		text-wrap: nowrap;
	}

	.cell {
		margin-left: 0px;
		margin-right: 0px;
		margin-bottom: 1rem;
		margin-top: 1rem;
		min-width: 100%;
	}

	@media (min-width: 768px) {
		.cell {
			width: 50%;
			min-width: 0px;
		}
	}
	"#,
		fg = theme.foreground_primary.clone(),
		fg2 = theme.foreground_secondary.clone()
	)
	.unwrap();

	html! {
		<div class={String::from("my-6 pl-5 py-4 border-l-8 border-solid ") + props.border_color.as_str()}>
			<div class="block xl:flex xl:justify-between text-foreground-primary">
				<div class="text-xl font-bold text-left">
					{
						match props.link.clone() {
							Some(link) => {
								html! {
									<a href={link} class="no-underline text-foreground-primary">
										{props.main_header.clone()}
									</a>
								}
							},
							None => {
								html! {
									{props.main_header.clone()}
								}
							}
						}
					}
				</div>
				<div class="text-left xl:text-right">
					{props.right_header.clone()}
				</div>
			</div>
			<div class="text-foreground-secondary">
				<SafeHtml html={props.subtitle.clone()} />
			</div>
			<div class={String::from(prose_content_css.get_class_name()) + " prose"}>
				<SafeHtml html={md_html.clone()} />
			</div>
		</div>
	}
}
