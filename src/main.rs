use std::borrow::Borrow;

use stylist::{
	yew::{styled_component, Global},
	Style,
};
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod layouts;
mod localization;
mod pages;
mod router;
mod safehtml;
mod theme;

use layouts::base::BaseLayout;
use router::{switch, Route};

use localization::LocalizationProvider;
use theme::{use_theme, ThemeProvider};

#[styled_component]
fn App() -> Html {
	let theme = use_theme();

	let mut rainbow_css = "".to_owned();
	rainbow_css += format!(".text-rainbow-1 {{ color: {}; }} .bg-rainbow-1 {{ background-color: {}; }} .border-rainbow_1 {{ border-color: {}; }} ", theme.rainbow_1.clone(), theme.rainbow_1.clone(), theme.rainbow_1.clone()).as_str();
	rainbow_css += format!(".text-rainbow-2 {{ color: {}; }} .bg-rainbow-2 {{ background-color: {}; }} .border-rainbow_2 {{ border-color: {}; }} ", theme.rainbow_2.clone(), theme.rainbow_2.clone(), theme.rainbow_2.clone()).as_str();
	rainbow_css += format!(".text-rainbow-3 {{ color: {}; }} .bg-rainbow-3 {{ background-color: {}; }} .border-rainbow_3 {{ border-color: {}; }} ", theme.rainbow_3.clone(), theme.rainbow_3.clone(), theme.rainbow_3.clone()).as_str();
	rainbow_css += format!(".text-rainbow-4 {{ color: {}; }} .bg-rainbow-4 {{ background-color: {}; }} .border-rainbow_4 {{ border-color: {}; }} ", theme.rainbow_4.clone(), theme.rainbow_4.clone(), theme.rainbow_4.clone()).as_str();
	rainbow_css += format!(".text-rainbow-5 {{ color: {}; }} .bg-rainbow-5 {{ background-color: {}; }} .border-rainbow_5 {{ border-color: {}; }} ", theme.rainbow_5.clone(), theme.rainbow_5.clone(), theme.rainbow_5.clone()).as_str();
	rainbow_css += format!(".text-rainbow-6 {{ color: {}; }} .bg-rainbow-6 {{ background-color: {}; }} .border-rainbow_6 {{ border-color: {}; }} ", theme.rainbow_6.clone(), theme.rainbow_6.clone(), theme.rainbow_6.clone()).as_str();

	let mut foreground_css = "".to_owned();
	foreground_css += format!(".text-foreground-primary {{ color: {}; }} .bg-foreground-primary {{ background-color: {}; }} .border-foreground-primary {{ border-color: {}; }} ", theme.foreground_primary.clone(), theme.foreground_primary.clone(), theme.foreground_primary.clone()).as_str();
	foreground_css += format!(".text-foreground-secondary {{ color: {}; }} .bg-foreground-secondary {{ background-color: {}; }} .border-foreground-secondary {{ border-color: {}; }} ", theme.foreground_secondary.clone(), theme.foreground_secondary.clone(), theme.foreground_secondary.clone()).as_str();
	foreground_css += format!(".text-foreground-tertiary {{ color: {}; }} .bg-foreground-tertiary {{ background-color: {}; }} .border-foreground-tertiary {{ border-color: {}; }} ", theme.foreground_tertiary.clone(), theme.foreground_tertiary.clone(), theme.foreground_tertiary.clone()).as_str();

	let mut background_css = "".to_owned();
	background_css += format!(".text-background-primary {{ color: {}; }} .bg-background-primary {{ background-color: {}; }} .border-background-primary {{ border-color: {}; }} ", theme.background_primary.clone(), theme.background_primary.clone(), theme.background_primary.clone()).as_str();
	background_css += format!(".text-background-secondary {{ color: {}; }} .bg-background-secondary {{ background-color: {}; }} .border-background-secondary {{ border-color: {}; }} ", theme.background_secondary.clone(), theme.background_secondary.clone(), theme.background_secondary.clone()).as_str();
	background_css += format!(".text-background-tertiary {{ color: {}; }} .bg-background-tertiary {{ background-color: {}; }} .border-background-tertiary {{ border-color: {}; }} ", theme.background_tertiary.clone(), theme.background_tertiary.clone(), theme.background_tertiary.clone()).as_str();

	let mut other_css = "".to_owned();
	other_css += format!(".text-other-primary {{ color: {}; }} .bg-other-primary {{ background-color: {}; }} .border-other-primary {{ border-color: {}; }} ", theme.other_primary.clone(), theme.other_primary.clone(), theme.other_primary.clone()).as_str();
	other_css += format!(".text-other-secondary {{ color: {}; }} .bg-other-secondary {{ background-color: {}; }} .border-other-secondary {{ border-color: {}; }} ", theme.other_secondary.clone(), theme.other_secondary.clone(), theme.other_secondary.clone()).as_str();
	other_css += format!(".text-other-tertiary {{ color: {}; }} .bg-other-tertiary {{ background-color: {}; }} .border-other-tertiary {{ border-color: {}; }} ", theme.other_tertiary.clone(), theme.other_tertiary.clone(), theme.other_tertiary.clone()).as_str();
	other_css += format!(".text-other-quaternary {{ color: {}; }} .bg-other-quaternary {{ background-color: {}; }} .border-other-quaternary {{ border-color: {}; }} ", theme.other_quaternary.clone(), theme.other_quaternary.clone(), theme.other_quaternary.clone()).as_str();

	let mut color_css = "".to_owned();
	color_css += format!(".text-color-error {{ color: {}; }} .bg-color-error {{ background-color: {}; }} .border-color-error {{ border-color: {}; }} ", theme.color_error.clone(), theme.color_error.clone(), theme.color_error.clone()).as_str();
	color_css += format!(".text-color-warning {{ color: {}; }} .bg-color-warning {{ background-color: {}; }} .border-color-warning {{ border-color: {}; }} ", theme.color_warning.clone(), theme.color_warning.clone(), theme.color_warning.clone()).as_str();
	color_css += format!(".text-color-success {{ color: {}; }} .bg-color-success {{ background-color: {}; }} .border-color-success {{ border-color: {}; }} ", theme.color_success.clone(), theme.color_success.clone(), theme.color_success.clone()).as_str();
	color_css += format!(".text-color-info {{ color: {}; }} .bg-color-info {{ background-color: {}; }} .border-color-info {{ border-color: {}; }} ", theme.color_info.clone(), theme.color_info.clone(), theme.color_info.clone()).as_str();

	let html_body_css = format!(
		r#"
		html, body {{
			font-family: 'Poppins', sans-serif;
			padding: 0;
			margin: 0;
			display: flex;
			flex-direction: column;
			min-height: 100vh;
			background-color: {bg_1};
			color: {fg_1};
		}}
	"#,
		bg_1 = theme.background_primary.clone(),
		fg_1 = theme.foreground_primary.clone()
	);

	let global_css = format!(
		"{}{}{}{}{}{}",
		rainbow_css, foreground_css, background_css, other_css, color_css, html_body_css
	);

	html! {
		<>
			<Global css={global_css.clone()} />
			<BrowserRouter>
				<BaseLayout>
					<Switch<Route> render={switch} />
				</BaseLayout>
			</BrowserRouter>
		</>
	}
}

#[function_component]
fn Root() -> Html {
	html! {
		<ThemeProvider>
			<LocalizationProvider>
				<App />
			</LocalizationProvider>
		</ThemeProvider>
	}
}

fn main() {
	yew::Renderer::<Root>::new().render();
}
