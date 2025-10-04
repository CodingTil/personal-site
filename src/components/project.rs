use comrak::{format_html, parse_document, Arena, ComrakOptions};
use serde::Deserialize;
use stylist::yew::styled_component;
use stylist::{css, style};
use yaml_front_matter::YamlFrontMatter;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

use crate::safehtml::SafeHtml;
use crate::theme::use_theme;

#[derive(Deserialize)]
pub struct ProjectMetadata {
	pub slug: String,
	pub image: String,
	pub title: String,
	pub color: String,
	pub tagline: String,
	pub url: String,
	pub date_range: String,
	pub skills: Vec<String>,
	#[allow(dead_code)]
	pub filters: Vec<String>,
	pub coauthors: Option<Vec<Coauthor>>,
	pub report: Option<String>,
}

#[derive(Deserialize)]
pub struct Coauthor {
	pub name: String,
	pub url: Option<String>,
}

#[derive(Debug, PartialEq, Properties)]
pub struct ProjectCardProps {
	pub markdown: String,
}

#[styled_component]
pub fn ProjectCard(props: &ProjectCardProps) -> Html {
	let md_str = props.markdown.clone();

	// Get Front Matter
	let document = YamlFrontMatter::parse::<ProjectMetadata>(&md_str).unwrap();
	let ProjectMetadata {
		slug,
		image,
		title,
		tagline,
		skills: tags,
		..
	} = document.metadata;

	let to_route = match slug.as_str() {
		"flappyking" => Route::FlappyKing,
		"simplechat" => Route::SimpleChat,
		"fractal" => Route::Fractal,
		"eiuie" => Route::Eiuie,
		"oceancurrents" => Route::OceanCurrents,
		"py_css" => Route::Pycss,
		_ => panic!("Invalid slug: {}", slug),
	};

	html! {
		<Link<Route> to={to_route}>
			<div class={css!(r#"
				position: relative;
				border-radius: 1rem;
				overflow: hidden;
				background: rgba(30, 41, 59, 0.5);
				border: 1px solid rgba(203, 213, 225, 0.1);
				transition: all 0.3s ease;
				height: 100%;
				display: flex;
				flex-direction: column;

				&:hover {
					transform: translateY(-4px);
					border-color: rgba(96, 165, 250, 0.3);
					box-shadow: 0 20px 40px rgba(0, 0, 0, 0.4);
					background: rgba(30, 41, 59, 0.7);
				}
			"#)}>
				<div class={css!(r#"
					position: relative;
					overflow: hidden;
					aspect-ratio: 16 / 9;
					background: rgba(15, 23, 42, 0.5);
				"#)}>
					<div class={css!(r#"
						transition: all 0.5s ease;
						&:hover {
							transform: scale(1.05);
						}
					"#)}>
						<SafeHtml html={image.clone()} />
					</div>

					<div class={css!(r#"
						position: absolute;
						inset: 0;
						background: linear-gradient(180deg, transparent 0%, rgba(15, 23, 42, 0.8) 100%);
						opacity: 0;
						transition: opacity 0.3s ease;

						&:hover {
							opacity: 1;
						}
					"#)} />
				</div>

				<div class={css!(r#"
					padding: 1.5rem;
					flex: 1;
					display: flex;
					flex-direction: column;
				"#)}>
					<h3 class={css!(r#"
						font-size: 1.25rem;
						font-weight: 700;
						color: #F1F5F9;
						margin-bottom: 0.5rem;
						font-family: 'Space Grotesk', sans-serif;
					"#)}>
						{title.clone()}
					</h3>

					<p class={css!(r#"
						color: rgba(203, 213, 225, 0.7);
						font-size: 0.875rem;
						line-height: 1.5;
						margin-bottom: 1rem;
					"#)}>
						{tagline.clone()}
					</p>

					<div class={css!(r#"
						display: flex;
						flex-wrap: wrap;
						gap: 0.5rem;
						margin-top: auto;
					"#)}>
						{
							for tags.iter().take(3).map(|tag| {
								html! {
									<span class={css!(r#"
										padding: 0.25rem 0.75rem;
										background: rgba(96, 165, 250, 0.1);
										border: 1px solid rgba(96, 165, 250, 0.2);
										border-radius: 0.375rem;
										font-size: 0.75rem;
										color: #60A5FA;
										font-weight: 500;
									"#)}>
										{tag}
									</span>
								}
							})
						}
					</div>
				</div>
			</div>
		</Link<Route>>
	}
}

/// Render coauthors block
fn render_coauthors(coauthors: Option<Vec<Coauthor>>) -> Html {
	match coauthors {
		Some(ca_list) if !ca_list.is_empty() => ca_list
			.iter()
			.map(|ca| match &ca.url {
				Some(url) => {
					html! {
						<a href={url.clone()} class={css!(r#"
    						display: flex;
    						align-items: center;
    						gap: 0.5rem;
    						padding: 0.5rem 1rem;
    						background: rgba(30, 41, 59, 0.5);
    						border: 1px solid rgba(203, 213, 225, 0.1);
    						border-radius: 0.5rem;
    						color: rgba(203, 213, 225, 0.8);
    						font-size: 0.875rem;
                            transition: all 0.2s ease;
                            &:hover {
                                color: #A78BFA;
                                border-color: rgba(96, 165, 250, 0.3);
                                background: rgba(30, 41, 59, 0.7);
                            }
    					"#)}>
							  <i class="fa-solid fa-users"></i>
							  <span>{&ca.name}</span>
						  </a>
					}
				}
				None => {
					html! {
						<div class={css!(r#"
						display: flex;
						align-items: center;
						gap: 0.5rem;
						padding: 0.5rem 1rem;
						background: rgba(30, 41, 59, 0.5);
						border: 1px solid rgba(203, 213, 225, 0.1);
						border-radius: 0.5rem;
						color: rgba(203, 213, 225, 0.8);
						font-size: 0.875rem;
					"#)}>
								  <i class="fa-solid fa-users"></i>
								  <span>{&ca.name}</span>
							  </div>
					}
				}
			})
			.collect::<Html>(),
		_ => html! {},
	}
}

/// Render report link block
fn render_report_link(report_url: Option<String>) -> Html {
	match report_url {
		Some(report_url) => html! {
			<a href={report_url} class={css!(r#"
				display: flex;
				align-items: center;
				gap: 0.5rem;
				padding: 0.5rem 1rem;
				background: rgba(30, 41, 59, 0.5);
				border: 1px solid rgba(203, 213, 225, 0.1);
				border-radius: 0.5rem;
				color: rgba(203, 213, 225, 0.8);
				font-size: 0.875rem;
				transition: all 0.2s ease;
				&:hover {
					color: #EF4444;
					border-color: rgba(239, 68, 68, 0.3);
					background: rgba(30, 41, 59, 0.7);
				}
			"#)}>
				<i class="fa-solid fa-file-pdf"></i>
				<span>{"View Report"}</span>
			</a>
		},
		_ => html! {},
	}
}

#[styled_component]
pub fn ProjectPost(props: &ProjectCardProps) -> Html {
	let theme = use_theme();

	let md_str = props.markdown.clone();

	// Get Front Matter
	let document = YamlFrontMatter::parse::<ProjectMetadata>(&md_str).unwrap();
	let ProjectMetadata {
		slug: _,
		image: _,
		title,
		color,
		tagline,
		url,
		date_range,
		skills: tags,
		filters: _,
		coauthors,
		report,
	} = document.metadata;
	let md = document.content;

	let arena = Arena::new();
	let mut options = ComrakOptions::default();
	options.extension.front_matter_delimiter = Some("---".to_string());
	options.render.unsafe_ = true;
	options.extension.autolink = true;
	options.extension.table = true;
	options.extension.strikethrough = true;
	options.extension.tasklist = true;
	let root = parse_document(&arena, &md, &options);
	let mut md_html = String::new();
	format_html(root, &options, &mut md_html).unwrap();

	let prose_content_css = style!(
		r#"
	width: 100%;
	margin-top: 0.75rem;
	margin-bottom: 0.75rem;
	min-height: fit-content;
	--tw-text-opacity: 1;
	color: ${fg} !important;

	@media (min-width: 640px) {
		max-width: 640px;
	}

	@media (min-width: 768px) {
		max-width: 768px;
	}

	@media (min-width: 1024px) {
		max-width: 1024px;
	}

	@media (min-width: 1280px) {
		max-width: 1280px;
	}

	@media (min-width: 1536px) {
		max-width: 1536px;
	}

	img {
		position: relative;
		margin-left: auto;
		margin-right: auto;
		margin-top: 0.75rem;
		margin-bottom: 0.75rem;
		max-width: 100%;
		overflow: hidden;
		padding-left: 0px;
		padding-right: 0.5rem;
	}

	iframe {
		position: relative;
		margin-left: auto;
		margin-right: auto;
		max-width: 100%;
		min-width: 100%;
		aspect-ratio: 16 / 9;
		overflow: hidden;
		padding-left: 0px;
		padding-right: 0.5rem;
	}

	h1, h2, h3, h4, h5, h6, th, td, a, p {
		--tw-text-opacity: 1;
		color: ${fg} !important;
		-webkit-text-decoration-line: none;
		text-decoration-line: none;
	}

	h1 {
		margin-top: 2rem;
		margin-bottom: 0px;
		border-bottom-width: 1px;
		--tw-border-opacity: 1;
		border-color: ${fg};
		padding-bottom: 0.25rem;
	}

	h2 {
		margin-top: 1rem;
		margin-bottom: -0.25rem;
	}

	h3 {
		margin-top: 0rem;
		margin-bottom: 0rem;
	}

	h4 {
		margin-top: 0rem;
		margin-bottom: 0rem;
	}

	hr {
		--tw-border-opacity: 1;
		border-color: ${fg};
		margin-top: 0rem;
		margin-bottom: 0rem;
	}

	a.my-a {
		-webkit-text-decoration-line: none;
				text-decoration-line: none;
	}

	p {
		margin-top: 0.75rem;
		margin-bottom: 0.75rem;
		line-height: 1.5rem;
	}

	ul {
		margin-top: -0.25rem;
		padding-left: 1.75rem;
		padding-right: 1.75rem;
	}

	li {
		margin-bottom: 0.25rem;
		line-height: 1.5rem;
	}

	sup {
		--tw-text-opacity: 1;
		color:${fg2};
		text-wrap: nowrap;
	}

	.cell {
		margin-left: 0px;
		margin-right: 0px;
		margin-bottom: 1rem;
		margin-top: 1rem;
		min-width: 100%;
	}

	@media (min-width: 768px) {
		.cell {
			width: 50%;
			min-width: 0px;
		}
	}
	"#,
		fg = theme.foreground_primary.clone(),
		fg2 = theme.foreground_secondary.clone()
	)
	.unwrap();

	html! {
		<div class={css!(r#"
			width: 100vw;
			min-width: 100%;
			background: #0F172A;
			padding-top: 6rem;
		"#)}>
			<div class={css!(r#"
				max-width: 80rem;
				margin: 0 auto;
				padding: 0 1rem 2rem;
				@media (min-width: 1280px) {
					padding: 0 0 2rem;
				}
			"#)}>
				<div class={css!(r#"
					margin-bottom: 3rem;
				"#)}>
					<h1 class={css!(r#"
						font-size: clamp(2.5rem, 6vw, 4rem);
						font-weight: 800;
						color: #F1F5F9;
						margin-bottom: 1rem;
						font-family: 'Space Grotesk', sans-serif;
						line-height: 1.2;
					"#)}>
						{title.clone()}
					</h1>
					<p class={css!(r#"
						font-size: 1.5rem;
						color: rgba(203, 213, 225, 0.8);
						margin-bottom: 2rem;
						line-height: 1.5;
					"#)}>
						{tagline.clone()}
					</p>

					<div class={css!(r#"
    					display: flex;
    					flex-wrap: wrap;
						gap: 1rem;
						margin-bottom: 2rem;
					"#)}>
						<div class={css!(r#"
							display: flex;
							align-items: center;
							gap: 0.5rem;
							padding: 0.5rem 1rem;
							background: rgba(30, 41, 59, 0.5);
							border: 1px solid rgba(203, 213, 225, 0.1);
							border-radius: 0.5rem;
							color: rgba(203, 213, 225, 0.8);
							font-size: 0.875rem;
						"#)}>
							<i class="fa-solid fa-calendar-days"></i>
							<span>{date_range.clone()}</span>
						</div>
						<a href={url.clone()} class={css!(r#"
							display: flex;
							align-items: center;
							gap: 0.5rem;
							padding: 0.5rem 1rem;
							background: rgba(30, 41, 59, 0.5);
							border: 1px solid rgba(203, 213, 225, 0.1);
							border-radius: 0.5rem;
							color: rgba(203, 213, 225, 0.8);
							font-size: 0.875rem;
							transition: all 0.2s ease;
							&:hover {
								color: #60A5FA;
								border-color: rgba(96, 165, 250, 0.3);
								background: rgba(30, 41, 59, 0.7);
							}
						"#)}>
							<i class="fa-brands fa-github"></i>
							<span>{"Repository"}</span>
						</a>

						{ render_coauthors(coauthors) }
						{ render_report_link(report) }
					</div>

					<div class={css!(r#"
						display: flex;
						flex-wrap: wrap;
						gap: 0.5rem;
					"#)}>
						{
							for tags.iter().map(|tag| {
								html! {
									<span class={css!(r#"
										padding: 0.375rem 0.875rem;
										background: rgba(96, 165, 250, 0.1);
										border: 1px solid rgba(96, 165, 250, 0.2);
										border-radius: 0.5rem;
										font-size: 0.875rem;
										color: #60A5FA;
										font-weight: 500;
									"#)}>
										{tag}
									</span>
								}
							})
						}
					</div>
				</div>

				<div class={format!("h-1 w-full mb-8 {}", color)} />

				<div class={css!(r#"
					max-width: 80rem;
					margin: 0 auto;
					padding: 0 1rem;
				"#)}>
					<div class={format!("{} prose prose-invert prose-lg max-w-none", prose_content_css.get_class_name())}>
						<SafeHtml html={md_html.clone()} />
					</div>
				</div>
			</div>
		</div>
	}
}
