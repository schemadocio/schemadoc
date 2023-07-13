mod common;
mod datasources;
mod projects;
mod schema;
mod utils;
mod versions;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::web::PayloadConfig;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use anyhow::bail;
use tokio::sync::RwLock;

use crate::app_state::AppState;
use crate::config::Config;
use crate::storage::{LocalStorage, Storage};
use crate::web::common::get_common_api_scope;
use crate::web::datasources::get_datasource_api_scope;
use crate::web::projects::get_projects_api_scope;
use crate::web::versions::get_versions_api_scope;

pub type AppStateType = RwLock<AppState>;

pub async fn serve(host: &str, port: u16) -> anyhow::Result<()> {
    let config = web::Data::new(Config::from_env());

    let storage = if config.persistence == "local" {
        Storage::Local(LocalStorage::new(&config.persistence_path))
    } else {
        bail!("Persistence {} not supported", config.persistence)
    };

    let state = web::Data::new(RwLock::new(AppState::read(storage).await?));

    HttpServer::new(move || {
        let cors = Cors::permissive();

        let json_config = web::JsonConfig::default()
            .limit(10 * 1024 * 1024)
            // .content_type(|mime| {
            //     mime.type_() == Mime::TEXT && mime.subtype() == mime::PLAIN
            // })
            .error_handler(|err, _req| {
                dbg!(&err);
                // TODO: Handle different kinds of json errors
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        let payload_config = PayloadConfig::default().limit(10 * 1024 * 1024);

        let files = Files::new("/static", config.frontend_static_files.clone())
            .prefer_utf8(true)
            .index_file("index.html");

        App::new()
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .app_data(state.clone())
            .app_data(config.clone())
            .app_data(json_config)
            .app_data(payload_config)
            .service(files)
            .service(
                web::scope("/v1")
                    .service(get_common_api_scope())
                    .service(get_versions_api_scope())
                    .service(get_projects_api_scope())
                    .service(get_datasource_api_scope()),
            )
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
