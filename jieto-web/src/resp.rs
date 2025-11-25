use crate::error::WebError;
use crate::{BusinessError, JietoResult};
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize, Default, Clone, Debug)]
pub struct ApiResult<T>
where
    T: Serialize,
{
    pub code: u16,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> Responder for ApiResult<T>
where
    T: Serialize,
{
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl<T> ApiResult<T>
where
    T: Serialize,
{
    const SUCCESS_CODE: u16 = 0;

    pub fn ok(data: T) -> JietoResult<T> {
        Ok(ApiResult {
            code: Self::SUCCESS_CODE,
            msg: "success".to_string(),
            data: Some(data),
        })
    }

    pub fn ok_data(data: Option<T>) -> JietoResult<T> {
        Ok(ApiResult {
            code: Self::SUCCESS_CODE,
            msg: "success".to_string(),
            data,
        })
    }

    pub fn ok_custom(msg: &str, data: Option<T>) -> JietoResult<T> {
        Ok(ApiResult {
            code: Self::SUCCESS_CODE,
            msg: msg.to_string(),
            data,
        })
    }

    pub fn ok_empty() -> JietoResult<T> {
        Ok(ApiResult {
            code: Self::SUCCESS_CODE,
            msg: "success".to_string(),
            data: None,
        })
    }

    pub fn error(business_error: &BusinessError) -> JietoResult<T> {
        Err(WebError::Business(
            business_error.code,
            String::from(business_error.msg),
        ))
    }

    pub fn error_custom(code: u16, msg: &str) -> JietoResult<()> {
        Err(WebError::Business(code, String::from(msg)))
    }
}
