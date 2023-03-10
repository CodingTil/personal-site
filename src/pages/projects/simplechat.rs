use yew::prelude::*;

use crate::components::project::ProjectPost;

use crate::localization::{use_localization, Localization};

#[function_component]
pub fn SimpleChat() -> Html {
	let localization = use_localization();

	let md_content = match localization.get() {
		Localization::EN => include_str!("../../content/en/projects/2021-02-01_SimpleChat.md"),
		Localization::DE => include_str!("../../content/de/projects/2021-02-01_SimpleChat.md"),
	};

	html! {
		<ProjectPost markdown={md_content} />
	}
}
