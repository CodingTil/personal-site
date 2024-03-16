use yew::prelude::*;

use crate::components::project::ProjectPost;

use crate::localization::{use_localization, Localization};

#[function_component]
pub fn OceanCurrents() -> Html {
	let localization = use_localization();

	let md_content = match localization.get() {
		Localization::EN => include_str!("../../content/en/projects/2021-10-12_OceanCurrents.md"),
		Localization::DE => include_str!("../../content/de/projects/2021-10-12_OceanCurrents.md"),
	};

	html! {
		<ProjectPost markdown={md_content} />
	}
}
