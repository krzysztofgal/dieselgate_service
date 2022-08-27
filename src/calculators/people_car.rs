use super::{DieselUsageCalculator, DieselUsageCalculationError};
use super::{UnitInjectorFailProbabilityCalculator, UnitInjectorFailCalculationError};
use super::{DieselConsumption, WearRatio};

use bigdecimal::BigDecimal;

const WEAR_RATIO: f64 = 1.05;

#[derive(Debug)]
pub struct PasWagonC6Calculator;

impl DieselUsageCalculator for PasWagonC6Calculator {
    fn calc_consumption_for_distance(&self, fuel_usage: usize, distance: usize, year_or_production: usize) -> Result<BigDecimal, DieselUsageCalculationError> {
        let wear = WearRatio::new(year_or_production, WEAR_RATIO)?;
        let calc = DieselConsumption::new(fuel_usage).with_wear(wear);
        calc.fuel_usage_at(distance)
    }
}

impl UnitInjectorFailProbabilityCalculator for PasWagonC6Calculator {
    fn calc_failure_probability(&self, vin: &str) -> Result<BigDecimal, UnitInjectorFailCalculationError> {
        todo!()
    }
}
