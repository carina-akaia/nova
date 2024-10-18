use {crate::entities::expert::*,
     akaia_graphics::*,
     dioxus::prelude::*,
     dioxus_class::prelude::*,
     dioxus_free_icons::{icons::hi_outline_icons::*, Icon}};

#[derive(PartialEq, Props)]
pub struct ExpertSkillsetProps {
	#[props(into)]
	class: Option<String>,
}

pub fn ExpertSkillset(cx: Scope<ExpertSkillsetProps>) -> Element {
	let ExpertSkillsetProps { class } = cx.props;
	let model::Expert { skills, .. } = use_profile(cx);

	let skill_badges = skills.iter()
	                         .map(|skill| rsx!(Badge { text: "{skill.name}", }));

	let root_class = class!(
		uno![cx; "flex-(~ col)", "gap-4"]
		class.to_owned().unwrap_or("".into())
	);

	let heading_class = uno![cx;
		"text-6", "font-600", "flex-(~)", "gap-4", "items-center", "m-0",
		"p-3", "print-p-0", "bg-[#26001e]", "rounded-4"
	];

	render!(
		aside { class: root_class,
			h2 { class: heading_class,
				Icon { class: uno![cx; "fill-transparent"], width: 30, height: 30, icon: HiStar }
				span { "Skills" }
			}

			div { class: uno![cx; "flex-(~ wrap)", "gap-3"], skill_badges }
		}
	)
}
