use {crate::domain::PortfolioDemo, leptos::*};

#[component]
pub fn IndexPage() -> impl IntoView {
	view! { <PortfolioDemo account_id={"carina.akaia.near".to_owned()}/> }
}
