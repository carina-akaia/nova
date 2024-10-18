use {crate::entities::expert::{model::Contacts, *},
     akaia_graphics::*,
     dioxus::prelude::*,
     dioxus_class::prelude::*,
     dioxus_free_icons::{icons::fa_brands_icons::*, Icon}};

#[derive(PartialEq, Props)]
pub struct ExpertHeadlineProps {
	#[props(into)]
	class: Option<String>,
}

pub fn ExpertHeadline(cx: Scope<ExpertHeadlineProps>) -> Element {
	let ExpertHeadlineProps { class } = cx.props;

	let Expert { name,
	             roles,
	             avatar_url,
	             banner_url,
	             status,
	             contacts: Contacts { email },
	             links,
	             .. } = use_profile(cx);

	let status_string = status.to_owned().unwrap_or("".into());

	let website_link = if let Some(link) = &links.website {
		rsx!(
			li { Link { address: link, label: "NEAR Social", with_icon: true } }
		)
	} else {
		rsx!(Element::None)
	};

	let linkedin_link = if let Some(link) = &links.linkedin {
		rsx!(
			li {
				Link {
					address: link,
					label: "LinkedIn",
					with_icon: true,
					custom_icon: render! {
			Icon { class : uno![cx; "fill-current"], width : 24, height : 24, icon : FaLinkedin }
		}
				}
			}
		)
	} else {
		rsx!(Element::None)
	};

	let root_class = class!(
		uno![cx;
			"flex-(~ col)",
			"justify-center",
			"items-center",
			"gap-8",
			"border-(gray-5)",
		]

		class.to_owned().unwrap_or("".into())
	);

	let root_style = format!(
	                         "&:before {{ background-image: url({:?}) }};",
	                         banner_url.to_owned().unwrap_or("".to_owned())
	);

	let credentials_class = uno![cx;
		"flex-(~ col)", "items-end", "gap-6", "line-height-none", "pt-4", "print-pt-2"
	];

	let status_class = uno![cx;
		"m-0", "text-(blue-5)", "hidden" => status_string.is_empty(), "print-hidden"
	];

	let linktree_class = uno![cx;
		"flex-(~ col)", "items-start", "gap-4", "list-none", "m-0", "p-0", "pt-4", "print-pt-2"
	];

	let role_list_class = uno![cx;
		"flex-(~ wrap)", "gap-4", "print-gap-6", "p-0", "m-0", "list-none"
	];

	render!(
		header { class: root_class, style: "{root_style}",
			div { class: uno![cx; "flex-(~)", "justify-center", "items-start", "gap-8"],
				div { class: credentials_class,
					div { class: uno![cx; "flex-(~ col)", "items-end", "gap-1"],
						h1 { class: uno![cx; "m-0", "text-9", "font-600"], format!("{}", name) }
						span { class: status_class, format!("{}", status_string) }
					}

					Link { address: "mailto:{email}", class: uno![cx; "text-4"] }
				}

				img {
					class: uno![cx; "rounded-full", "w-38", "h-38", "p-2", "print-p-0", "bg-[#26001e]"],
					// Prevent the image from being stretched before the class is applied.
					width: 152,
					height: 152,
					src: "{avatar_url}"
				}

				ul { class: linktree_class, website_link, linkedin_link }
			}

			ul { class: role_list_class, roles.iter().map(|role| rsx!( Badge { text: role } )) }
		}
	)
}
