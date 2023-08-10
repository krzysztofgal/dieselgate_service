use axum::{
    http::header::{self, HeaderName},
    response::{IntoResponse, Response},
};
use bytes::{BufMut, Bytes, BytesMut};
use serde::Serialize;

pub use axum::http::StatusCode;

#[derive(Debug)]
pub struct JsonResponse(StatusCode, Option<Bytes>);
#[derive(Debug)]
pub struct JsonErrorResponse(StatusCode, Option<Bytes>);

pub type JsonResult = Result<JsonResponse, JsonErrorResponse>;

// return error as json object { error: "..." }
#[derive(Debug, Serialize)]
struct JsonError {
    error: String,
}

impl JsonResponse {
    pub fn encode_with<T>(val: &T, status: StatusCode) -> JsonResult
    where
        T: Serialize,
    {
        let mut writer = BytesMut::new().writer();
        serde_json::to_writer(&mut writer, val)
            .map_err(|_err| JsonErrorResponse(StatusCode::INTERNAL_SERVER_ERROR, None))?;

        Ok(JsonResponse(status, Some(writer.into_inner().freeze())))
    }

    pub fn encode<T>(val: &T) -> JsonResult
    where
        T: Serialize,
    {
        Self::encode_with(val, StatusCode::OK)
    }
}

impl JsonErrorResponse {
    pub fn error<Msg: std::fmt::Display>(status: StatusCode, message: Msg) -> JsonErrorResponse {
        let err = JsonError {
            error: message.to_string(),
        };

        let mut writer = BytesMut::new().writer();
        match serde_json::to_writer(&mut writer, &err) {
            Ok(_) => JsonErrorResponse(status, Some(writer.into_inner().freeze())),
            Err(_err) => JsonErrorResponse(StatusCode::INTERNAL_SERVER_ERROR, None),
        }
    }
}

const JSON_HEADER: [(HeaderName, &str); 1] = [(header::CONTENT_TYPE, "application/json")];

impl IntoResponse for JsonResponse {
    fn into_response(self) -> Response {
        let (code, bin_opt) = (self.0, self.1);
        let body = match bin_opt {
            Some(bin) => bin,
            None => Bytes::new(),
        };

        (code, JSON_HEADER, body).into_response()
    }
}

impl IntoResponse for JsonErrorResponse {
    fn into_response(self) -> Response {
        let (code, bin_opt) = (self.0, self.1);
        let body = match bin_opt {
            Some(bin) => bin,
            None => Bytes::new(),
        };

        (code, JSON_HEADER, body).into_response()
    }
}

use crate::calculation_errors::{DieselUsageCalculationError, UnitInjectorFailCalculationError};

impl From<DieselUsageCalculationError> for JsonErrorResponse {
    fn from(err: DieselUsageCalculationError) -> Self {
        use DieselUsageCalculationError::*;

        let status_code = match err {
            InvalidParams(..) => StatusCode::BAD_REQUEST,
            CalculationFailed => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        JsonErrorResponse::error(status_code, err)
    }
}

impl From<UnitInjectorFailCalculationError> for JsonErrorResponse {
    fn from(err: UnitInjectorFailCalculationError) -> Self {
        use UnitInjectorFailCalculationError::*;

        let status_code = match err {
            InvalidParams(..) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        JsonErrorResponse::error(status_code, err)
    }
}
