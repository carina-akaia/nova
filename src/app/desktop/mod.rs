use {crate::AppShell, leptos::*};

#[component]
pub fn NovaDesktop() -> impl IntoView {
	view! { <AppShell props={r#"{ "name": "akaia" }"#} /> }
}
