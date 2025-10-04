use std::borrow::Cow;

use stylist::yew::styled_component;
use yew::prelude::*;

use include_dir::{include_dir, Dir};

use crate::localization::{use_localization, Localization};

use crate::components::content_education::ContentEducation;
use crate::components::content_experience::ContentExperience;
use crate::components::project::ProjectCard;

struct Translations {
	hero_job_title: Cow<'static, str>,
	hero_location: Cow<'static, str>,
	hero_cta_contact: Cow<'static, str>,
	hero_cta_work: Cow<'static, str>,
	projects_title: Cow<'static, str>,
	projects_subtitle: Cow<'static, str>,
	experience_title: Cow<'static, str>,
	experience_subtitle: Cow<'static, str>,
	education_title: Cow<'static, str>,
	education_subtitle: Cow<'static, str>,
	contact_title: Cow<'static, str>,
	contact_subtitle: Cow<'static, str>,
	contact_email_title: Cow<'static, str>,
	contact_email_desc: Cow<'static, str>,
	contact_github_title: Cow<'static, str>,
	contact_github_desc: Cow<'static, str>,
	contact_linkedin_title: Cow<'static, str>,
	contact_linkedin_desc: Cow<'static, str>,
}

const EN_TRANSLATIONS: Translations = Translations {
	hero_job_title: Cow::Borrowed("Software Consultant @ TNG Technology Consulting GmbH"),
	hero_location: Cow::Borrowed("Munich, Germany"),
	hero_cta_contact: Cow::Borrowed("Get in Touch"),
	hero_cta_work: Cow::Borrowed("View Work"),
	projects_title: Cow::Borrowed("Featured Projects"),
	projects_subtitle: Cow::Borrowed("A selection of my side projects"),
	experience_title: Cow::Borrowed("Experience"),
	experience_subtitle: Cow::Borrowed("My professional journey"),
	education_title: Cow::Borrowed("Education"),
	education_subtitle: Cow::Borrowed("My academic background"),
	contact_title: Cow::Borrowed("Get in Touch"),
	contact_subtitle: Cow::Borrowed("Feel free to reach out through any of these channels!"),
	contact_email_title: Cow::Borrowed("Email"),
	contact_email_desc: Cow::Borrowed("Drop me a message anytime"),
	contact_github_title: Cow::Borrowed("GitHub"),
	contact_github_desc: Cow::Borrowed("Check out my code and projects"),
	contact_linkedin_title: Cow::Borrowed("LinkedIn"),
	contact_linkedin_desc: Cow::Borrowed("Let's connect professionally"),
};

