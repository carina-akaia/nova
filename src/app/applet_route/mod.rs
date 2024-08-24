use {
	crate::Applet,
	leptos::*,
	leptos_router::{use_params, Params},
};

#[derive(Params, PartialEq)]
struct AppletScreenParams {
	applet_route: String,
}

#[component]
pub fn AppletScreen() -> impl IntoView {
	let routeParams = use_params::<AppletScreenParams>();

	// let routeQuery = use_query_map();
	//
	// let queryJson = move || {
	// 	routeQuery.with(|queryMap| {
	// 		queryMap
	// 			.0
	// 			.into_iter()
	// 			.reduce(|a, b| format!("{:?}:{:?}", a, b))
	// 			.unwrap();
	// 	})
	// };

	let applet_route = routeParams.with(|params| {
		params
			.as_ref()
			.map(|params| params.applet_route.clone())
			.unwrap()
	});

	view! { <Applet route={applet_route}/> }
}
