mod desktop;
mod error;
mod index;

use {
	self::{desktop::CommLinkDesktop, error::ErrorScreen, index::Dashboard},
	crate::IMPORT_MAP,
	leptos::*,
	leptos_meta::*,
	leptos_query::{provide_query_client_with_options_and_persister, query_persister},
	leptos_query_devtools::LeptosQueryDevtools,
	leptos_router::*,
};

#[component]
pub fn CommLinkPlatform() -> impl IntoView {
	provide_meta_context();

	provide_query_client_with_options_and_persister(
		Default::default(),
		query_persister::LocalStoragePersister,
	);

	view! {
		<Title text={"CommLink"}/>
		<Script r#type_="importmap">{IMPORT_MAP}</Script>
		<Script r#type_="module" id={"framework"} src={"/static/packages/extension-framework/index.js"}/>
		<Script id={"unocss"} src={"/static/packages/uno_attributify.runtime.js"}/>
		<Stylesheet id={"leptos"} href={"/app/akaia_commlink.css"}/>
		<Body attr:m={"0"} attr:h={"100vh"} attr:flex={"~ col"}/>

		<Router>
			<main p={"4"} h={"100%"} flex={"~ col"} items={"center"} justify={"center"}>
				<Routes>
					<Route path={"/"} view={Dashboard}/>
					<Route path={"/:app_route"} view={CommLinkDesktop}/>
					<Route path={"/error"} view={ErrorScreen}/>
					<Route path={"/*any"} view={Dashboard}/>
				</Routes>
			</main>
		</Router>

		<LeptosQueryDevtools/>
	}
}
