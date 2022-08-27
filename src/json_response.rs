use axum::{response::{IntoResponse, Response}, http::{header::{self, HeaderName}},};
use bytes::{BufMut, BytesMut, Bytes};
use serde::Serialize;

pub use axum::http::StatusCode;

#[derive(Debug)]
pub enum JsonResponse {
    Ok(StatusCode, Option<Bytes>),
    Error(StatusCode, Option<Bytes>)
}

pub type JsonResult = Result<JsonResponse, JsonResponse>;

#[derive(Debug, Serialize)]
struct JsonError {
    error: String,
}

impl JsonResponse {
    pub fn encode_with<T>(val: &T, status: StatusCode) -> JsonResult where T: Serialize {
        let mut writer = BytesMut::new().writer();
        serde_json::to_writer(&mut writer, val)
            .map_err(|_err| JsonResponse::Error(StatusCode::INTERNAL_SERVER_ERROR, None))?;

        Ok(JsonResponse::Ok(status, Some(writer.into_inner().freeze())))
    }

    pub fn encode<T>(val: &T) -> JsonResult where T: Serialize {
        Self::encode_with(val, StatusCode::OK)
    }

    pub fn error<Msg: std::fmt::Display>(status: StatusCode, message: Msg) -> JsonResult {
        let err = JsonError {
            error: message.to_string()
        };

        let mut writer = BytesMut::new().writer();
        serde_json::to_writer(&mut writer, &err)
            .map_err(|_err| JsonResponse::Error(StatusCode::INTERNAL_SERVER_ERROR, None))?;

        Ok(JsonResponse::Error(status, Some(writer.into_inner().freeze())))
    }
}

const JSON_HEADER: [(HeaderName, &str); 1] = [(header::CONTENT_TYPE, "application/json")];

impl IntoResponse for JsonResponse {
    fn into_response(self) -> Response {
        match self {
            Self::Ok(code, bin_opt) => {
                let body = match bin_opt {
                    Some(bin) => bin,
                    None => Bytes::new(),
                };

                (code,
                    JSON_HEADER,
                    body
                ).into_response()
            }
            Self::Error(code, bin_opt) => {
                let body = match bin_opt {
                    Some(bin) => bin,
                    None => Bytes::new(),
                };

                (code,
                 JSON_HEADER,
                 body
                ).into_response()
            }
        }
    }
}

use crate::calculation_errors::{DieselUsageCalculationError, UnitInjectorFailCalculationError};

impl From<DieselUsageCalculationError> for JsonResponse {
    fn from(err: DieselUsageCalculationError) -> Self {
        use DieselUsageCalculationError::*;

        let status_code = match err {
            InvalidParams(..) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        JsonResponse::error(status_code, err)
            .unwrap_or(JsonResponse::Error(StatusCode::INTERNAL_SERVER_ERROR, None))
    }
}

impl From<UnitInjectorFailCalculationError> for JsonResponse {
    fn from(err: UnitInjectorFailCalculationError) -> Self {
        use UnitInjectorFailCalculationError::*;

        let status_code = match err {
            InvalidParams(..) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        JsonResponse::error(status_code, err)
            .unwrap_or(JsonResponse::Error(StatusCode::INTERNAL_SERVER_ERROR, None))
    }
}
