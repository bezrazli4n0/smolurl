use crate::{db, middleware, service, state, utils};
use actix_web::{web, App, HttpServer};
use std::path::PathBuf;

pub fn config_app_service(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/api")
                .wrap(middleware::Auth)
                .service(service::create_link)
                .service(service::get_link)
                .service(service::get_links),
        )
        .service(
            web::scope("/auth")
                .service(service::register)
                .service(service::login),
        )
        .service(web::scope("/r").service(service::redirect))
        .service(service::ping);
}

pub async fn run(
    host: impl AsRef<str>,
    port: u16,
    db_url: impl AsRef<str>,
    private_key_path: PathBuf,
) -> Result<(), anyhow::Error> {
    let app_state = state::App {
        db_pool: db::init(db_url.as_ref()).await?,
        keypair: utils::load_keypair_from_file(private_key_path)?,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .configure(config_app_service)
    })
    .bind((host.as_ref(), port))
    .expect("Unable to bind server")
    .run()
    .await
    .expect("Unable to run server");

    Ok(())
}
