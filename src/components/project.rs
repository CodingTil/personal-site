use stylist::style;
use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

use crate::router::Route;

use crate::safehtml::SafeHtml;

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
	let md = document.content;
	let ProjectMetadata {
		slug,
		image,
		title,
		color,
		tagline,
		url,
		date_range,
		skills,
		filters,
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

	html! {
		<div class={String::from("py-3 overflow-auto mx-2 ") + size_css.get_class_name()}>
			<Link<Route> to={Route::Home}>
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
