use crate::ApiResult;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebError {
    #[error("[Business]:{0}")]
    Business(u16, String),
    #[error("[WEB]:{0}")]
    Web(#[from] actix_web::Error),
    #[cfg(feature = "database")]
    #[error("[DB]:{0}")]
    DataSource(#[from] jieto_db::error::DbError),
    #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
    #[error("[EX]:{0}")]
    Execution(#[from] sqlx::Error),
    #[error("[AU]:{0}")]
    #[cfg(feature = "auth")]
    Auth(#[from] jieto_auth::error::AuthError),
}

impl ResponseError for WebError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            WebError::Web(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            #[cfg(feature = "database")]
            WebError::DataSource(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            #[cfg(any(feature = "mysql", feature = "postgres", feature = "sqlite"))]
            WebError::Execution(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            #[cfg(feature = "auth")]
            WebError::Auth(_) => actix_web::http::StatusCode::FORBIDDEN,
            WebError::Business(..) => actix_web::http::StatusCode::default(),
        }
    }
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let (code, msg) = match self {
            WebError::Business(biz_code, msg) => (*biz_code, msg.clone()),
            _ => (status.as_u16(), format!("{}", self)),
        };

        let res = ApiResult::<()> {
            code,
            msg,
            data: None,
        };

        HttpResponse::build(status).json(res)
    }
}
