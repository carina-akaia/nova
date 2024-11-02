use {crate::ExtensionProvider, leptos::*};

#[component]
pub fn CommLinkDesktop() -> impl IntoView {
	view! { <ExtensionProvider props={r#"{ "name": "akaia" }"#} /> }
}
