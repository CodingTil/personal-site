use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;

#[derive(Properties, PartialEq)]
pub struct BaseLayoutProps {
	#[prop_or_default]
	pub children: Children,
}

#[function_component]
pub fn BaseLayout(props: &BaseLayoutProps) -> Html {
	html! {
		<>
			<Header />
			<div>{ for props.children.iter() }</div>
			/* Footer */
			<div class="min-w-full border-solid border-t-2 border-background-secondary">
				<section id="footer" class=" mx-0 xl:mx-auto max-w-7xl flex flex-col xl:flex-row flex-wrap items-center justify-evenly space-y-1 py-8">
					<a href="mailto:me@tilmohr.com" class="flex items-center text-foreground-primary" >
						<i class="fa-solid fa-envelope"></i>
						<span class="ml-2">{"me@tilmohr.com"}</span>
					</a>
					<a href="https://github.com/CodingTil" class="flex items-center text-foreground-primary">
						<i class="fa-brands fa-square-github"></i>
						<span class="ml-2">{"CodingTil"}</span>
					</a>
					<a href="https://linkedin.com/in/tilmohr" class="mx-2 flex items-center text-foreground-primary">
						<i class="fa-brands fa-linkedin"></i>
						<span class="ml-2 whitespace-nowrap" >{"tilmohr"}</span>
					</a>
				</section>
			</div>
			<Footer />
		</>
	}
}
