use {dioxus::prelude::*, dioxus_class::prelude::*};

#[derive(Props)]
pub struct ExpertsLayoutProps<'a> {
	#[props(default = "100%")]
	width:    Option<&'a str>,
	#[props(default = "100%")]
	height:   Option<&'a str>,
}

pub fn ExpertsLayout<'a>(cx: Scope<'a, ExpertsLayoutProps<'a>>) -> Element {
	let ExpertsLayoutProps { width, height } = cx.props;

	let root_class = class!(uno![cx; "w-full", "h-full", "overflow-auto"]);

	render!(
    div { class: root_class, width: width.to_owned(), height: height.to_owned(), Router::<ExpertsRoute> {} }
)
}
