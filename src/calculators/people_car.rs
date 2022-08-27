use super::{DieselUsageCalculator, DieselUsageCalculationError};
use super::{UnitInjectorFailProbabilityCalculator, UnitInjectorFailCalculationError};
use super::{DieselConsumption, WearRatio, UnitInjectorRandomCalc};

use bigdecimal::BigDecimal;

#[derive(Debug)]
pub struct PasWagonC6Calculator {
    wear_ratio: f64,
}

impl PasWagonC6Calculator {
    pub fn new(wear_ratio: f64) -> Self {
        Self { wear_ratio }
    }
}

impl DieselUsageCalculator for PasWagonC6Calculator {
    fn calc_consumption_for_distance(&self, fuel_usage: usize, distance: usize, year_or_production: usize) -> Result<BigDecimal, DieselUsageCalculationError> {
        let wear = WearRatio::new(year_or_production, self.wear_ratio)?;
        let calc = DieselConsumption::new(fuel_usage).with_wear(wear);
        calc.fuel_usage_at(distance)
    }
}

impl UnitInjectorFailProbabilityCalculator for PasWagonC6Calculator {
    fn calc_failure_probability(&self, vin: &str) -> Result<BigDecimal, UnitInjectorFailCalculationError> {
        UnitInjectorRandomCalc.calc_failure_probability(vin)
    }
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use chrono::Datelike;
    use super::PasWagonC6Calculator;

    #[test]
    fn calc_consumptions() {
        use super::DieselUsageCalculator;

        let car = PasWagonC6Calculator::new(2.0);
        let fuel_usage = 5usize;
        let current_year = chrono::Local::now().year() as usize;

        let consumption = car.calc_consumption_for_distance(fuel_usage, 100, current_year).unwrap();
        dbg!(&consumption);
        assert_eq!(BigDecimal::from(5), consumption);

        let consumption = car.calc_consumption_for_distance(fuel_usage, 100, current_year - 1).unwrap();
        dbg!(&consumption);
        assert_eq!(BigDecimal::from(10), consumption);
    }

    #[test]
    fn test_injection_failure() {
        use super::UnitInjectorFailProbabilityCalculator;

        let car = PasWagonC6Calculator::new(2.0);
        let fail_chance = car.calc_failure_probability("_unused");
        dbg!(&fail_chance);
        assert!(fail_chance.is_ok());
    }
}
