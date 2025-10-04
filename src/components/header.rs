use gloo::timers::callback::Timeout;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

use stylist::yew::styled_component;

use crate::localization::{use_localization, Localization};
use crate::router::Route;

fn scroll_to_section(section_id: &str) {
	if let Some(window) = window() {
		if let Some(document) = window.document() {
			if let Some(element) = document.get_element_by_id(section_id) {
				element.scroll_into_view();
			}
		}
	}
}

#[styled_component]
pub fn Header() -> Html {
	let localization = use_localization();
	let navigator = use_navigator().unwrap();
	let location = use_location().unwrap();

	let current_locale = localization.get().clone();

	let other_locale = match current_locale {
		Localization::DE => Localization::EN,
		Localization::EN => Localization::DE,
	};

	let other_locale_str = other_locale.to_string();

	let switch_locale = Callback::from(move |_| localization.set(other_locale.clone()));

	// Get translations for navigation items
	let (projects_text, experience_text, education_text, contact_text) = match current_locale {
		Localization::EN => ("Projects", "Experience", "Education", "Contact"),
		Localization::DE => ("Projekte", "Erfahrung", "Bildung", "Kontakt"),
	};

	// Check if we're on the home page
	let is_home = location.path() == "/";

	// Create navigation callbacks for sections
	let nav_to_section = {
		let navigator = navigator.clone();
		Callback::from(move |section: String| {
			let section_clone = section.clone();
			if is_home {
				// Already on home, just scroll
				scroll_to_section(&section);
			} else {
				// Navigate to home first
				navigator.push(&Route::Home);
				// Wait a bit for the DOM to update, then scroll
				Timeout::new(100, move || {
					scroll_to_section(&section_clone);
				})
				.forget();
			}
		})
	};

	let on_projects_click = {
		let nav = nav_to_section.clone();
		Callback::from(move |e: MouseEvent| {
			e.prevent_default();
			nav.emit("projects".to_string());
		})
	};

	let on_experience_click = {
		let nav = nav_to_section.clone();
		Callback::from(move |e: MouseEvent| {
			e.prevent_default();
			nav.emit("experience".to_string());
		})
	};

	let on_education_click = {
		let nav = nav_to_section.clone();
		Callback::from(move |e: MouseEvent| {
			e.prevent_default();
			nav.emit("education".to_string());
		})
	};

	let on_contact_click = {
		let nav = nav_to_section.clone();
		Callback::from(move |e: MouseEvent| {
			e.prevent_default();
			nav.emit("contact".to_string());
		})
	};

	html! {
		<div class={css!(r#"
			position: fixed;
			top: 0;
			left: 0;
			right: 0;
			z-index: 1000;
			backdrop-filter: blur(12px) saturate(180%);
			-webkit-backdrop-filter: blur(12px) saturate(180%);
			background-color: rgba(15, 23, 42, 0.8);
		"#)}>
			<nav class={css!(r#"
				max-width: 80rem;
				margin: 0 auto;
				padding: 1.25rem 2rem;
				display: flex;
				justify-content: space-between;
				align-items: center;
				@media (max-width: 768px) {
					padding: 1rem 1.5rem;
				}
			"#)}>
				<Link<Route> to={Route::Home} classes={css!(r#"
					display: flex;
					align-items: center;
					gap: 0.75rem;
					color: #F1F5F9;
					font-weight: 700;
					font-size: 1.125rem;
					transition: all 0.2s ease;
					font-family: 'Space Grotesk', sans-serif;
					&:hover {
						color: #60A5FA;
					}
				"#)}>
					<span>{"TM"}</span>
				</Link<Route>>

				<div class={css!(r#"
					display: flex;
					gap: 2rem;
					align-items: center;
				"#)}>
					<a href="#projects" onclick={on_projects_click} class={css!(r#"
						color: rgba(203, 213, 225, 0.8);
						font-weight: 500;
						transition: color 0.2s ease;
						display: flex;
						align-items: center;
						line-height: 1;
						cursor: pointer;
						&:hover {
							color: #F1F5F9;
						}
						@media (max-width: 640px) {
							display: none;
						}
					"#)}>
						{projects_text}
					</a>
					<a href="#experience" onclick={on_experience_click} class={css!(r#"
						color: rgba(203, 213, 225, 0.8);
						font-weight: 500;
						transition: color 0.2s ease;
						display: flex;
						align-items: center;
						line-height: 1;
						cursor: pointer;
						&:hover {
							color: #F1F5F9;
						}
						@media (max-width: 640px) {
							display: none;
						}
					"#)}>
						{experience_text}
					</a>
					<a href="#education" onclick={on_education_click} class={css!(r#"
						color: rgba(203, 213, 225, 0.8);
						font-weight: 500;
						transition: color 0.2s ease;
						display: flex;
						align-items: center;
						line-height: 1;
						cursor: pointer;
						&:hover {
							color: #F1F5F9;
						}
						@media (max-width: 640px) {
							display: none;
						}
					"#)}>
						{education_text}
					</a>
					<a href="#contact" onclick={on_contact_click} class={css!(r#"
						color: rgba(203, 213, 225, 0.8);
						font-weight: 500;
						transition: color 0.2s ease;
						display: flex;
						align-items: center;
						line-height: 1;
						cursor: pointer;
						&:hover {
							color: #F1F5F9;
						}
						@media (max-width: 640px) {
							display: none;
						}
					"#)}>
						{contact_text}
					</a>

					<button onclick={switch_locale} class={css!(r#"
						padding: 0.5rem 1rem;
						border-radius: 0.5rem;
						background-color: rgba(51, 65, 85, 0.5);
						color: #F1F5F9;
						font-weight: 500;
						border: 1px solid rgba(203, 213, 225, 0.1);
						transition: all 0.2s ease;
						display: flex;
						align-items: center;
						line-height: 1;
						&:hover {
							background-color: rgba(71, 85, 105, 0.7);
							border-color: rgba(96, 165, 250, 0.3);
						}
					"#)}>
						{other_locale_str}
					</button>
				</div>
			</nav>
		</div>
	}
}
