use bigdecimal::BigDecimal;

use crate::calculation_errors::{DieselUsageCalculationError, UnitInjectorFailCalculationError};
use crate::calculation_traits::{DieselUsageCalculator, UnitInjectorFailProbabilityCalculator};

pub mod people_car;

pub struct DieselConsumption {
    avg_usage: usize,
    wear: Option<WearRatio>,
}

impl DieselConsumption {
    pub fn new(avg_usage_per_100km: usize) -> Self {
        Self {
            avg_usage: avg_usage_per_100km,
            wear: None,
        }
    }

    pub fn with_wear(mut self, wear: WearRatio) -> Self {
        self.wear = Some(wear);
        self
    }

    pub fn fuel_usage_at(
        &self,
        distance: usize,
    ) -> Result<BigDecimal, DieselUsageCalculationError> {
        use bigdecimal::FromPrimitive;
        use DieselUsageCalculationError::*;

        if self.avg_usage < 1 {
            return Err(InvalidParams(
                "Fuel usage per 100km cannot be less than 1".into(),
            ));
        }

        if distance < 1 {
            return Err(InvalidParams("Distance cannot be less than 1".into()));
        }

        let mut avg_usage = self.avg_usage as f64;
        if let Some(ref wear) = self.wear {
            let usage_with_ratio = avg_usage + avg_usage * wear.get_ratio();

            // prevent to much scaling, car should not ever have more than 3 times of normal consumption.
            let max_usage = 3.0 * avg_usage;
            avg_usage = f64::min(usage_with_ratio, max_usage);
        }

        let consumption = (distance as f64 / 100.0) * avg_usage;
        let consumption = BigDecimal::from_f64(consumption).ok_or(CalculationFailed)?;

        Ok(consumption.with_prec(2))
    }
}

pub struct WearRatio {
    car_age: usize,
    wear_ratio: f64,
}

impl WearRatio {
    pub fn new(
        production_year: usize,
        wear_ratio: f64,
    ) -> Result<Self, DieselUsageCalculationError> {
        use chrono::{Datelike, Local};
        use DieselUsageCalculationError::{InvalidParams, Unimplemented};

        if wear_ratio < 1.0 {
            return Err(Unimplemented);
        }

        let current_year = Local::now().year();
        if production_year as i32 > current_year {
            return Err(InvalidParams("This car does not exist yet :)".into()));
        }

        let car_age = current_year - production_year as i32;

        Ok(Self {
            car_age: car_age as usize,
            wear_ratio,
        })
    }

    pub fn get_ratio(&self) -> f64 {
        if self.car_age > 0 {
            self.car_age as f64 * (self.wear_ratio - 1.0)
        } else {
            1.0
        }
    }
}

pub struct UnitInjectorRandomCalc;

impl UnitInjectorFailProbabilityCalculator for UnitInjectorRandomCalc {
    fn calc_failure_probability(
        &self,
        vin: &str,
    ) -> Result<BigDecimal, UnitInjectorFailCalculationError> {
        use bigdecimal::FromPrimitive;
        use rand::Rng;
        use UnitInjectorFailCalculationError::InvalidParams;

        if vin.is_empty() {
            return Err(InvalidParams("Vin cannot be empty".into()));
        }

        let mut r = rand::thread_rng();
        // max 80%
        let random_val = r.gen_range(0.1..0.8);

        Ok(BigDecimal::from_f64(random_val)
            .unwrap_or_default()
            .with_prec(2))
    }
}
