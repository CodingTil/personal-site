use yew::prelude::*;

use stylist::{style, yew::styled_component};

pub fn get_icon_badge(name: &str) -> Option<Html> {
	match name.to_lowercase().as_str() {
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
		"html" => Some(html! {<HTML />}),
		"css" => Some(html! {<CSS />}),
		"js" => Some(html! {<JS />}),
		"javascript" => Some(html! {<JS />}),
		"vue" => Some(html! {<Vue />}),
		"vuejs" => Some(html! {<Vue />}),
		"vue.js" => Some(html! {<Vue />}),
		"nuxt" => Some(html! {<Vue />}),
		"nuxtjs" => Some(html! {<Vue />}),
		"nuxt.js" => Some(html! {<Vue />}),
		_ => None,
	}
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
			<div class={String::from("fill-[#007396] bg-[#d8dee9] hover:bg-[#007396] hover:fill-[#d8dee9] transition-colors duration-200 p-1 min-h-10 text-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
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
			<div class={String::from("fill-[#3776AB] bg-[#d8dee9] hover:bg-[#3776AB] hover:fill-[#d8dee9] transition-colors duration-200 p-1 min-h-10 text-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
				<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 256 255">
					<path d="M126.916.072c-64.832 0-60.784 28.115-60.784 28.115l.072 29.128h61.868v8.745H41.631S.145 61.355.145 126.77c0 65.417 36.21 63.097 36.21 63.097h21.61v-30.356s-1.165-36.21 35.632-36.21h61.362s34.475.557 34.475-33.319V33.97S194.67.072 126.916.072M92.802 19.66a11.12 11.12 0 0 1 11.13 11.13a11.12 11.12 0 0 1-11.13 11.13a11.12 11.12 0 0 1-11.13-11.13a11.12 11.12 0 0 1 11.13-11.13" />
					<path fill="#ffc331" d="M128.757 254.126c64.832 0 60.784-28.115 60.784-28.115l-.072-29.127H127.6v-8.745h86.441s41.486 4.705 41.486-60.712c0-65.416-36.21-63.096-36.21-63.096h-21.61v30.355s1.165 36.21-35.632 36.21h-61.362s-34.475-.557-34.475 33.32v56.013s-5.235 33.897 62.518 33.897m34.114-19.586a11.12 11.12 0 0 1-11.13-11.13a11.12 11.12 0 0 1 11.13-11.131a11.12 11.12 0 0 1 11.13 11.13a11.12 11.12 0 0 1-11.13 11.13" />
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
			<div class={String::from("text-[#8B3103] bg-[#d8dee9] hover:bg-[#8B3103] hover:text-[#d8dee9] transition-colors duration-200 p-1 min-h-10 ext-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
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
			<div class={String::from("bg-[#d8dee9] hover:bg-[#644eef] transition-colors duration-200 p-1 min-h-10 ext-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
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
			<div class={String::from("bg-[#d8dee9] hover:bg-[#a3be8c] transition-colors duration-200 p-1 min-h-10 text-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
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
			<div class={String::from("text-[#ef5032] bg-[#d8dee9] hover:bg-[#ef5032] hover:text-[#d8dee9] transition-colors duration-200 p-1 min-h-10 ext-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
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
			<div class={String::from("bg-[#d8dee9] hover:bg-[#b9baea] transition-colors duration-200 mx-auto p-1 min-h-10 text-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
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
		<a href="https://pytorch.org/">
			<div class={String::from("fill-[#ee4c2c] bg-[#d8dee9] hover:bg-[#ee4c2c] hover:fill-[#d8dee9] transition-colors duration-200 mx-auto p-1 min-h-10 text-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
				<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 32 32">
					<path d="M16.005.052L6.636 9.427a13.106 13.106 0 0 0 0 18.636a13.105 13.105 0 0 0 18.629 0c5.297-5.188 5.297-13.563.115-18.636l-2.317 2.313c3.859 3.859 3.859 10.145 0 14.005c-3.86 3.859-10.145 3.859-14.005 0c-3.86-3.86-3.86-10.147 0-14.005l6.177-6.172l.776-.885zm4.744 5.183c-.973 0-1.765.792-1.765 1.765a1.763 1.763 0 1 0 1.765-1.765" />
				</svg>
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
			<div class={String::from("text-[#bf616a] bg-[#d8dee9] hover:bg-[#bf616a] hover:text-[#d8dee9] transition-colors duration-200 p-1 min-h-10 ext-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
				<i class="fa-solid fa-dog" style="font-size: 35px;"/>
			</div>
		</a>
	}
}

#[styled_component]
pub fn HTML() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://www.w3schools.com/html/">
			<div class={String::from("text-[#e54c22] bg-[#d8dee9] hover:bg-[#e54c22] hover:text-[#d8dee9] transition-colors duration-200 p-1 min-h-10 ext-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
				<i class="fa-brands fa-html5" style="font-size: 40px;"/>
			</div>
		</a>
	}
}

#[styled_component]
pub fn CSS() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://www.w3schools.com/css/">
			<div class={String::from("text-[#214ce6] bg-[#d8dee9] hover:bg-[#214ce6] hover:text-[#d8dee9] transition-colors duration-200 p-1 min-h-10 ext-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
				<i class="fa-brands fa-css3-alt" style="font-size: 40px;"/>
			</div>
		</a>
	}
}

#[styled_component]
pub fn JS() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://www.javascript.com/">
			<div class={String::from("text-[#e8d44d] bg-[#d8dee9] hover:bg-[#e8d44d] hover:text-[#d8dee9] transition-colors duration-200 p-1 min-h-10 ext-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
				<i class="fa-brands fa-square-js" style="font-size: 40px;"/>
			</div>
		</a>
	}
}

#[styled_component]
pub fn Vue() -> Html {
	let style_class = style!(
		r#"
		aspect-ratio: 1 / 1;
	"#
	)
	.unwrap();
	html! {
		<a href="https://vuejs.org/">
			<div class={String::from("text-[#40b27f] bg-[#d8dee9] hover:bg-[#40b27f] hover:text-[#d8dee9] transition-colors duration-200 p-1 min-h-10 ext-center flex justify-center content-center items-center whitespace-nowrap block rounded drop-shadow-md ") + style_class.get_class_name()}>
				<i class="fa-brands fa-vuejs" style="font-size: 40px;"/>
			</div>
		</a>
	}
}
