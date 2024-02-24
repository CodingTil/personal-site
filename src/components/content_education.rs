use yew::prelude::*;

use stylist::yew::styled_component;

use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

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
	pub border_color: String,
}

#[styled_component]
pub fn ContentEducation(props: &ContentEducationProps) -> Html {
	let md_str = props.markdown.clone();

	// Get Front Matter
	let document = YamlFrontMatter::parse::<Metadata>(&md_str).unwrap();
	let Metadata {
		title,
		education,
		date_range,
		location,
	} = document.metadata;
	let md = document.content;

	html! {
		<ContentItem
			main_header={title}
			right_header={location}
			subtitle={format!("{}<br>{}", education, date_range)}
			markdown={md}
			border_color={props.border_color.clone()}
		/>
	}
}
