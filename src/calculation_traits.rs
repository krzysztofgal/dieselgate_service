use super::BigDecimal;
use super::calculation_errors::{DieselUsageCalculationError, UnitInjectorFailCalculationError};

pub trait DieselUsageCalculator {
    fn calc_consumption_for_distance(&self, fuel_usage: usize, distance: usize, year_or_production: usize) -> Result<BigDecimal, DieselUsageCalculationError>;
}

pub struct NullDieselUsageCalculator;
impl DieselUsageCalculator for NullDieselUsageCalculator {
    fn calc_consumption_for_distance(&self, _fuel_usage: usize, _distance: usize, _year_or_production: usize) -> Result<BigDecimal, DieselUsageCalculationError> {
        Err(DieselUsageCalculationError::Unimplemented)
    }
}

pub trait UnitInjectorFailProbabilityCalculator {
    fn calc_failure_probability(&self, vin: &str) -> Result<BigDecimal, UnitInjectorFailCalculationError>;
}

pub struct NullUnitInjectorFailProbabilityCalculator;
impl UnitInjectorFailProbabilityCalculator for NullUnitInjectorFailProbabilityCalculator {
    fn calc_failure_probability(&self, _vin: &str) -> Result<BigDecimal, UnitInjectorFailCalculationError> {
        Err(UnitInjectorFailCalculationError::Unimplemented)
    }
}
