use {leptos::*, leptos_use::use_window};

#[island]
pub fn Applet(#[prop(into)] route: String) -> impl IntoView {
	let this = use_window();
	let id = create_rw_signal("".to_string());
	let account_id = create_rw_signal("akaia.near".to_string());

	create_effect(move |_| {
		let domain_segments: Vec<String> = this
			.as_ref()
			.unwrap()
			.location()
			.hostname()
			.unwrap()
			.split(".")
			.map(|subdomain| subdomain.to_string())
			.collect();

		match domain_segments.len() {
			| 1 => account_id.set(format!("{}.near", domain_segments[0])),

			| 2 => account_id.set(format!(
				"{}.{}.near",
				domain_segments[0], domain_segments[1]
			)),

			| 3 => {
				account_id.set(format!(
					"{}.{}.near",
					domain_segments[1], domain_segments[2]
				));

				id.set(format!("{}", domain_segments[0]));
			},

			| _ => logging::error!("Invalid URL"),
		}
	});

	view! {
		<style>"akaia-applet-launcher {display:\"block\";width:\"100%\";height:\"100%\";}"</style>
		<akaia-applet-launcher account_id={account_id} applet_id={id} route={route} />
	}
}
