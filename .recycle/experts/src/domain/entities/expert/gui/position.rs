use {crate::entities::expert::{model::Position, *},
     akaia_graphics::Paragraph,
     dioxus::prelude::*,
     dioxus_class::prelude::*,
     dioxus_free_icons::{icons::hi_outline_icons::HiOfficeBuilding, Icon}};

#[derive(PartialEq, Props)]
pub struct ExpertPositionProps {
	#[props(into)]
	class: Option<String>,
	data:  model::Position,
}

pub fn ExpertPosition(cx: Scope<ExpertPositionProps>) -> Element {
	let ExpertPositionProps { class, data } = cx.props;

	let Position { project_name,
	               summary,
	               .. } = data;

	let summary_string = summary.to_owned().unwrap_or("".into());

	let root_class = class!(
		uno![cx; "flex-(~)", "gap-3"]
		class.to_owned().unwrap_or("".into())
	);

	render!(
		div { class: root_class,
			Icon {
				class: uno![cx; "fill-transparent", "min-w-fit", "p-3", "bg-[#26001e]", "rounded-full"],
				width: 30,
				height: 30,
				icon: HiOfficeBuilding
			}

			Paragraph { heading: project_name, content: summary_string }
		}
	)
}
