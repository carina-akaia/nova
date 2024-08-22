use {
	crate::{near::NearAccountId, widgets::Vault},
	leptos::*,
	leptos_router::{use_params, Params},
};

#[island]
fn Widget(#[prop(into)] id: String) -> impl IntoView {
	let account_id = create_rw_signal(NearAccountId("akaia.near".to_string()));

	create_effect(move |_| {
		use leptos_use::use_window;
		let w = use_window();

		let result = format!(
			"{}.near",
			w.as_ref().unwrap().location().hostname().unwrap()
		);

		account_id.set(NearAccountId(result));
	});

	view! { <Vault account_id_signal={account_id}/> }
}

#[derive(Params, PartialEq)]
struct WidgetPageParams {
	widget_id: String,
}

#[component]
pub fn WidgetPage() -> impl IntoView {
	let params = use_params::<WidgetPageParams>();

	let widget_id = params.with(|params| {
		params
			.as_ref()
			.map(|params| params.widget_id.clone())
			.unwrap()
	});

	view! { <Widget id={widget_id}/> }
}
