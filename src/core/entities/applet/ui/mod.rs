use {crate::near::NearAccountId, leptos::*};

#[island]
pub fn Applet(#[prop(into)] screen_id: String) -> impl IntoView {
	let id = create_rw_signal("vault".to_string());
	let account_id = create_rw_signal(NearAccountId("akaia.near".to_string()));

	create_effect(move |_| {
		use leptos_use::use_window;
		let w = use_window();

		let result = format!(
			"{}.near",
			w.as_ref().unwrap().location().hostname().unwrap()
		);

		account_id.set(NearAccountId(result));
	});

	view! { <akaia-applet id={id} screen={screen_id} query="{}" /> }
}
