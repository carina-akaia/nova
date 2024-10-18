use {crate::entities::expert::*, akaia_graphics::*, dioxus::prelude::*, dioxus_class::prelude::*};

#[derive(PartialEq, Props)]
pub struct ExpertSummaryProps {
	#[props(into)]
	class: Option<String>,
}

pub fn ExpertSummary(cx: Scope<ExpertSummaryProps>) -> Element {
	let ExpertSummaryProps { class } = cx.props;
	let model::Expert { summary, .. } = use_profile(cx);

	let root_class = class!(
		uno![cx; "flex-(~ col)", "gap-4"]
		class.to_owned().unwrap_or("".into())
	);

	render!(Paragraph { class:     root_class.to_string(),
	                    heading:   "Summary",
	                    content:   summary,
	                    with_icon: true, })
}
