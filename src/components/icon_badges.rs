use yew::prelude::*;

use stylist::{style, yew::styled_component};

pub fn get_icon_badge(name: &str) -> Option<Html> {
	return match name.to_lowercase().as_str() {
		"java" => Some(html! {<Java />}),
		"python" => Some(html! {<Python />}),
		"rust" => Some(html! {<Rust />}),
		"wasm" => Some(html! {<WASM />}),
		"webassembly" => Some(html! {<WASM />}),
		"gpu" => Some(html! {<GPU />}),
		"wgpu" => Some(html! {<GPU />}),
		"webgpu" => Some(html! {<GPU />}),
		"git" => Some(html! {<Git />}),
		"lwjgl" => Some(html! {<LWJGL />}),
		"pytorch" => Some(html! {<PyTorch />}),
		"pyterrier" => Some(html! {<PyTerrier />}),
		_ => None,
	};
}

#[derive(Debug, PartialEq, Properties)]
pub struct BadgesStripProps {
	pub tags: Vec<String>,
	pub scale: Option<u8>,
}

#[styled_component]
pub fn BadgesStrip(props: &BadgesStripProps) -> Html {
	// split comma-seperated, possibly whitespace-added list of badges
	let tags: Vec<&str> = props.tags.iter().map(|s| s.trim()).collect();
	let badges: Vec<Html> = tags.iter().filter_map(|tag| get_icon_badge(tag)).collect();

	let scale = props.scale.unwrap_or(100);

	// only if there are badges, render the strip
	if !badges.is_empty() {
		html! {
			<div class={format!("flex flex-row justify-content-center content-center scale-{} gap-x-2", scale)}>
				{ badges.into_iter().collect::<Html>() }
			</div>
		}
	} else {
		html! {<div />}
	}
}

#[styled_component]
pub fn Java() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://www.java.com/">
			<div class={String::from("fill-[#007396] bg-[#d8dee9] hover:bg-[#007396] hover:fill-[#d8dee9] transition-colors duration-200 my-2 p-1 min-h-10 text-center whitespace-nowrap block rounded ") + style_class.get_class_name()}>
				<svg viewBox="0 0 50 50" xmlns="http://www.w3.org/2000/svg">
					<path class="icon" d="M 28.1875 0 C 30.9375 6.363281 18.328125 10.292969 17.15625 15.59375 C 16.082031 20.464844 24.648438 26.125 24.65625 26.125 C 23.355469 24.109375 22.398438 22.449219 21.09375 19.3125 C 18.886719 14.007813 34.535156 9.207031 28.1875 0 Z M 36.5625 8.8125 C 36.5625 8.8125 25.5 9.523438 24.9375 16.59375 C 24.6875 19.742188 27.847656 21.398438 27.9375 23.6875 C 28.011719 25.558594 26.0625 27.125 26.0625 27.125 C 26.0625 27.125 29.609375 26.449219 30.71875 23.59375 C 31.949219 20.425781 28.320313 18.285156 28.6875 15.75 C 29.039063 13.324219 36.5625 8.8125 36.5625 8.8125 Z M 19.1875 25.15625 C 19.1875 25.15625 9.0625 25.011719 9.0625 27.875 C 9.0625 30.867188 22.316406 31.089844 31.78125 29.25 C 31.78125 29.25 34.296875 27.519531 34.96875 26.875 C 28.765625 28.140625 14.625 28.28125 14.625 27.1875 C 14.625 26.179688 19.1875 25.15625 19.1875 25.15625 Z M 38.65625 25.15625 C 37.664063 25.234375 36.59375 25.617188 35.625 26.3125 C 37.90625 25.820313 39.84375 27.234375 39.84375 28.84375 C 39.84375 32.46875 34.59375 35.875 34.59375 35.875 C 34.59375 35.875 42.71875 34.953125 42.71875 29 C 42.71875 26.296875 40.839844 24.984375 38.65625 25.15625 Z M 16.75 30.71875 C 15.195313 30.71875 12.875 31.9375 12.875 33.09375 C 12.875 35.417969 24.5625 37.207031 33.21875 33.8125 L 30.21875 31.96875 C 24.351563 33.847656 13.546875 33.234375 16.75 30.71875 Z M 18.1875 35.9375 C 16.058594 35.9375 14.65625 37.222656 14.65625 38.1875 C 14.65625 41.171875 27.371094 41.472656 32.40625 38.4375 L 29.21875 36.40625 C 25.457031 37.996094 16.015625 38.238281 18.1875 35.9375 Z M 11.09375 38.625 C 7.625 38.554688 5.375 40.113281 5.375 41.40625 C 5.375 48.28125 40.875 47.964844 40.875 40.9375 C 40.875 39.769531 39.527344 39.203125 39.03125 38.9375 C 41.933594 45.65625 9.96875 45.121094 9.96875 41.15625 C 9.96875 40.253906 12.320313 39.390625 14.5 39.8125 L 12.65625 38.75 C 12.113281 38.667969 11.589844 38.636719 11.09375 38.625 Z M 44.625 43.25 C 39.226563 48.367188 25.546875 50.222656 11.78125 47.0625 C 25.542969 52.695313 44.558594 49.535156 44.625 43.25 Z" />
					<title>{"Java"}</title>
				</svg>
			</div>
		</a>
	}
}

