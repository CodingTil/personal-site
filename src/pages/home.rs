use std::borrow::Cow;

use stylist::yew::styled_component;
use yew::prelude::*;

use comrak::{format_html, parse_document, Arena, ComrakOptions};

use include_dir::{include_dir, Dir};

use crate::localization::{use_localization, Localization};

use crate::safehtml::SafeHtml;

use crate::components::content_education::ContentEducation;
use crate::components::content_teaching::ContentTeaching;
use crate::components::project::ProjectCard;

struct Translations {
	projects: Cow<'static, str>,
	more: Cow<'static, str>,
	teaching: Cow<'static, str>,
	education: Cow<'static, str>,
}

const EN_TRANSLATIONS: Translations = Translations {
	projects: Cow::Borrowed("Projects"),
	more: Cow::Borrowed("More About Me"),
	teaching: Cow::Borrowed("Teaching"),
	education: Cow::Borrowed("Education"),
};

const DE_TRANSLATIONS: Translations = Translations {
	projects: Cow::Borrowed("Projekte"),
	more: Cow::Borrowed("Mehr Über Mich"),
	teaching: Cow::Borrowed("Lehre"),
	education: Cow::Borrowed("Bildung"),
};

fn border_color(index: usize) -> String {
	let mod_index = index % 6;
	let color = match mod_index {
		0 => "border-rainbow-1",
		1 => "border-rainbow-2",
		2 => "border-rainbow-3",
		3 => "border-rainbow-4",
		4 => "border-rainbow-5",
		5 => "border-rainbow-6",
		_ => "border-rainbow-1",
	};
	String::from(color)
}

static CONTENT_DE_EDUCATION_DIR: Dir = include_dir!("src/content/de/education/");
static CONTENT_EN_EDUCATION_DIR: Dir = include_dir!("src/content/en/education/");
static CONTENT_DE_PROJECTS_DIR: Dir = include_dir!("src/content/de/projects/");
static CONTENT_EN_PROJECTS_DIR: Dir = include_dir!("src/content/en/projects/");
static CONTENT_DE_TEACHING_DIR: Dir = include_dir!("src/content/de/teaching/");
static CONTENT_EN_TEACHING_DIR: Dir = include_dir!("src/content/en/teaching/");

#[styled_component]
pub fn Home() -> Html {
	let localization = use_localization();

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

	let education_md_dir = match localization.get() {
		Localization::EN => &CONTENT_EN_EDUCATION_DIR,
		Localization::DE => &CONTENT_DE_EDUCATION_DIR,
	};
	let mut education_md_files = education_md_dir.files().collect::<Vec<_>>();
	education_md_files.sort_by(|a, b| b.path().cmp(a.path()));
	let education_md_contents = education_md_files
		.iter()
		.map(|file| file.contents_utf8().unwrap())
		.map(|s| String::from(s))
		.collect::<Vec<String>>();

	let projects_md_dir = match localization.get() {
		Localization::EN => &CONTENT_EN_PROJECTS_DIR,
		Localization::DE => &CONTENT_DE_PROJECTS_DIR,
	};
	let mut projects_md_files = projects_md_dir.files().collect::<Vec<_>>();
	projects_md_files.sort_by(|a, b| b.path().cmp(a.path()));
	let projects_md_contents = projects_md_files
		.iter()
		.map(|file| file.contents_utf8().unwrap())
		.map(|s| String::from(s))
		.collect::<Vec<String>>();

	let teaching_md_dir = match localization.get() {
		Localization::EN => &CONTENT_EN_TEACHING_DIR,
		Localization::DE => &CONTENT_DE_TEACHING_DIR,
	};
	let teaching_border_color_offset = teaching_md_dir.files().count() + 1;
	let mut teaching_md_files = teaching_md_dir.files().collect::<Vec<_>>();
	teaching_md_files.sort_by(|a, b| b.path().cmp(a.path()));
	let teaching_md_contents = teaching_md_files
		.iter()
		.map(|file| file.contents_utf8().unwrap())
		.map(|s| String::from(s))
		.collect::<Vec<String>>();

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
						<div class="border-r-0 xl:border-r-2 border-solid pr-2 border-foreground-tertiary">
							<SafeHtml html={intro_html} />
						</div>
						<div class="border-t-2 xl:border-t-0 border-solid p-2 xl:pr-0 border-foreground-tertiary">
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
			<section id="project_cards" class={String::from("bg-background-tertiary text-foreground-primary ") + &cv_section_css}>
				<div class={section_box_css.clone()}>
					<div class={String::from("border-foreground-secondary ") + &section_title_css}>
						{ translation.projects.clone() }
					</div>
					<div class="flex flex-row flex-wrap justify-around p-5">
						{
							for projects_md_contents.into_iter().map(|content| {
								html! {
									<ProjectCard markdown={content} />
								}
							})
						}
					</div>
				</div>
			</section>

			/* More About Me */
			<section id="more_about_me" class={String::from("bg-foreground-tertiary text-background-primary ") + &cv_section_css}>
				<div class={section_box_css.clone()}>
					<div class={String::from("border-background-primary mb-3 ") + &section_title_css}>
						{ translation.more.clone() }
					</div>
					<SafeHtml html={more_html} />
				</div>
			</section>

			/* Education */
			<section id="education" class={String::from("text-foreground-primary ") + &cv_section_css}>
				<div class={section_box_css.clone()}>
					<div class={String::from("border-foreground-secondary ") + &section_title_css}>
						{ translation.education.clone() }
					</div>
					{
						education_md_contents.into_iter().enumerate().map(|(index, md_str)| {
							html! {
								<div class="min-w-full w-full">
									<ContentEducation markdown={md_str} border_color={border_color(index)} />
								</div>
							}
						}).collect::<Html>()
					}
				</div>
			</section>

			/* Teaching */
			<section id="teaching" class={String::from("bg-background-tertiary text-foreground-primary ") + &cv_section_css}>
				<div class={section_box_css.clone()}>
					<div class={String::from("border-foreground-secondary ") + &section_title_css}>
						{ translation.teaching.clone() }
					</div>
					{
						teaching_md_contents.into_iter().enumerate().map(|(index, md_str)| {
							html! {
								<div class="min-w-full w-full">
									<ContentTeaching markdown={md_str} border_color={border_color(index + teaching_border_color_offset)} />
								</div>
							}
						}).collect::<Html>()
					}
				</div>
			</section>

			/* Footer */
			<section id="footer" class=" mx-0 xl:mx-auto max-w-7xl flex flex-col xl:flex-row flex-wrap items-center justify-evenly space-y-1 py-8">
				<a href="mailto:me@tilmohr.com" class="flex items-center text-foreground-primary" >
					<i class="fa-solid fa-envelope"></i>
					<span class="ml-2">{"me@tilmohr.com"}</span>
				</a>
				<a href="https://github.com/CodingTil" class="flex items-center text-foreground-primary">
					<i class="fa-brands fa-square-github"></i>
					<span class="ml-2">{"CodingTil"}</span>
				</a>
			</section>
		</div>
	}
}