const DE_TRANSLATIONS: Translations = Translations {
	hero_job_title: Cow::Borrowed("Software Consultant @ TNG Technology Consulting GmbH"),
	hero_location: Cow::Borrowed("München, Deutschland"),
	hero_cta_contact: Cow::Borrowed("Kontakt aufnehmen"),
	hero_cta_work: Cow::Borrowed("Projekte ansehen"),
	projects_title: Cow::Borrowed("Ausgewählte Projekte"),
	projects_subtitle: Cow::Borrowed("Eine Auswahl meiner Nebenprojekte"),
	experience_title: Cow::Borrowed("Erfahrung"),
	experience_subtitle: Cow::Borrowed("Mein beruflicher Werdegang"),
	education_title: Cow::Borrowed("Bildung"),
	education_subtitle: Cow::Borrowed("Mein akademischer Hintergrund"),
	contact_title: Cow::Borrowed("Kontakt aufnehmen"),
	contact_subtitle: Cow::Borrowed(
		"Zögern Sie nicht, mich über einen dieser Kanäle zu kontaktieren!",
	),
	contact_email_title: Cow::Borrowed("E-Mail"),
	contact_email_desc: Cow::Borrowed("Schreiben Sie mir jederzeit"),
	contact_github_title: Cow::Borrowed("GitHub"),
	contact_github_desc: Cow::Borrowed("Schauen Sie sich meinen Code und meine Projekte an"),
	contact_linkedin_title: Cow::Borrowed("LinkedIn"),
	contact_linkedin_desc: Cow::Borrowed("Lassen Sie uns professionell vernetzen"),
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

	// Modern design system
	let section_spacing = String::from("py-20 px-4 md:px-8");
	let container_max = String::from("max-w-7xl mx-auto");

	// Mesh gradient background
	let mesh_bg = css!(
		r#"
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		z-index: -1;
		background:
			radial-gradient(at 0% 0%, rgba(96, 165, 250, 0.15) 0px, transparent 50%),
			radial-gradient(at 100% 0%, rgba(167, 139, 250, 0.15) 0px, transparent 50%),
			radial-gradient(at 100% 100%, rgba(244, 114, 182, 0.15) 0px, transparent 50%),
			radial-gradient(at 0% 100%, rgba(52, 211, 153, 0.15) 0px, transparent 50%),
			#0F172A;
	"#
	);

	html! {
		<>
			<div class={mesh_bg} />
			<div class="min-w-full relative">
				<section id="hero" class={css!(r#"
					min-height: 100vh;
					display: flex;
					align-items: center;
					justify-content: center;
					padding: 8rem 2rem 4rem;
					position: relative;
				"#)}>
					<div class={css!(r#"
						max-width: 72rem;
						text-align: center;
					"#)}>
						<h1 class={css!(r#"
							font-size: clamp(3rem, 8vw, 7rem);
							font-weight: 800;
							letter-spacing: -0.03em;
							margin-bottom: 1.5rem;
							background: linear-gradient(135deg, #60A5FA 0%, #A78BFA 25%, #F472B6 50%, #FBBF24 75%, #34D399 100%);
							background-size: 200% 200%;
							-webkit-background-clip: text;
							background-clip: text;
							-webkit-text-fill-color: transparent;
							animation: gradient-x 8s ease infinite;
							font-family: 'Space Grotesk', sans-serif;

							@keyframes gradient-x {
								0%, 100% {
									background-position: 0% 50%;
								}
								50% {
									background-position: 100% 50%;
								}
							}
						"#)}>
							{"TIL MOHR"}
						</h1>

						<p class={css!(r#"
							font-size: clamp(1.25rem, 3vw, 2rem);
							color: rgba(226, 232, 240, 0.9);
							font-weight: 500;
							margin-bottom: 1rem;
							font-family: 'Inter', sans-serif;
						"#)}>
							{translation.hero_job_title.clone()}
						</p>

						<p class={css!(r#"
							font-size: 1.125rem;
							color: rgba(203, 213, 225, 0.7);
							margin-bottom: 3rem;
							display: flex;
							align-items: center;
							justify-content: center;
							gap: 0.5rem;
						"#)}>
							<i class="fa-solid fa-location-dot"></i>
							{translation.hero_location.clone()}
						</p>

						<div class={css!(r#"
							display: flex;
							gap: 1rem;
							justify-content: center;
							flex-wrap: wrap;
							margin-bottom: 4rem;
						"#)}>
							<a href="#contact" class={css!(r#"
								padding: 1rem 2rem;
								background: linear-gradient(135deg, #60A5FA, #A78BFA);
								color: white;
								border-radius: 0.75rem;
								font-weight: 600;
								transition: all 0.3s ease;
								box-shadow: 0 4px 6px rgba(96, 165, 250, 0.3);
								&:hover {
									transform: translateY(-2px);
									box-shadow: 0 8px 12px rgba(96, 165, 250, 0.4);
								}
							"#)}>
								<i class="fa-solid fa-address-card mr-2"></i>
								{translation.hero_cta_contact.clone()}
							</a>
							<a href="#projects" class={css!(r#"
								padding: 1rem 2rem;
								background: rgba(51, 65, 85, 0.5);
								color: #F1F5F9;
								border-radius: 0.75rem;
								font-weight: 600;
								border: 1px solid rgba(203, 213, 225, 0.2);
								transition: all 0.3s ease;
								&:hover {
									background: rgba(71, 85, 105, 0.7);
									transform: translateY(-2px);
								}
							"#)}>
								{translation.hero_cta_work.clone()}
								<i class="fa-solid fa-arrow-right ml-2"></i>
							</a>
						</div>

						<div class={css!(r#"
							display: flex;
							gap: 1.5rem;
							justify-content: center;
							opacity: 0;
							animation: fadeIn 1s ease 0.5s forwards;
							@keyframes fadeIn {
								to { opacity: 1; }
							}
						"#)}>
							<a href="mailto:me@tilmohr.com" class={css!(r#"
								width: 3rem;
								height: 3rem;
								display: flex;
								align-items: center;
								justify-content: center;
								border-radius: 50%;
								background: rgba(51, 65, 85, 0.5);
								color: #CBD5E1;
								font-size: 1.25rem;
								transition: all 0.3s ease;
								border: 1px solid rgba(203, 213, 225, 0.1);
								&:hover {
									background: rgba(71, 85, 105, 0.7);
									color: #60A5FA;
									transform: translateY(-2px);
								}
							"#)}>
								<i class="fa-solid fa-envelope"></i>
							</a>
							<a href="https://github.com/CodingTil" class={css!(r#"
								width: 3rem;
								height: 3rem;
								display: flex;
								align-items: center;
								justify-content: center;
								border-radius: 50%;
								background: rgba(51, 65, 85, 0.5);
								color: #CBD5E1;
								font-size: 1.25rem;
								transition: all 0.3s ease;
								border: 1px solid rgba(203, 213, 225, 0.1);
								&:hover {
									background: rgba(71, 85, 105, 0.7);
									color: #60A5FA;
									transform: translateY(-2px);
								}
							"#)}>
								<i class="fa-brands fa-github"></i>
							</a>
							<a href="https://linkedin.com/in/tilmohr" class={css!(r#"
								width: 3rem;
								height: 3rem;
								display: flex;
								align-items: center;
								justify-content: center;
								border-radius: 50%;
								background: rgba(51, 65, 85, 0.5);
								color: #CBD5E1;
								font-size: 1.25rem;
								transition: all 0.3s ease;
								border: 1px solid rgba(203, 213, 225, 0.1);
								&:hover {
									background: rgba(71, 85, 105, 0.7);
									color: #0077B5;
									transform: translateY(-2px);
								}
							"#)}>
								<i class="fa-brands fa-linkedin-in"></i>
							</a>
						</div>

						<div class={css!(r#"
							position: absolute;
							bottom: 2rem;
							left: 50%;
							transform: translateX(-50%);
							opacity: 0;
							animation: fadeIn 1s ease 2s forwards, bounce 2s ease-in-out 3s infinite;
							@keyframes fadeIn {
								to { opacity: 1; }
							}
							@keyframes bounce {
								0%, 100% { transform: translate(-50%, 0); }
								50% { transform: translate(-50%, 10px); }
							}
						"#)}>
							<a href="#projects" class={css!(r#"
								color: rgba(203, 213, 225, 0.5);
								font-size: 2rem;
								&:hover {
									color: rgba(203, 213, 225, 0.8);
								}
							"#)}>
								<i class="fa-solid fa-chevron-down"></i>
							</a>
						</div>
					</div>
				</section>

				<section id="projects" class={format!("{} {}", section_spacing, container_max)}>
					<div class={css!(r#"
						margin-bottom: 4rem;
					"#)}>
						<h2 class={css!(r#"
							font-size: clamp(2.5rem, 5vw, 4rem);
							font-weight: 700;
							color: #F1F5F9;
							margin-bottom: 1rem;
							font-family: 'Space Grotesk', sans-serif;
						"#)}>
							{translation.projects_title.clone()}
						</h2>
						<p class={css!(r#"
							font-size: 1.25rem;
							color: rgba(203, 213, 225, 0.7);
							max-width: 48rem;
						"#)}>
							{translation.projects_subtitle.clone()}
						</p>
					</div>

					<div class={css!(r#"
						display: grid;
						gap: 1.5rem;
						grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
						@media (min-width: 768px) {
							grid-template-columns: repeat(2, 1fr);
						}
						@media (min-width: 1024px) {
							grid-template-columns: repeat(3, 1fr);
						}
					"#)}>
						{
							for projects_md_contents.into_iter().map(|content| {
								html! {
									<ProjectCard markdown={content} />
								}
							})
						}
					</div>
				</section>

				<section id="experience" class={format!("{} {}", section_spacing, container_max)}>
					<div class={css!(r#"
						margin-bottom: 4rem;
					"#)}>
						<h2 class={css!(r#"
							font-size: clamp(2.5rem, 5vw, 4rem);
							font-weight: 700;
							color: #F1F5F9;
							margin-bottom: 1rem;
							font-family: 'Space Grotesk', sans-serif;
						"#)}>
							{translation.experience_title.clone()}
						</h2>
						<p class={css!(r#"
							font-size: 1.25rem;
							color: rgba(203, 213, 225, 0.7);
							max-width: 48rem;
						"#)}>
							{translation.experience_subtitle.clone()}
						</p>
					</div>

					<div class={css!(r#"
						position: relative;
						padding-left: 2rem;
						&::before {
							content: '';
							position: absolute;
							left: 0;
							top: 0;
							bottom: 0;
							width: 2px;
							background: linear-gradient(180deg, #60A5FA, transparent);
						}
					"#)}>
						{
							experience_md_contents.into_iter().enumerate().map(|(index, md_str)| {
								html! {
									<ContentExperience markdown={md_str} border_color={border_color(index)} />
								}
							}).collect::<Html>()
						}
					</div>
				</section>

				<section id="education" class={format!("{} {}", section_spacing, container_max)}>
					<div class={css!(r#"
						margin-bottom: 4rem;
					"#)}>
						<h2 class={css!(r#"
							font-size: clamp(2.5rem, 5vw, 4rem);
							font-weight: 700;
							color: #F1F5F9;
							margin-bottom: 1rem;
							font-family: 'Space Grotesk', sans-serif;
						"#)}>
							{translation.education_title.clone()}
						</h2>
						<p class={css!(r#"
							font-size: 1.25rem;
							color: rgba(203, 213, 225, 0.7);
							max-width: 48rem;
						"#)}>
							{translation.education_subtitle.clone()}
						</p>
					</div>

					<div class={css!(r#"
						position: relative;
						padding-left: 2rem;
						&::before {
							content: '';
							position: absolute;
							left: 0;
							top: 0;
							bottom: 0;
							width: 2px;
							background: linear-gradient(180deg, #A78BFA, transparent);
						}
					"#)}>
						{
							education_md_contents.into_iter().enumerate().map(|(index, md_str)| {
								html! {
									<ContentEducation markdown={md_str} border_color={border_color(index + education_border_color_offset)} />
								}
							}).collect::<Html>()
						}
					</div>
				</section>

				<section id="contact" class={format!("{} {}", section_spacing, container_max)}>
					<div class={css!(r#"
						margin-bottom: 4rem;
						text-align: center;
					"#)}>
						<h2 class={css!(r#"
							font-size: clamp(2.5rem, 5vw, 4rem);
							font-weight: 700;
							color: #F1F5F9;
							margin-bottom: 1rem;
							font-family: 'Space Grotesk', sans-serif;
						"#)}>
							{translation.contact_title.clone()}
						</h2>
						<p class={css!(r#"
							font-size: 1.25rem;
							color: rgba(203, 213, 225, 0.7);
							max-width: 48rem;
							margin: 0 auto;
						"#)}>
							{translation.contact_subtitle.clone()}
						</p>
					</div>

					<div class={css!(r#"
						display: grid;
						gap: 1.5rem;
						grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
						max-width: 60rem;
						margin: 0 auto;
					"#)}>
						<a href="mailto:me@tilmohr.com" class={css!(r#"
							padding: 2rem;
							background: rgba(30, 41, 59, 0.5);
							border: 1px solid rgba(203, 213, 225, 0.1);
							border-radius: 1rem;
							transition: all 0.3s ease;
							text-align: center;
							&:hover {
								transform: translateY(-4px);
								border-color: rgba(251, 191, 36, 0.3);
								background: rgba(30, 41, 59, 0.7);
								box-shadow: 0 10px 30px rgba(251, 191, 36, 0.1);
							}
						"#)}>
							<div class={css!(r#"
								width: 4rem;
								height: 4rem;
								margin: 0 auto 1.5rem;
								border-radius: 50%;
								background: linear-gradient(135deg, rgba(251, 191, 36, 0.2), rgba(251, 191, 36, 0.1));
								display: flex;
								align-items: center;
								justify-content: center;
								border: 1px solid rgba(251, 191, 36, 0.2);
							"#)}>
								<i class="fa-solid fa-envelope" style="font-size: 1.5rem; color: #FBBF24;"></i>
							</div>
							<h3 class={css!(r#"
								font-size: 1.25rem;
								font-weight: 700;
								color: #F1F5F9;
								margin-bottom: 0.5rem;
								font-family: 'Space Grotesk', sans-serif;
							"#)}>
								{translation.contact_email_title.clone()}
							</h3>
							<p class={css!(r#"
								color: rgba(203, 213, 225, 0.7);
								font-size: 0.875rem;
								margin-bottom: 0.75rem;
							"#)}>
								{translation.contact_email_desc.clone()}
							</p>
							<p class={css!(r#"
								color: #FBBF24;
								font-weight: 600;
								font-size: 0.9rem;
							"#)}>
								{"me@tilmohr.com"}
							</p>
						</a>

						<a href="https://github.com/CodingTil" class={css!(r#"
							padding: 2rem;
							background: rgba(30, 41, 59, 0.5);
							border: 1px solid rgba(203, 213, 225, 0.1);
							border-radius: 1rem;
							transition: all 0.3s ease;
							text-align: center;
							&:hover {
								transform: translateY(-4px);
								border-color: rgba(96, 165, 250, 0.3);
								background: rgba(30, 41, 59, 0.7);
								box-shadow: 0 10px 30px rgba(96, 165, 250, 0.1);
							}
						"#)}>
							<div class={css!(r#"
								width: 4rem;
								height: 4rem;
								margin: 0 auto 1.5rem;
								border-radius: 50%;
								background: linear-gradient(135deg, rgba(96, 165, 250, 0.2), rgba(96, 165, 250, 0.1));
								display: flex;
								align-items: center;
								justify-content: center;
								border: 1px solid rgba(96, 165, 250, 0.2);
							"#)}>
								<i class="fa-brands fa-github" style="font-size: 1.5rem; color: #60A5FA;"></i>
							</div>
							<h3 class={css!(r#"
								font-size: 1.25rem;
								font-weight: 700;
								color: #F1F5F9;
								margin-bottom: 0.5rem;
								font-family: 'Space Grotesk', sans-serif;
							"#)}>
								{translation.contact_github_title.clone()}
							</h3>
							<p class={css!(r#"
								color: rgba(203, 213, 225, 0.7);
								font-size: 0.875rem;
								margin-bottom: 0.75rem;
							"#)}>
								{translation.contact_github_desc.clone()}
							</p>
							<p class={css!(r#"
								color: #60A5FA;
								font-weight: 600;
								font-size: 0.9rem;
							"#)}>
								{"@CodingTil"}
							</p>
						</a>

						<a href="https://linkedin.com/in/tilmohr" class={css!(r#"
							padding: 2rem;
							background: rgba(30, 41, 59, 0.5);
							border: 1px solid rgba(203, 213, 225, 0.1);
							border-radius: 1rem;
							transition: all 0.3s ease;
							text-align: center;
							&:hover {
								transform: translateY(-4px);
								border-color: rgba(0, 119, 181, 0.3);
								background: rgba(30, 41, 59, 0.7);
								box-shadow: 0 10px 30px rgba(0, 119, 181, 0.1);
							}
						"#)}>
							<div class={css!(r#"
								width: 4rem;
								height: 4rem;
								margin: 0 auto 1.5rem;
								border-radius: 50%;
								background: linear-gradient(135deg, rgba(0, 119, 181, 0.2), rgba(0, 119, 181, 0.1));
								display: flex;
								align-items: center;
								justify-content: center;
								border: 1px solid rgba(0, 119, 181, 0.2);
							"#)}>
								<i class="fa-brands fa-linkedin-in" style="font-size: 1.5rem; color: #0077B5;"></i>
							</div>
							<h3 class={css!(r#"
								font-size: 1.25rem;
								font-weight: 700;
								color: #F1F5F9;
								margin-bottom: 0.5rem;
								font-family: 'Space Grotesk', sans-serif;
							"#)}>
								{translation.contact_linkedin_title.clone()}
							</h3>
							<p class={css!(r#"
								color: rgba(203, 213, 225, 0.7);
								font-size: 0.875rem;
								margin-bottom: 0.75rem;
							"#)}>
								{translation.contact_linkedin_desc.clone()}
							</p>
							<p class={css!(r#"
								color: #0077B5;
								font-weight: 600;
								font-size: 0.9rem;
							"#)}>
								{"in/tilmohr"}
							</p>
						</a>
					</div>
				</section>
			</div>
		</>
	}
}
