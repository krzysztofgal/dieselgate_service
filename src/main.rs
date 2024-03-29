use axum::{extract::Query, response::IntoResponse, routing::get, Extension, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

// arbitrary precision type
use bigdecimal::BigDecimal;

mod calculation_errors;
mod calculation_traits;
mod calculators;
mod json_response;

const SERVER_LISTEN_ADDR: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    use calculators::people_car::PasWagonC6Calculator;

    tracing_subscriber::fmt::init();

    let calculator = Arc::new(PasWagonC6Calculator::new(1.015));

    let app_state_usage_calc = Arc::new(UsageCalcState {
        diesel_usage_calculator: calculator.clone(),
    });

    let app_state_fail_calc = Arc::new(FailCalcState {
        injector_fail_calculator: calculator,
    });

    let app = Router::new()
        .route(
            "/calculateDisselUsageForDistance",
            get(handler_get_diesel_usage_for_distance).layer(Extension(app_state_usage_calc)),
        )
        .route(
            "/probabilityOfUnitInjectorFail",
            get(handler_get_injector_fail_probability).layer(Extension(app_state_fail_calc)),
        );

    info!("Starting Server at: {SERVER_LISTEN_ADDR}");

    axum::Server::bind(&SERVER_LISTEN_ADDR.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub struct UsageCalcState {
    diesel_usage_calculator: Arc<dyn calculation_traits::DieselUsageCalculator + Send + Sync>,
}

pub struct FailCalcState {
    injector_fail_calculator:
        Arc<dyn calculation_traits::UnitInjectorFailProbabilityCalculator + Send + Sync>,
}

#[derive(Debug, Deserialize)]
struct GetDieselUsageForDistanceQuery {
    distance: usize,
    #[serde(rename = "yearOfProduction")]
    production_year: usize,
    #[serde(rename = "fuelUsagePer100KM")]
    avg_fuel_usage: usize,
}

#[derive(Debug, Serialize)]
struct GetDieselUsageForDistanceResponse {
    #[serde(rename = "fuelUsage")]
    fuel_usage: BigDecimal,
}

async fn handler_get_diesel_usage_for_distance(
    Extension(app): Extension<Arc<UsageCalcState>>,
    Query(query): Query<GetDieselUsageForDistanceQuery>,
) -> impl IntoResponse {
    use json_response::JsonResponse;

    let GetDieselUsageForDistanceQuery {
        distance,
        production_year,
        avg_fuel_usage,
    } = query;

    let fuel_usage = app.diesel_usage_calculator.calc_consumption_for_distance(
        avg_fuel_usage,
        distance,
        production_year,
    )?;

    let response = GetDieselUsageForDistanceResponse { fuel_usage };

    JsonResponse::encode(&response)
}

#[derive(Debug, Deserialize)]
struct GetUnitInjectorFailQuery {
    #[serde(rename = "VIN")]
    vehicle_vin: String,
}

#[derive(Debug, Serialize)]
struct GetUnitInjectorFailResponse {
    #[serde(rename = "failProbability")]
    fail_probability: BigDecimal,
}

async fn handler_get_injector_fail_probability(
    Extension(app): Extension<Arc<FailCalcState>>,
    Query(query): Query<GetUnitInjectorFailQuery>,
) -> impl IntoResponse {
    use json_response::JsonResponse;

    let GetUnitInjectorFailQuery { vehicle_vin } = query;

    let fail_probability = app
        .injector_fail_calculator
        .calc_failure_probability(&vehicle_vin)?;

    let response = GetUnitInjectorFailResponse { fail_probability };

    JsonResponse::encode(&response)
}
