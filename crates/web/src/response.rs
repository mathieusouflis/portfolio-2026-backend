use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    code: u16,
    data: T,
    message: String,
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

pub trait IntoStatusCode {
    fn into_status_code(self) -> StatusCode;
}

impl IntoStatusCode for StatusCode {
    fn into_status_code(self) -> StatusCode {
        self
    }
}

impl IntoStatusCode for u16 {
    fn into_status_code(self) -> StatusCode {
        StatusCode::from_u16(self).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl IntoStatusCode for i32 {
    fn into_status_code(self) -> StatusCode {
        u16::try_from(self)
            .map(u16::into_status_code)
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub fn to_response<T>(code: impl IntoStatusCode, data: T, message: String) -> Response<T> {
    Response {
        code: code.into_status_code().as_u16(),
        data,
        message,
    }
}
