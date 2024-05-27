mod index;
mod not_found;

use {index::*, leptos::*, leptos_meta::*, leptos_query::*,
     leptos_query_devtools::LeptosQueryDevtools, leptos_router::*, not_found::*};

#[component]
pub fn Nova() -> impl IntoView {
	provide_meta_context();

	provide_query_client_with_options_and_persister(
	                                                Default::default(),
	                                                query_persister::LocalStoragePersister,
	);

	view! {
		<Stylesheet id={"leptos"} href={"/app/akaia_nova.css"}/>
		<Title text={"AKAIA"}/>

		<Router>
			<main>
				<Routes>
					<Route path={"/"} view={IndexPage}/>
					<Route path={"/*any"} view={NotFoundPage}/>
				</Routes>
			</main>
		</Router>

		<LeptosQueryDevtools/>
	}
}
