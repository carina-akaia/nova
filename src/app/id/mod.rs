use {
	crate::core::PortfolioDemo,
	leptos::*,
	leptos_router::{use_params, Params},
};

#[derive(Params, PartialEq)]
struct IdPageParams {
	id: String,
}

#[component]
pub fn IdPage() -> impl IntoView {
	let params = use_params::<IdPageParams>();

	let id = params.with(|params| params.as_ref().map(|params| params.id.clone()).unwrap());

	view! { <PortfolioDemo account_id={id}/> }
}
