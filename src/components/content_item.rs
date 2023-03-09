use yew::prelude::*;

use stylist::yew::styled_component;

use comrak::{format_html, parse_document, Arena, ComrakOptions};
use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

use crate::safehtml::SafeHtml;

#[derive(Debug, PartialEq, Properties)]
pub struct ContentEducationProps {
	pub main_header: String,
	pub right_header: String,
	pub subtitle: String,
	#[prop_or_default]
	pub link: Option<String>,
	pub markdown: String,
}

#[styled_component]
pub fn ContentItem(props: &ContentEducationProps) -> Html {
	let md = props.markdown.clone().as_str().trim();

	// Render html
	let arena = Arena::new();
	let mut options = ComrakOptions::default();
	options.extension.front_matter_delimiter = Some("---".to_string());
	options.render.unsafe_ = true;
	let root = parse_document(&arena, &md, &options);
	let mut md_html_vec = vec![];
	format_html(root, &ComrakOptions::default(), &mut md_html_vec).unwrap();
	let md_html = String::from_utf8(md_html_vec).unwrap();

	html! {
		<div class="my-6 pl-5 py-4 border-l-8 border-solid">
			<div class="block xl:flex xl:justify-between text-foreground-primary">
				<div class="text-xl font-bold text-left">
					{
						match props.link {
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
			<SafeHtml html={md_html} />
		</div>
	}

}