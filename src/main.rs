use stylist::yew::{styled_component, Global};
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod layouts;
mod pages;
mod router;
mod theme;

use layouts::base::BaseLayout;
use router::{switch, Route};

use theme::{use_theme, ThemeProvider};

#[styled_component]
fn App() -> Html {
	let theme = use_theme();

	html! {
		<>
			<Global css={css!(
				r#"
					html, body {
						font-family: 'Poppins', sans-serif;
						padding: 0;
						margin: 0;
						display: flex;
						flex-direction: column;
						min-height: 100vh;
						background-color: ${bg};
						color: ${fg};
					}
				"#,
				bg = theme.background_primary.clone(),
				fg = theme.foreground_primary.clone(),
			)} />

			<BrowserRouter>
				<BaseLayout>
					<Switch<Route> render={switch} />
				</BaseLayout>
			</BrowserRouter>
		</>
	}
}

#[styled_component]
fn Root() -> Html {
	html! {
		<ThemeProvider>
			<App />
		</ThemeProvider>
	}
}

fn main() {
	yew::Renderer::<Root>::new().render();
}
