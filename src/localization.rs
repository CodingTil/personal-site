use core::fmt::Display;

use stylist::yew::styled_component;
use yew::html::ImplicitClone;
use yew::prelude::*;

use std::default::Default;

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) enum Localization {
	DE,
	#[default]
	EN,
}

impl Display for Localization {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Localization::DE => write!(f, "Deutsch"),
			Localization::EN => write!(f, "English"),
		}
	}
}

impl ImplicitClone for Localization {}

#[derive(Debug, Clone)]
pub(crate) struct LocalizationContext {
	inner: UseStateHandle<Localization>,
}

impl LocalizationContext {
	pub fn new(inner: UseStateHandle<Localization>) -> Self {
		Self { inner }
	}

	pub fn set(&self, localization: Localization) {
		self.inner.set(localization);
	}

	pub fn get(&self) -> Localization {
		(*self.inner).clone()
	}
}

impl PartialEq for LocalizationContext {
	fn eq(&self, other: &Self) -> bool {
		self.inner == other.inner
	}
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct LocalizationProviderProps {
	pub children: Children,
}

#[styled_component]
pub(crate) fn LocalizationProvider(props: &LocalizationProviderProps) -> Html {
	let localization = use_state(Localization::default);

	let localization_ctx = LocalizationContext::new(localization);

	html! {
		<ContextProvider<LocalizationContext> context={localization_ctx}>
			{props.children.clone()}
		</ContextProvider<LocalizationContext>>
	}
}

#[hook]
pub(crate) fn use_localization() -> LocalizationContext {
	use_context::<LocalizationContext>().unwrap()
}
