use yew::prelude::*;

use stylist::yew::styled_component;

use crate::components::typewriter::Typewriter;
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq, Properties)]
pub struct TagsProps {
	pub file_content: String,
}

#[styled_component]
pub fn Tags(tags: &TagsProps) -> Html {
	let mut document = serde_yaml::from_str::<Vec<String>>(&tags.file_content)
		.unwrap()
		.iter()
		.map(|tag| tag.to_string())
		.collect::<Vec<String>>();
	document.shuffle(&mut rand::thread_rng());

	html! {
		<Typewriter texts={Vec::from(document)} class="text-wrap text-clip overflow-hidden max-w-full text-3xl font-black italic text-center" />
	}
}
