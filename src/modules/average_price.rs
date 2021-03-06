use crate::modules::FeatureModule;
use crate::common::Trade;

#[derive(Debug, Default)]
pub struct ModuleAveragePrice {
    count: usize,
    sum: f64,
}

impl FeatureModule for ModuleAveragePrice {
    fn name(&self) -> &str {
        "AveragePrice"
    }

    fn value(&self) -> f64 {
        self.sum / self.count as f64
    }

    fn update(&mut self, trade: &Trade, init: bool) {
        if init {
            self.count = 0;
            self.sum = 0.0;
        }
        self.count += 1;
        self.sum += trade.price;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn module_average_price() {
        let mut m = ModuleAveragePrice::default();
        for t in &crate::modules::tests::TRADES {
            m.update(t, false);
        }
        assert_eq!(m.value(), 102.0);
    }
}