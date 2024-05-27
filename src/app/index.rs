use {
	crate::{account::AccountId, domain::PortfolioDemo},
	leptos::*,
};

#[component]
pub fn IndexPage() -> impl IntoView {
	view! { <PortfolioDemo account_id={AccountId("carina.akaia.near".to_owned())}/> }
}
