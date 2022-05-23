mod pages;
mod router;

use crate::router::{switch, Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Main)]
fn app() -> Html {
	html! {
		<BrowserRouter>
			<Switch<Route> render={switch} />
		</BrowserRouter>
	}
}

fn main() {
	yew::Renderer::<Main>::new().render();
}
