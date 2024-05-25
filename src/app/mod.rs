mod index;
mod not_found;

use {index::*, leptos::*, leptos_meta::*, leptos_router::*, not_found::*};

#[component]
pub fn App() -> impl IntoView {
	provide_meta_context();

	view! {
		<Stylesheet id="leptos" href="/pkg/akaia_nova.css"/>
		<Title text="Welcome to Leptos"/>

		<Router>
			<main>
				<Routes>
					<Route path="/" view={IndexPage}/>
					<Route path="/*any" view={NotFoundPage}/>
				</Routes>
			</main>
		</Router>
	}
}
