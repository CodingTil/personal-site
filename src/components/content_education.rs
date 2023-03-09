use yew::prelude::*;

use stylist::yew::styled_component;

use comrak::{format_html, parse_document, Arena, ComrakOptions};
use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

use crate::safehtml::SafeHtml;

use crate::components::content_item::ContentItem;

#[derive(Deserialize)]
struct Metadata {
	title: String,
	education: String,
	date_range: String,
	location: String,
}

#[derive(Debug, PartialEq, Properties)]
pub struct ContentEducationProps {
	pub markdown: String,
}

#[styled_component]
pub fn ContentEducation(props: &ContentEducationProps) -> Html {
	let md_str = props.markdown.clone().as_str().trim();

	// Get Front Matter
	let document = YamlFrontMatter::parse::<Metadata>(&md_str).unwrap();
	let front_matter = document.metadata;
	let md = document.content;
	let Metadata { title, education, date_range, location } = front_matter;

	html! {
		<ContentItem
			main_header={title}
			right_header={location}
			subtitle={format!("{}<br>{}", education, date_range)}
			markdown={md}
		/>
	}

}