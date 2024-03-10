use yew::prelude::*;
use gloo::timers::callback::Timeout;

struct Block {
	subtext: String,
	is_whitespace: bool,
}

#[derive(Debug, PartialEq)]
pub enum Msg {
	NextChar,
	Backspace,
}

#[derive(Debug, PartialEq, Properties)]
pub struct TypewriterProps {
	#[prop_or(None)]
	pub class: Option<String>,
	pub texts: Vec<String>,
}

pub struct Typewriter {
	texts: Vec<Vec<Block>>,
	texts_index: usize,
	curr_text_index: usize,
	curr_disp_text: String,
	div: web_sys::Element,
}

impl Typewriter {
	fn schedule_next_char(&mut self, ctx: &Context<Self>, immediate: bool) {
		let delay = if immediate { 0 } else { 80 };
		let link = ctx.link().clone();
		Timeout::new(delay, move || link.send_message(Msg::NextChar)).forget();
	}

	fn schedule_backspace(&mut self, ctx: &Context<Self>, delayed: bool) {
		let delay = if delayed { 500 } else { 40 };
		let link = ctx.link().clone();
		Timeout::new(delay, move || link.send_message(Msg::Backspace)).forget();
	}

	fn character_mapper(c: char) -> String {
		match c {
			'\n' => "<br>".to_string(),
			'\t' => "&nbsp;&nbsp;&nbsp;&nbsp;".to_string(),
            '"' => "&quot;".to_string(),
			_ => c.to_string(),
		}
	}

	fn iswhitespace(c: char) -> bool {
		match c {
			' ' | '\t' | '\n' => true,
			_ => false,
		}
	}

	fn text_mapper(s: String) -> Vec<Block> {
		s.chars()
			.map(|c| Block {
				subtext: Typewriter::character_mapper(c),
				is_whitespace: Typewriter::iswhitespace(c),
			})
			.collect()
	}
}

impl Component for Typewriter {
	type Message = Msg;
	type Properties = TypewriterProps;

	fn create(ctx: &Context<Self>) -> Self {
		let div = gloo_utils::document().create_element("div").unwrap();
		div.set_class_name(ctx.props().class.as_ref().map(|s| s.as_str()).unwrap_or(""));
		Typewriter {
			texts: ctx.props().texts.clone().into_iter().map(Typewriter::text_mapper).collect(),
			texts_index: 0,
			curr_text_index: 0,
			curr_disp_text: String::new(),
			div,
		}
	}

	fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::NextChar => {
				if self.curr_text_index < self.texts[self.texts_index].len() {
					let block = &self.texts[self.texts_index][self.curr_text_index];
					self.curr_disp_text.push_str(&block.subtext.as_str());
					self.curr_text_index += 1;
					self.schedule_next_char(ctx, block.is_whitespace);
				} else {
					self.schedule_backspace(ctx, true);
				}
			}
			Msg::Backspace => {
				if self.curr_text_index > 0 {
					self.curr_disp_text.truncate(self.curr_disp_text.len() - self.texts[self.texts_index][self.curr_text_index - 1].subtext.len());
					self.curr_text_index -= 1;
					self.schedule_backspace(ctx, false);
				} else {
					self.texts_index = (self.texts_index + 1) % self.texts.len();
					self.curr_text_index = 0;
					self.curr_disp_text.clear();
					self.schedule_next_char(ctx, false);
				}
			}
		}
		true
	}

	fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
		// unsupported
		false
	}

	fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
		if first_render {
			ctx.link().send_message(Msg::NextChar);
		}
	}

	fn view(&self, _ctx: &Context<Self>) -> Html {
		self.div.set_inner_html(&self.curr_disp_text.clone());
		Html::VRef(self.div.clone().into())
	}
}