#[styled_component]
pub fn Python() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://www.python.org/">
			<div class={String::from("fill-[#3776AB] bg-[#d8dee9] hover:bg-[#3776AB] hover:fill-[#d8dee9] transition-colors duration-200 my-2 p-1 min-h-10 text-center whitespace-nowrap block rounded ") + style_class.get_class_name()}>
				<svg role="img" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
					<path class="icon" d="M14.25.18l.9.2.73.26.59.3.45.32.34.34.25.34.16.33.1.3.04.26.02.2-.01.13V8.5l-.05.63-.13.55-.21.46-.26.38-.3.31-.33.25-.35.19-.35.14-.33.1-.3.07-.26.04-.21.02H8.77l-.69.05-.59.14-.5.22-.41.27-.33.32-.27.35-.2.36-.15.37-.1.35-.07.32-.04.27-.02.21v3.06H3.17l-.21-.03-.28-.07-.32-.12-.35-.18-.36-.26-.36-.36-.35-.46-.32-.59-.28-.73-.21-.88-.14-1.05-.05-1.23.06-1.22.16-1.04.24-.87.32-.71.36-.57.4-.44.42-.33.42-.24.4-.16.36-.1.32-.05.24-.01h.16l.06.01h8.16v-.83H6.18l-.01-2.75-.02-.37.05-.34.11-.31.17-.28.25-.26.31-.23.38-.2.44-.18.51-.15.58-.12.64-.1.71-.06.77-.04.84-.02 1.27.05zm-6.3 1.98l-.23.33-.08.41.08.41.23.34.33.22.41.09.41-.09.33-.22.23-.34.08-.41-.08-.41-.23-.33-.33-.22-.41-.09-.41.09zm13.09 3.95l.28.06.32.12.35.18.36.27.36.35.35.47.32.59.28.73.21.88.14 1.04.05 1.23-.06 1.23-.16 1.04-.24.86-.32.71-.36.57-.4.45-.42.33-.42.24-.4.16-.36.09-.32.05-.24.02-.16-.01h-8.22v.82h5.84l.01 2.76.02.36-.05.34-.11.31-.17.29-.25.25-.31.24-.38.2-.44.17-.51.15-.58.13-.64.09-.71.07-.77.04-.84.01-1.27-.04-1.07-.14-.9-.2-.73-.25-.59-.3-.45-.33-.34-.34-.25-.34-.16-.33-.1-.3-.04-.25-.02-.2.01-.13v-5.34l.05-.64.13-.54.21-.46.26-.38.3-.32.33-.24.35-.2.35-.14.33-.1.3-.06.26-.04.21-.02.13-.01h5.84l.69-.05.59-.14.5-.21.41-.28.33-.32.27-.35.2-.36.15-.36.1-.35.07-.32.04-.28.02-.21V6.07h2.09l.14.01zm-6.47 14.25l-.23.33-.08.41.08.41.23.33.33.23.41.08.41-.08.33-.23.23-.33.08-.41-.08-.41-.23-.33-.33-.23-.41-.08-.41.08z" />
					<title>{"Python"}</title>
				</svg>
			</div>
		</a>
	}
}

