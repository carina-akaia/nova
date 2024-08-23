use {
	crate::core::Applet,
	leptos::*,
	leptos_router::{use_params, Params},
};

#[derive(Params, PartialEq)]
struct AppletScreenParams {
	screen_id: String,
}

#[component]
pub fn AppletScreen() -> impl IntoView {
	let params = use_params::<AppletScreenParams>();

	let screen_id = params.with(|params| {
		params
			.as_ref()
			.map(|params| params.screen_id.clone())
			.unwrap()
	});

	view! { <Applet screen_id={screen_id}/> }
}
