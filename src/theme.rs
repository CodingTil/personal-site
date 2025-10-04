use std::ops::Deref;

use once_cell::sync::Lazy;
use stylist::yew::styled_component;
use yew::html::ImplicitClone;
use yew::prelude::*;

use std::default::Default;

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) enum ThemeKind {
	#[default]
	Dark,
}

impl ImplicitClone for ThemeKind {}

impl ThemeKind {
	pub fn current(&self) -> &Theme {
		static DARK_THEME: Lazy<Theme> = Lazy::new(|| Theme {
			// Richer, deeper backgrounds with better contrast
			background_primary: "#0F172A".to_string(),   // Slate 900
			background_secondary: "#1E293B".to_string(), // Slate 800
			background_tertiary: "#334155".to_string(),  // Slate 700

			// Brighter foregrounds for better readability
			foreground_primary: "#F1F5F9".to_string(),   // Slate 100
			foreground_secondary: "#E2E8F0".to_string(), // Slate 200
			foreground_tertiary: "#CBD5E1".to_string(),  // Slate 300

			// Modern accent colors
			other_primary: "#059669".to_string(),
			other_secondary: "#10b981".to_string(),
			other_tertiary: "#34d399".to_string(),
			other_quaternary: "#6ee7b7".to_string(),

			..Default::default()
		});

		match self {
			ThemeKind::Dark => &DARK_THEME,
		}
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Theme {
	pub rainbow_1: String,
	pub rainbow_2: String,
	pub rainbow_3: String,
	pub rainbow_4: String,
	pub rainbow_5: String,
	pub rainbow_6: String,

	pub color_error: String,
	pub color_success: String,
	pub color_warning: String,
	pub color_info: String,

	pub background_primary: String,
	pub background_secondary: String,
	pub background_tertiary: String,

	pub foreground_primary: String,
	pub foreground_secondary: String,
	pub foreground_tertiary: String,

	pub other_primary: String,
	pub other_secondary: String,
	pub other_tertiary: String,
	pub other_quaternary: String,
}

impl Default for Theme {
	fn default() -> Theme {
		Theme {
			// Modern vibrant rainbow palette with better saturation
			rainbow_1: "#60A5FA".to_string(), // Modern blue
			rainbow_2: "#F472B6".to_string(), // Modern pink
			rainbow_3: "#FBBF24".to_string(), // Modern amber
			rainbow_4: "#A78BFA".to_string(), // Modern purple
			rainbow_5: "#34D399".to_string(), // Modern emerald
			rainbow_6: "#FB923C".to_string(), // Modern orange

			color_error: "#F87171".to_string(),
			color_success: "#34D399".to_string(),
			color_warning: "#FBBF24".to_string(),
			color_info: "#60A5FA".to_string(),

			background_primary: "#000000".to_string(),
			background_secondary: "#000000".to_string(),
			background_tertiary: "#000000".to_string(),

			foreground_primary: "#000000".to_string(),
			foreground_secondary: "#000000".to_string(),
			foreground_tertiary: "#000000".to_string(),

			other_primary: "#000000".to_string(),
			other_secondary: "#000000".to_string(),
			other_tertiary: "#000000".to_string(),
			other_quaternary: "#000000".to_string(),
		}
	}
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeContext {
	inner: UseStateHandle<ThemeKind>,
}

impl ThemeContext {
	pub fn new(inner: UseStateHandle<ThemeKind>) -> Self {
		Self { inner }
	}
}

impl Deref for ThemeContext {
	type Target = Theme;

	fn deref(&self) -> &Self::Target {
		self.inner.current()
	}
}

impl PartialEq for ThemeContext {
	fn eq(&self, rhs: &Self) -> bool {
		*self.inner == *rhs.inner
	}
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ThemeProviderProps {
	pub children: Children,
}

#[styled_component]
pub(crate) fn ThemeProvider(props: &ThemeProviderProps) -> Html {
	let theme_kind = use_state(ThemeKind::default);

	let theme_ctx = ThemeContext::new(theme_kind);

	html! {
		<ContextProvider<ThemeContext> context={theme_ctx}>
			{props.children.clone()}
		</ContextProvider<ThemeContext>>
	}
}

#[hook]
pub(crate) fn use_theme() -> ThemeContext {
	use_context::<ThemeContext>().unwrap()
}
