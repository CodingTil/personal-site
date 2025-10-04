use yew::prelude::*;

use stylist::yew::styled_component;

#[derive(Debug, PartialEq, Properties)]
pub struct TagsProps {
	pub file_content: String,
}

#[styled_component]
pub fn Tags(tags: &TagsProps) -> Html {
	let document = serde_yaml::from_str::<Vec<String>>(&tags.file_content)
		.unwrap()
		.iter()
		.map(|tag| tag.to_string())
		.collect::<Vec<String>>();

	let tags_text = document.join(" • ");

	html! {
		<div class={css!(r#"
			font-size: 1.5rem;
			font-weight: 600;
			font-style: italic;
			text-align: center;
			max-width: 100%;
			line-height: 2rem;
			color: rgba(226, 232, 240, 0.9);
			@media (min-width: 768px) {
				font-size: 1.875rem;
				line-height: 2.5rem;
			}
		"#)}>
			{tags_text}
		</div>
	}
}
