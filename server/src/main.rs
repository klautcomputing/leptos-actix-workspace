use actix_files::Files;
use actix_web::*;
use app::*;
use leptos::*;
use leptos_actix::{generate_route_list, LeptosRoutes};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

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
