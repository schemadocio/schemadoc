mod branches;
mod common;
mod projects;
mod schema;
mod utils;
mod versions;

use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::web::PayloadConfig;
use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use tokio::sync::RwLock;

use crate::app_state::AppState;
use crate::settings::Settings;
use crate::web::branches::get_branches_api_scope;
use crate::web::common::get_common_api_scope;
use crate::web::projects::get_projects_api_scope;
use crate::web::versions::get_versions_api_scope;

pub type AppStateType = RwLock<AppState>;

fn get_ui_service(settings: &Settings) -> Files {
    let index_path = format!("{}/index.html", settings.frontend_static_files);

    async fn default_handler(
        req: ServiceRequest,
        index_path: String,
    ) -> error::Result<ServiceResponse> {
        let (req, _) = req.into_parts();
        let file = NamedFile::open_async(index_path).await?;
        let res = file.into_response(&req);
        Ok(ServiceResponse::new(req, res))
    }

    Files::new("/", settings.frontend_static_files.clone())
        .prefer_utf8(true)
        .index_file("a++--")
        .default_handler(move |req| default_handler(req, index_path.clone()))
}

pub async fn serve(host: &str, port: u16) -> anyhow::Result<()> {
    let settings = web::Data::new(Settings::from_env()?);

    let state = AppState::from_settings(&settings).await?;
    let state = web::Data::new(RwLock::new(state));

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

        App::new()
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .app_data(state.clone())
            .app_data(settings.clone())
            .app_data(json_config)
            .app_data(payload_config)
            .service(
                web::scope("/api/v1")
                    .service(get_common_api_scope())
                    .service(get_versions_api_scope())
                    .service(get_branches_api_scope())
                    .service(get_projects_api_scope()),
            )
            .service(get_ui_service(&settings))
    })
    .bind((host, port))?
    .run()
    .await?;

    Ok(())
}
