mod ui;

use {
	self::ui::AppShellWidgetViewport,
	leptos::*,
	leptos_router::{use_params, use_query_map, Params},
};

#[derive(Params, PartialEq)]
pub struct ApplicationShellRouteParams {
	widget_route: String,
}

#[component]
pub fn AppShell(#[prop(into)] props: String) -> impl IntoView {
	let widget_route = use_params::<ApplicationShellRouteParams>().with(|params| {
		params
			.as_ref()
			.map(|params| params.widget_route.clone())
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

	view! { <AppShellWidgetViewport route={widget_route} query={route_query_json()} props={props} /> }
}
