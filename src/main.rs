use yew::prelude::*;
use yew_router::prelude::*;

mod layouts;
mod pages;
mod router;

use layouts::base::BaseLayout;
use router::{switch, Route};

#[function_component(App)]
fn app() -> Html {
	html! {
		<BaseLayout>
			<BrowserRouter>
				<Switch<Route> render={switch} />
			</BrowserRouter>
		</BaseLayout>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}