#[styled_component]
pub fn Rust() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://www.rust-lang.org/">
			<div class={String::from("text-[#8B3103] bg-[#d8dee9] hover:bg-[#8B3103] hover:text-[#d8dee9] transition-colors duration-200 my-2 p-1 min-h-10 text-center content-center items-center whitespace-nowrap block rounded ") + style_class.get_class_name()}>
				<i class="fa-brands fa-rust" style="font-size: 40px;"/>
			</div>
		</a>
	}
}

#[styled_component]
pub fn WASM() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://webassembly.org/">
			<div class={String::from("bg-[#d8dee9] hover:bg-[#644eef] transition-colors duration-200 my-2 p-1 min-h-10 text-center content-center items-center whitespace-nowrap block rounded ") + style_class.get_class_name()}>
				<img width="40px" height="35px" alt="WebAssembly Logo" src="https://upload.wikimedia.org/wikipedia/commons/thumb/1/1f/WebAssembly_Logo.svg/68px-WebAssembly_Logo.svg.png?useskin=vector" />
			</div>
		</a>
	}
}

#[styled_component]
pub fn GPU() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://wgpu.rs/">
			<div class={String::from("bg-[#d8dee9] hover:bg-[#a3be8c] transition-colors duration-200 my-2 p-1 min-h-10 text-center content-center items-center whitespace-nowrap block rounded ") + style_class.get_class_name()}>
				<img src="https://wgpu.rs/logo.min.svg" width="33px" alt="WebGPU Logo" />
			</div>
		</a>
	}
}

#[styled_component]
pub fn Git() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://git-scm.com/">
			<div class={String::from("text-[#ef5032] bg-[#d8dee9] hover:bg-[#ef5032] hover:text-[#d8dee9] transition-colors duration-200 my-2 p-1 min-h-10 text-center content-center items-center whitespace-nowrap block rounded ") + style_class.get_class_name()}>
				<i class="fa-brands fa-git-alt" style="font-size: 40px;"/>
			</div>
		</a>
	}
}

#[styled_component]
pub fn LWJGL() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://www.lwjgl.org//">
			<div class={String::from("bg-[#d8dee9] hover:bg-[#b9baea] transition-colors duration-200 my-2 p-1 min-h-10 text-center content-center items-center whitespace-nowrap block rounded ") + style_class.get_class_name()}>
				<img height="40px" width="35px" alt="LWJGL Logo" src="https://upload.wikimedia.org/wikipedia/commons/thumb/8/84/LWJGL_logo.svg/272px-LWJGL_logo.svg.png?useskin=vector" />
			</div>
		</a>
	}
}

#[styled_component]
pub fn PyTorch() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://pytorch.org//">
			<div class={String::from("bg-[#d8dee9] hover:bg-[#5e81ac] transition-colors duration-200 my-2 p-1 min-h-10 text-center content-center items-center whitespace-nowrap block rounded ") + style_class.get_class_name()}>
				<img width="35px" alt="PyTorch Logo" src="https://upload.wikimedia.org/wikipedia/commons/1/10/PyTorch_logo_icon.svg" />
			</div>
		</a>
	}
}

#[styled_component]
pub fn PyTerrier() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://pyterrier.readthedocs.io/en/latest//">
			<div class={String::from("bg-[#d8dee9] hover:bg-[#8fbcbb] transition-colors duration-200 my-2 p-1 min-h-10 text-center content-center items-center whitespace-nowrap block rounded ") + style_class.get_class_name()}>
				<img width="35px" alt="PyTerrier Logo" src="http://terrier.org/img/terrier-logo-large-transparent.png" />
			</div>
		</a>
	}
}
