use {leptos::*, leptos_use::use_window};

#[island]
pub fn Applet(#[prop(into)] route: String) -> impl IntoView {
	let this = use_window();
	let id = create_rw_signal("vault".to_string());
	let account_id = create_rw_signal("akaia.near".to_string());

	create_effect(move |_| {
		let result = format!(
			"{}.near",
			this.as_ref().unwrap().location().hostname().unwrap()
		);

		account_id.set(result);
	});

	view! {
		<style>"akaia-applet-launcher {display:\"block\";width:\"100%\";height:\"100%\";}"</style>
		<akaia-applet-launcher account_id={account_id} applet_id={id} route={route} />
	}
}
