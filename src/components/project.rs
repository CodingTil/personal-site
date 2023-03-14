use comrak::{format_html, parse_document, Arena, ComrakOptions};
use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

use crate::router::Route;

use crate::safehtml::SafeHtml;
use crate::theme::use_theme;

#[derive(Deserialize)]
pub struct ProjectMetadata {
	pub slug: String,
	pub image: String,
	pub title: String,
	pub color: String,
	pub tagline: String,
	pub url: String,
	pub date_range: String,
	pub skills: Vec<String>,
	pub filters: Vec<String>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct ProjectCardProps {
	pub markdown: String,
}

#[styled_component]
pub fn ProjectCard(props: &ProjectCardProps) -> Html {
	let md_str = props.markdown.clone();

	// Get Front Matter
	let document = YamlFrontMatter::parse::<ProjectMetadata>(&md_str).unwrap();
	let front_matter = document.metadata;
	let ProjectMetadata {
		slug,
		image,
		title,
		color,
		tagline,
		url: _,
		date_range: _,
		skills: _,
		filters: _,
	} = front_matter;

	let size_css = style!(
		r#"
		@media (min-width: 640px) {
			max-width: 100%;
		}
		@media (min-width: 768px) {
			max-width: 45%;
		}
		@media (min-width: 1024px) {
			max-width: 30%;
		}
	"#
	)
	.unwrap();
	let hide_css = style!(
		r#"
		:hover .hidden {
			position: absolute;
			top: 0;
			left: 0;
			width: 100%;
			height: 100%;
			overflow: hidden;
			display: inline-block;
		}
	"#
	)
	.unwrap();

	let to_route = match slug.as_str() {
		"flappyking" => Route::FlappyKing,
		"simplechat" => Route::SimpleChat,
		"fractal" => Route::Fractal,
		_ => panic!("Invalid slug: {}", slug),
	};

	html! {
		<div class={String::from("py-3 overflow-auto mx-2 ") + size_css.get_class_name()}>
			<Link<Route> to={to_route}>
				<div class={String::from("group h-auto relative mb-3 inline-block ") + hide_css.get_class_name()}>
					<div class="opacity-100 group-hover:opacity-0">
						<SafeHtml html={image.clone()} />
					</div>
					<div class={String::from("hidden text-white font-medium p-5 hover:") + &color}>
						{tagline.clone()}
					</div>
				</div>
				<br />
				<span class="text-foreground-primary font-medium">
					{title.clone()}
				</span>
			</Link<Route>>
		</div>
	}
}

#[derive(Debug, PartialEq, Properties)]
pub struct ProjectPostProps {
	pub markdown: String,
}

#[styled_component]
pub fn ProjectPost(props: &ProjectCardProps) -> Html {
	let theme = use_theme();

	let md_str = props.markdown.clone();

	// Get Front Matter
	let document = YamlFrontMatter::parse::<ProjectMetadata>(&md_str).unwrap();
	let front_matter = document.metadata;
	let md = document.content;
	let ProjectMetadata {
		slug: _,
		image: _,
		title,
		color,
		tagline,
		url,
		date_range,
		skills: _,
		filters: _,
	} = front_matter;

	let arena = Arena::new();
	let mut options = ComrakOptions::default();
	options.extension.front_matter_delimiter = Some("---".to_string());
	options.render.unsafe_ = true;
	options.extension.autolink = true;
	options.extension.table = true;
	options.extension.strikethrough = true;
	options.extension.tasklist = true;
	let root = parse_document(&arena, &md, &options);
	let mut md_html_vec = vec![];
	format_html(root, &options, &mut md_html_vec).unwrap();
	let md_html = String::from_utf8(md_html_vec).unwrap();

	let prose_content_css = style!(
		r#"
	width: 100%;
	margin-top: 0.75rem;
	margin-bottom: 0.75rem;
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
		margin-top: 2rem;
		margin-bottom: 0px;
		border-bottom-width: 1px;
		--tw-border-opacity: 1;
		border-color: ${fg};
		padding-bottom: 0.25rem;
	}

	h2 {
		margin-top: 1rem;
		margin-bottom: -0.25rem;
	}

	hr {
		--tw-border-opacity: 1;
		border-color: ${fg};
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
	}

	.cell {
		float: left;
		margin-left: 0px;
		margin-right: 0px;
		margin-bottom: 1rem;
		margin-top: -2rem;
		min-width: 100%;
	}

	@media (min-width: 768px) {
		.cell {
			width: 50%;
			min-width: 0px;
		}
	}

	/* used for "last cell" - workaround */

	.cell-non-md {
		float: left;
		margin-left: 0px;
		margin-right: 0px;
		margin-bottom: 1rem;
		margin-top: -2rem;
		min-width: 100%;
	}
	"#,
		fg = theme.foreground_primary.clone(),
		fg2 = theme.foreground_secondary.clone()
	)
	.unwrap();

	html! {
		<div class="container mx-0 my-0 w-screen min-w-full">
			<div class="mx-auto px-4 xl:px-0 mt-4 mb-5 max-w-7xl">
				<div class="flex flex-row justify-center md:justify-between items-center flex-wrap md:flex-nowrap">
					<div class="mb-4">
						<h1 class="text-foreground-primary text-4xl font-bold mb-2">
							{title.clone()}
						</h1>
						<h3 class="text-foreground-secondary text-2xl font-bold">
							{tagline.clone()}
						</h3>
					</div>

					<div class="border-t-2 md:border-t-0 border-solid border-foreground-tertiary p-2 md:pr-0">
						<div class="flex flex-row md:flex-col flex-wrap justify-center md:justify-start">
							<div class="mx-2 flex items-center text-foreground-primary">
								<i class="fa-solid fa-calendar-days"></i>
								<span class="ml-2 whitespace-nowrap">
									{date_range.clone()}
								</span>
							</div>
							<a href={url.clone()} class="mx-2 flex items-center text-foreground-primary">
								<i class="fa-brands fa-square-github"></i>
								<span class="ml-2 whitespace-nowra">
									{"git"}
								</span>
							</a>
						</div>
					</div>
				</div>
			</div>

			<div class={String::from("h-2 w-full ") + &color} />

			<div class="mt-8 mb-14 px-4 xl:px-0 mx-auto max-w-7xl">
				<div class={String::from(prose_content_css.get_class_name()) + " prose"}>
					<SafeHtml html={md_html.clone()} />
				</div>
			</div>
		</div>
	}
}
