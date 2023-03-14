use yew::prelude::*;

use crate::components::project::ProjectPost;

use crate::localization::{use_localization, Localization};

#[function_component]
pub fn Fractal() -> Html {
	let localization = use_localization();

	let md_content = match localization.get() {
		Localization::EN => include_str!("../../content/en/projects/2023-03-10_FractalRust.md"),
		Localization::DE => include_str!("../../content/de/projects/2023-03-10_FractalRust.md"),
	};

	html! {
		<FractalInner markdown={md_content} />
	}
}

#[derive(PartialEq, Properties)]
struct FractalInnerProps {
	markdown: String,
}

struct FractalInner;

impl Component for FractalInner {
	type Message = ();
	type Properties = FractalInnerProps;

	fn create(_ctx: &Context<Self>) -> Self {
		Self {}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		html! {
			<ProjectPost markdown={ctx.props().markdown.clone()} />
		}
	}

	/*
	fn destroy(&mut self, _ctx: &Context<Self>) {
		// Reload is required to free the canvas and remove the wasm worker
		//web_sys::window().unwrap().location().reload().unwrap();
	}

	fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
		// No clue why the canvas isn't dynamically resizable here, although it is on https://github.com/CodingTil/fractal_rust
		if first_render {
			spawn_local(async move {
				//run_with_canvas(canvas_arc); // nothing
				//run(); // nothing
				//pollster::block_on(run_with_canvas(canvas_arc)); // -> no resize
				pollster::block_on(run()); // -> no resize
			});
			// From here: Does not go away when switching pages
			//run_with_canvas(canvas_arc); // nothing
			//run(); // nothing
			//pollster::block_on(run_with_canvas(canvas_arc)); // -> no resize
			//pollster::block_on(run()); // -> no resize
		}
	}
	*/
}
