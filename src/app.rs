mod applet_route;
mod error;
mod index;

use {
	self::{applet_route::AppletScreen, error::ErrorScreen, index::Dashboard},
	crate::IMPORT_MAP,
	leptos::*,
	leptos_meta::*,
	leptos_query::{provide_query_client_with_options_and_persister, query_persister},
	leptos_query_devtools::LeptosQueryDevtools,
	leptos_router::*,
};

#[component]
pub fn Nova() -> impl IntoView {
	provide_meta_context();

	provide_query_client_with_options_and_persister(
		Default::default(),
		query_persister::LocalStoragePersister,
	);

	view! {
		<Title text={"AKAIA"}/>
		<Script r#type_="importmap">{IMPORT_MAP}</Script>
		<Script id={"unocss"} src={"/static/vendor/uno_attributify.runtime.js"}/>
		<Script r#type_="module" id={"applet-runtime"} src={"/static/vendor/akaia-applet-runtime.js"}/>
		<Stylesheet id={"leptos"} href={"/app/akaia_nova.css"}/>
		<Body attr:m={"0"} attr:h={"100vh"} attr:flex={"~ col"}/>

		<Router>
			<main p={"4"} h={"100%"} flex={"~ col"} items={"center"} justify={"center"}>
				<Routes>
					<Route path={"/"} view={Dashboard}/>
					<Route path={"/:applet_route"} view={AppletScreen}/>
					<Route path={"/error"} view={ErrorScreen}/>
					<Route path={"/*any"} view={Dashboard}/>
				</Routes>
			</main>
		</Router>

		<LeptosQueryDevtools/>
	}
}
