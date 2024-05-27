use {
	crate::{domain::PortfolioDemo, near_account::AccountId},
	leptos::*,
};

#[component]
pub fn IndexPage() -> impl IntoView {
	view! { <PortfolioDemo account_id={AccountId("carina.akaia.near".to_owned())}/> }
}
