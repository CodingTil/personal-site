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
			background_primary: "#2e3440".to_string(),
			background_secondary: "#242933".to_string(),
			background_tertiary: "#3B4252".to_string(),

			foreground_primary: "#ECEFF4".to_string(),
			foreground_secondary: "#E5E9f0".to_string(),
			foreground_tertiary: "#D8DEE9".to_string(),

			other_primary: "#065f46".to_string(),
			other_secondary: "#059669".to_string(),
			other_tertiary: "#10b981".to_string(),
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
			/*rainbow_1: "#5197E8".to_string(),
			rainbow_2: "#50E3C2".to_string(),
			rainbow_3: "#F5A623".to_string(),
			rainbow_4: "#F8E71C".to_string(),
			rainbow_5: "#DB2FFF".to_string(),
			rainbow_6: "#FF5757".to_string(),*/
			rainbow_1: "#8fbcbb".to_string(),
			rainbow_2: "#bf616a".to_string(),
			rainbow_3: "#ebcb8b".to_string(),
			rainbow_4: "#5e81ac".to_string(),
			rainbow_5: "#a3be8c".to_string(),
			rainbow_6: "#b48ead".to_string(),

			color_error: "#F44336".to_string(),
			color_success: "#00C853".to_string(),
			color_warning: "#FF7043".to_string(),
			color_info: "#A7FFEB".to_string(),

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

	pub fn set(&self, kind: ThemeKind) {
		self.inner.set(kind)
	}

	pub fn kind(&self) -> ThemeKind {
		(*self.inner).clone()
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
