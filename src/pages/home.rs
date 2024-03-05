use std::borrow::Cow;

use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

use comrak::{format_html, parse_document, Arena, ComrakOptions};

use include_dir::{include_dir, Dir};

use crate::localization::{use_localization, Localization};

use crate::safehtml::SafeHtml;

use crate::components::content_education::ContentEducation;
use crate::components::content_experience::ContentExperience;
use crate::components::project::ProjectCard;

struct Translations {
	projects: Cow<'static, str>,
	more: Cow<'static, str>,
	experience: Cow<'static, str>,
	education: Cow<'static, str>,
}

const EN_TRANSLATIONS: Translations = Translations {
	projects: Cow::Borrowed("Projects"),
	more: Cow::Borrowed("More About Me"),
	experience: Cow::Borrowed("Experience"),
	education: Cow::Borrowed("Education"),
};

const DE_TRANSLATIONS: Translations = Translations {
	projects: Cow::Borrowed("Projekte"),
	more: Cow::Borrowed("Mehr Über Mich"),
	experience: Cow::Borrowed("Erfahrung"),
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
static CONTENT_DE_EXPERIENCE_DIR: Dir = include_dir!("src/content/de/experience/");
static CONTENT_EN_EXPERIENCE_DIR: Dir = include_dir!("src/content/en/experience/");

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
	let intro_root = parse_document(&arena, intro_md_str, &options);
	let more_root = parse_document(&arena, more_md_str, &options);
	let mut intro_html_vec = vec![];
	let mut more_html_vec = vec![];
	format_html(intro_root, &options, &mut intro_html_vec).unwrap();
	format_html(more_root, &options, &mut more_html_vec).unwrap();
	let intro_html = String::from_utf8(intro_html_vec).unwrap();
	let more_html = String::from_utf8(more_html_vec).unwrap();

	let projects_md_dir = match localization.get() {
		Localization::EN => &CONTENT_EN_PROJECTS_DIR,
		Localization::DE => &CONTENT_DE_PROJECTS_DIR,
	};
	let mut projects_md_files = projects_md_dir.files().collect::<Vec<_>>();
	projects_md_files.sort_by(|a, b| b.path().cmp(a.path()));
	let projects_md_contents = projects_md_files
		.iter()
		.map(|file| file.contents_utf8().unwrap())
		.map(String::from)
		.collect::<Vec<String>>();

	let experience_md_dir = match localization.get() {
		Localization::EN => &CONTENT_EN_EXPERIENCE_DIR,
		Localization::DE => &CONTENT_DE_EXPERIENCE_DIR,
	};
	let mut experience_md_files = experience_md_dir.files().collect::<Vec<_>>();
	experience_md_files.sort_by(|a, b| b.path().cmp(a.path()));
	let experience_md_contents = experience_md_files
		.iter()
		.map(|file| file.contents_utf8().unwrap())
		.map(String::from)
		.collect::<Vec<String>>();

	let education_md_dir = match localization.get() {
		Localization::EN => &CONTENT_EN_EDUCATION_DIR,
		Localization::DE => &CONTENT_DE_EDUCATION_DIR,
	};
	let mut education_md_files = education_md_dir.files().collect::<Vec<_>>();
	let education_border_color_offset = experience_md_dir.files().count();
	education_md_files.sort_by(|a, b| b.path().cmp(a.path()));
	let education_md_contents = education_md_files
		.iter()
		.map(|file| file.contents_utf8().unwrap())
		.map(String::from)
		.collect::<Vec<String>>();

	let cv_section_css = String::from("container mx-0 my-0 w-screen min-w-full");
	let section_box_css = String::from(
		"flex flex-col justify-center items-center py-8 px-4 xl:px-0 mx-0 xl:mx-auto max-w-7xl",
	);
	let section_title_css = String::from(
		"inline-block px-6 py-4 w-min font-extrabold text-4xl whitespace-nowrap underline",
	);

	let tilmohr_fontsize_css = style!(
		r#"
		font-size: 18vw;

		@media (min-width: 1280px) {
			font-size: 15em;
		}

		@media (max-width: 350px) {
			font-size: 16vw;
		}
	"#
	)
	.unwrap();

	html! {
		<div class="min-w-full">
			/* Intro */
			<section id="intro" class={String::from("-mt-20 ") + &cv_section_css}>
				<div class="min-h-screen pt-28 pb-8 px-4 flex flex-col justify-center items-center flex-wrap place-content-between xl:px-0 mx-0 xl:mx-auto max-w-7xl">
					<span class={String::from("text-rainbow-1 text-bold ") + tilmohr_fontsize_css.get_class_name()}>{"TIL MOHR"}</span>
					<div class="grow text-center pt-8">
						<SafeHtml html={intro_html} />
					</div>
					<div class="pt-8 border-foreground-tertiary">
						<div class="flex flex-row xl:flex-col flex-wrap justify-center xl:justify-start">
							<a href="mailto:me@tilmohr.com" class="mx-2 flex items-center text-foreground-primary">
								<i class="fa-solid fa-envelope text-[#EBCB8B]"></i>
								<span class="ml-2 whitespace-nowrap">{"me@tilmohr.com"}</span>
							</a>
							<a href="https://github.com/CodingTil" class="mx-2 flex items-center text-foreground-primary">
								<i class="fa-brands fa-square-github"></i>
								<span class="ml-2 whitespace-nowrap" >{"CodingTil"}</span>
							</a>
							<a href="https://linkedin.com/in/tilmohr" class="mx-2 flex items-center text-foreground-primary">
								<i class="fa-brands fa-linkedin text-[#0077B5]"></i>
								<span class="ml-2 whitespace-nowrap" >{"tilmohr"}</span>
							</a>
						</div>
					</div>
					<div class={css!(r#"
						animation: fadeIn 1s ease 2s forwards;
						opacity: 0;
						@keyframes fadeIn {
							from {
								opacity: 0;
							}
							to {
								opacity: 1;
							}
						}
					"#)}>
						<div class="mb-0 mt-14 animate-bounce rounded-full h-14 w-14 text-2xl border-2 border-foreground-tertiary text-foreground-tertiary flex items-center justify-center">
							<i class="fa-solid fa-angles-down"></i>
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

			/* Experience */
			<section id="experience" class={String::from("bg-background-tertiary text-foreground-primary ") + &cv_section_css}>
				<div class={section_box_css.clone()}>
					<div class={String::from("border-foreground-secondary ") + &section_title_css}>
						{ translation.experience.clone() }
					</div>
					{
						experience_md_contents.into_iter().enumerate().map(|(index, md_str)| {
							html! {
								<div class="min-w-full w-full">
									<ContentExperience markdown={md_str} border_color={border_color(index)} />
								</div>
							}
						}).collect::<Html>()
					}
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
									<ContentEducation markdown={md_str} border_color={border_color(index + education_border_color_offset)} />
								</div>
							}
						}).collect::<Html>()
					}
				</div>
			</section>
		</div>
	}
}
