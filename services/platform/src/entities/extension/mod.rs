mod ui;

use {
	self::ui::ExtensionShell,
	leptos::*,
	leptos_router::{Params, use_params, use_query_map},
};

#[derive(Params, PartialEq)]
pub struct ExtensionProviderRouteParams {
	app_route: String,
}

#[component]
pub fn ExtensionProvider(#[prop(into)] props: String) -> impl IntoView {
	let app_route = use_params::<ExtensionProviderRouteParams>().with(|params| {
		params
			.as_ref()
			.map(|params| params.app_route.clone())
			.unwrap()
	});

	let route_query_json = move || {
		use_query_map().with(|queryMap| {
			String::from("{")
				+ queryMap
					.0
					.clone()
					.into_iter()
					.map(|(key, value)| format!("{:#?}:{:#?}", key, value))
					.collect::<Vec<String>>()
					.join(",")
					.as_str() + "}"
		})
	};

	view! { <ExtensionShell route={app_route} query={route_query_json()} props={props} /> }
}
