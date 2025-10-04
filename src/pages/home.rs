use std::borrow::Cow;

use stylist::style;
use stylist::yew::styled_component;
use yew::prelude::*;

use include_dir::{include_dir, Dir};

use crate::components::tags::Tags;
use crate::localization::{use_localization, Localization};

use crate::components::content_education::ContentEducation;
use crate::components::content_experience::ContentExperience;
use crate::components::project::ProjectCard;

struct Translations {
	projects: Cow<'static, str>,
	experience: Cow<'static, str>,
	education: Cow<'static, str>,
}

const EN_TRANSLATIONS: Translations = Translations {
	projects: Cow::Borrowed("Projects"),
	experience: Cow::Borrowed("Experience"),
	education: Cow::Borrowed("Education"),
};

const DE_TRANSLATIONS: Translations = Translations {
	projects: Cow::Borrowed("Projekte"),
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

	let tags_en = include_str!("../content/en/tags.yaml");
	let tags_de = include_str!("../content/de/tags.yaml");

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

	let tilmohr_fontsize_css = css!(
		r#"
		font-size: 12vw;
		font-family: 'Space Grotesk', ui-sans-serif, system-ui, sans-serif;
		font-weight: 700;
		background: linear-gradient(135deg, #60A5FA 0%, #A78BFA 25%, #F472B6 50%, #FBBF24 75%, #34D399 100%);
		background-size: 200% 200%;
		-webkit-background-clip: text;
		background-clip: text;
		-webkit-text-fill-color: transparent;
		animation: gradient-x 8s ease infinite;
		letter-spacing: -0.02em;

		@media (min-width: 1280px) {
			font-size: 8em;
		}

		@media (max-width: 350px) {
			font-size: 10vw;
		}

		@keyframes gradient-x {
			0%, 100% {
				background-position: 0% 50%;
			}
			50% {
				background-position: 100% 50%;
			}
		}
		"#
	);

	let quotes_css = style!(
		r#"
		display: flex;
		flex-direction: row;
		height: 100%;
		max-width: 100%;
		min-width: 100%;

		> * {
			display: flex;
			flex-direction: row;
			justify-content: center;
			align-items: center;
			max-width: 100%;
			min-width: 100%;
		}
		"#
	)
	.unwrap();

	html! {
		<div class="min-w-full">
			/* Intro */
			<section id="intro" class={String::from("-mt-20 ") + &cv_section_css}>
				<div class={css!(r#"
					min-height: 100vh;
					padding-top: 7rem;
					padding-bottom: 2rem;
					padding-left: 1rem;
					padding-right: 1rem;
					display: flex;
					flex-direction: column;
					justify-content: center;
					align-items: center;
					max-width: 80rem;
					margin-left: auto;
					margin-right: auto;
					@media (min-width: 80rem) {
						padding-left: 0;
						padding-right: 0;
					}
				"#)}>
					<span class={tilmohr_fontsize_css}>{"TIL MOHR"}</span>
					<div class={String::from("py-6 mx-2 max-w-4xl ") + quotes_css.get_class_name()}>
						{ match localization.get() {
							// If I dont wrap one of the Quotes in a div, it seems that simply the prop is switched, which is not supported!
							Localization::EN => html! { <Tags file_content={tags_en}/> },
							Localization::DE => html! { <div><Tags file_content={tags_de}/></div> },
						} }
					</div>
					<div class={css!(r#"
						margin-top: 3rem;
						display: flex;
						flex-direction: column;
						justify-content: center;
						gap: 1rem;
						@media (min-width: 768px) {
							flex-direction: row;
							flex-wrap: nowrap;
							gap: 1.5rem;
						}
						@media (min-width: 80rem) {
							flex-direction: column;
							justify-content: flex-start;
						}
					"#)}>
						<a href="mailto:me@tilmohr.com" class={css!(r#"
							display: flex;
							align-items: center;
							gap: 0.5rem;
							padding: 0.75rem 1.5rem;
							background-color: rgba(51, 65, 85, 0.5);
							border-radius: 0.75rem;
							transition: all 0.3s ease;
							border: 1px solid rgba(203, 213, 225, 0.1);
							&:hover {
								transform: translateY(-2px);
								background-color: rgba(71, 85, 105, 0.6);
								box-shadow: 0 10px 15px rgba(0, 0, 0, 0.3);
							}
						"#)}>
							<i class="fa-solid fa-envelope text-xl text-rainbow-3"></i>
							<span class="text-foreground-primary font-medium whitespace-nowrap">{"me@tilmohr.com"}</span>
						</a>
						<a href="https://github.com/CodingTil" class={css!(r#"
							display: flex;
							align-items: center;
							gap: 0.5rem;
							padding: 0.75rem 1.5rem;
							background-color: rgba(51, 65, 85, 0.5);
							border-radius: 0.75rem;
							transition: all 0.3s ease;
							border: 1px solid rgba(203, 213, 225, 0.1);
							&:hover {
								transform: translateY(-2px);
								background-color: rgba(71, 85, 105, 0.6);
								box-shadow: 0 10px 15px rgba(0, 0, 0, 0.3);
							}
						"#)}>
							<i class="fa-brands fa-square-github text-xl text-rainbow-1"></i>
							<span class="text-foreground-primary font-medium whitespace-nowrap" >{"CodingTil"}</span>
						</a>
						<a href="https://linkedin.com/in/tilmohr" class={css!(r#"
							display: flex;
							align-items: center;
							gap: 0.5rem;
							padding: 0.75rem 1.5rem;
							background-color: rgba(51, 65, 85, 0.5);
							border-radius: 0.75rem;
							transition: all 0.3s ease;
							border: 1px solid rgba(203, 213, 225, 0.1);
							&:hover {
								transform: translateY(-2px);
								background-color: rgba(71, 85, 105, 0.6);
								box-shadow: 0 10px 15px rgba(0, 0, 0, 0.3);
							}
						"#)}>
							<i class="fa-brands fa-linkedin text-xl text-[#0077B5]"></i>
							<span class="text-foreground-primary font-medium whitespace-nowrap" >{"tilmohr"}</span>
						</a>
					</div>
					<div class={css!(r#"
						animation: fadeIn 1s ease 2s forwards, float 6s ease-in-out 3s infinite;
						opacity: 0;
						margin-top: 4rem;
						@keyframes fadeIn {
							from {
								opacity: 0;
							}
							to {
								opacity: 1;
							}
						}
						@keyframes float {
							0%, 100% {
								transform: translateY(0px);
							}
							50% {
								transform: translateY(-10px);
							}
						}
					"#)}>
						<a href="#project_cards" class={css!(r#"
							border-radius: 9999px;
							height: 3.5rem;
							width: 3.5rem;
							font-size: 1.5rem;
							border: 2px solid rgba(203, 213, 225, 0.3);
							color: rgba(203, 213, 225, 0.7);
							display: flex;
							align-items: center;
							justify-content: center;
							background: linear-gradient(135deg, rgba(96, 165, 250, 0.1) 0%, rgba(167, 139, 250, 0.1) 100%);
							transition: all 0.3s ease;
							cursor: pointer;
							&:hover {
								border-color: rgba(203, 213, 225, 0.5);
								color: rgba(203, 213, 225, 0.9);
								transform: scale(1.1);
							}
						"#)}>
							<i class="fa-solid fa-angles-down"></i>
						</a>
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

			/* Experience */
			<section id="experience" class={String::from("text-foreground-primary ") + &cv_section_css}>
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
			<section id="education" class={String::from("bg-background-tertiary text-foreground-primary ") + &cv_section_css}>
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
