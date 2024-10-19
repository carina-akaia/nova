use {crate::ExtensionShell, leptos::*};

#[component]
pub fn CommLinkDesktop() -> impl IntoView {
	view! { <ExtensionShell props={r#"{ "name": "akaia" }"#} /> }
}
