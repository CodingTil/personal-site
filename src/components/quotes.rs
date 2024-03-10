use std::fmt::Display;

use yew::prelude::*;

use stylist::yew::styled_component;

use serde::Deserialize;

use crate::components::typewriter::Typewriter;
use rand::seq::SliceRandom;

#[derive(Deserialize)]
struct Quote {
	quote: String,
	author: String,
}

impl Display for Quote {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\"{}\"\n\t - {}", self.quote, self.author)
	}
}

#[derive(Debug, PartialEq, Properties)]
pub struct QuotesProps {
	pub file_content: String,
}

#[styled_component]
pub fn Quotes(props: &QuotesProps) -> Html {
	let mut document = serde_yaml::from_str::<Vec<Quote>>(&props.file_content).unwrap().iter().map(|quote| quote.to_string()).collect::<Vec<String>>();
	document.shuffle(&mut rand::thread_rng());

	html! {
		<Typewriter texts={Vec::from(document)} class="text-wrap text-clip overflow-hidden max-w-full text-3xl font-black italic text-center" />
	}
}
