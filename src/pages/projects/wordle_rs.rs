use yew::prelude::*;

use crate::components::project::ProjectPost;

use crate::localization::{use_localization, Localization};

#[function_component]
pub fn WordleRs() -> Html {
	let localization = use_localization();

	let md_content = match localization.get() {
		Localization::EN => include_str!("../../content/en/projects/2025-12-01_wordle-rs.md"),
		Localization::DE => include_str!("../../content/de/projects/2025-12-01_wordle-rs.md"),
	};

	html! {
		<ProjectPost markdown={md_content} />
	}
}
