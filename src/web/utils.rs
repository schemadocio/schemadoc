use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::Serialize;

pub fn json_response<T: Serialize>(status: StatusCode, body: &T) -> HttpResponse {
    let Ok(bytes) = serde_json::to_vec(body) else {
        return HttpResponse::InternalServerError().finish();
    };

    HttpResponseBuilder::new(status)
        .content_type(ContentType::json())
        .body(BoxBody::new(bytes))
}
