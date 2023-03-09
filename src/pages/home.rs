use std::borrow::Cow;

use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

use comrak::{format_html, parse_document, Arena, ComrakOptions};
use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

use crate::localization::{use_localization, Localization};
use crate::theme::use_theme;

use crate::safehtml::SafeHtml;

struct Translations {
	projects: Cow<'static, str>,
	more: Cow<'static, str>,
	experience: Cow<'static, str>,
	teaching: Cow<'static, str>,
	education: Cow<'static, str>,
}

const EN_TRANSLATIONS: Translations = Translations {
	projects: Cow::Borrowed("Projects"),
	more: Cow::Borrowed("More About Me"),
	experience: Cow::Borrowed("Experience"),
	teaching: Cow::Borrowed("Teaching"),
	education: Cow::Borrowed("Education"),
};

const DE_TRANSLATIONS: Translations = Translations {
	projects: Cow::Borrowed("Projekte"),
	more: Cow::Borrowed("Mehr Über Mich"),
	experience: Cow::Borrowed("Erfahrung"),
	teaching: Cow::Borrowed("Lehre"),
	education: Cow::Borrowed("Bildung"),
};

#[styled_component]
pub fn Home() -> Html {
	let theme = use_theme();
	let localization = use_localization();

	let intro_border_css = style!(
		r#"
		border-color: ${bg};
		"#,
		bg = theme.foreground_tertiary.clone(),
	)
	.unwrap();

	let translation = match localization.get() {
		Localization::EN => &EN_TRANSLATIONS,
		Localization::DE => &DE_TRANSLATIONS,
	};

	let intro_md_str = match localization.get() {
		Localization::EN => include_str!("../content/en/intro.md"),
		Localization::DE => include_str!("../content/de/intro.md"),
	};
	let more_md_str = match localization.get() {
		Localization::EN => include_str!("../content/en/more.md"),
		Localization::DE => include_str!("../content/de/more.md"),
	};
	let arena = Arena::new();
	let mut options = ComrakOptions::default();
	options.render.unsafe_ = true;
	let intro_root = parse_document(&arena, &intro_md_str, &options);
	let more_root = parse_document(&arena, &more_md_str, &options);
	let mut intro_html_vec = vec![];
	let mut more_html_vec = vec![];
	format_html(intro_root, &options, &mut intro_html_vec).unwrap();
	format_html(more_root, &options, &mut more_html_vec).unwrap();
	let intro_html = String::from_utf8(intro_html_vec).unwrap();
	let more_html = String::from_utf8(more_html_vec).unwrap();

	let cv_section_css = String::from("container mx-0 my-0 w-screen min-w-full");
	let section_box_css = String::from(
		"flex flex-col justify-center items-center py-8 px-4 xl:px-0 mx-0 xl:mx-auto max-w-7xl",
	);
	let section_title_css = String::from(
		"inline-block px-6 py-4 w-min font-bold text-xl whitespace-nowrap border border-solid",
	);

	html! {
		<div class="min-w-full">
			/* Intro */
			<section id="intro" class={cv_section_css.clone()}>
				<div class={section_box_css.clone()}>
					<div class="flex flex-row justify-center items-center flex-wrap xl:flex-nowrap" >
						<div class={String::from("border-r-0 xl:border-r-2 border-solid pr-2 ") + intro_border_css.get_class_name()}>
							<SafeHtml html={intro_html} />
						</div>
						<div class={String::from("border-t-2 xl:border-t-0 border-solid p-2 xl:pr-0") + " " + intro_border_css.get_class_name()}>
							<div class="flex flex-row xl:flex-col flex-wrap justify-center xl:justify-start">
								<a href="mailto:me@tilmohr.com" class="mx-2 flex items-center text-foreground-primary">
									<i class="fa-solid fa-envelope"></i>
									<span class="ml-2 whitespace-nowrap">{"me@tilmohr.com"}</span>
								</a>
								<a href="https://github.com/CodingTil" class="mx-2 flex items-center text-foreground-primary">
									<i class="fa-brands fa-square-github"></i>
									<span class="ml-2 whitespace-nowrap" >{"CodingTil"}</span>
								</a>
							</div>
						</div>
					</div>
				</div>
			</section>

			/* Project Cards */

			/* More About Me */
			<section id="more_about_me" class={String::from("bg-foreground-tertiary text-background-primary ") + &cv_section_css}>
				<div class={section_box_css.clone()}>
					<div class={String::from("border-background-primary ") + &section_title_css}>
						{ translation.more.clone() }
					</div>
					<SafeHtml html={more_html} />
				</div>
			</section>

		</div>
	}
}
