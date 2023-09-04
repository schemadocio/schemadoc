use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponseInner<T: Serialize> {
    result: T,
}

pub struct ApiResponse {
    bytes: Vec<u8>,
    status_code: StatusCode,
}

impl<T: Serialize> From<(T, StatusCode)> for ApiResponse {
    fn from((result, status_code): (T, StatusCode)) -> Self {
        let inner = ApiResponseInner { result };

        if let Ok(bytes) = serde_json::to_vec(&inner) {
            ApiResponse { bytes, status_code }
        } else {
            ApiResponse {
                bytes: Vec::new(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }
}

impl<T: Serialize> From<(T,)> for ApiResponse {
    fn from(result: (T,)) -> Self {
        (result.0, StatusCode::OK).into()
    }
}

impl Responder for ApiResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponseBuilder::new(self.status_code)
            .content_type(ContentType::json())
            .body(BoxBody::new(self.bytes))
    }
}
