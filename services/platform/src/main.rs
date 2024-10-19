#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
	use {
		actix_files::Files,
		actix_web::*,
		akaia_commlink::app::*,
		leptos::*,
		leptos_actix::{LeptosRoutes, generate_route_list},
	};

	let conf = get_configuration(None).await.unwrap();
	let addr = conf.leptos_options.site_addr;
	let routes = generate_route_list(CommLinkPlatform);

	println!("\n\n[ 📡 ] Listening on http://{} ...\n\n", &addr);

	HttpServer::new(move || {
		let leptos_options = &conf.leptos_options;
		let site_root = &leptos_options.site_root;

		App::new()
			.service(Files::new("/app", format!("{site_root}/app")))
			.service(Files::new("/static", site_root))
			.service(favicon)
			.leptos_routes(
				leptos_options.to_owned(),
				routes.to_owned(),
				CommLinkPlatform,
			)
			.app_data(web::Data::new(leptos_options.to_owned()))
			.wrap(middleware::Compress::default())
	})
	.bind(&addr)?
	.run()
	.await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
	leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
	let leptos_options = leptos_options.into_inner();
	let site_root = &leptos_options.site_root;

	Ok(actix_files::NamedFile::open(format!(
		"{site_root}/favicon.ico"
	))?)
}
