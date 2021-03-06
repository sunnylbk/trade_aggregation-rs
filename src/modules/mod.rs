use crate::common::Trade;
use crate::modules::close::ModuleClose;
use crate::modules::open::ModuleOpen;
use crate::modules::high::ModuleHigh;
use crate::modules::low::ModuleLow;
use std::fmt::{Debug};
use crate::modules::weighted_price::ModuleWeightedPrice;
use crate::modules::directional_trade_ratio::ModuleDirectionalTradeRatio;
use crate::modules::directional_volume_ratio::ModuleDirectionalVolumeRatio;
use crate::modules::std_dev_prices::ModuleStdDevPrices;
use crate::modules::std_dev_sizes::ModuleStdDevSizes;
use crate::modules::volume::ModuleVolume;
use crate::modules::average_price::ModuleAveragePrice;
use crate::modules::last_spread::ModuleLastSpread;
use crate::modules::avg_spread::ModuleAvgSpread;
use crate::modules::directional_trade_entropy::ModuleDirectionalTradeEntropy;
use crate::modules::directional_volume_entropy::ModuleDirectionalVolumeEntropy;
use crate::modules::time_velocity::ModuleTimeVelocity;
use crate::modules::num_trades::ModuleNumTrades;

mod open;
mod high;
mod low;
mod close;
mod volume;
mod directional_trade_ratio;
mod directional_volume_ratio;
mod std_dev_sizes;
mod std_dev_prices;
mod weighted_price;
mod average_price;
mod last_spread;
mod avg_spread;
mod directional_trade_entropy;
mod directional_volume_entropy;
mod time_velocity;
mod num_trades;

#[derive(Debug, Default)]
pub struct ModularCandle {
    pub features: Vec<CandleFeature>,
}

impl ModularCandle {
    pub fn from_modules(modules: &Vec<Box<dyn FeatureModule>>) -> Self {
        let mut features: Vec<CandleFeature> = Vec::new();
        for m in modules {
            let f = CandleFeature{
                value: m.value(),
            };
            features.push(f);
        }
        Self {
            features,
        }
    }
}

#[derive(Debug)]
pub struct CandleFeature {
    value: f64,
}

impl CandleFeature {
    pub fn value(&self) -> f64 {
        self.value
    }
}

/// enumeration of all available features
pub enum FeatureModules {
    // TODO: how to integrate different types for FeatureModule
    // Timestamp,
    Open,
    High,
    Low,
    Close,
    Volume,
    AveragePrice,
    WeightedPrice,
    NumTrades,
    DirectionalTradeRatio,
    DirectionalVolumeRatio,
    StdDevPrices,
    StdDevSizes,
    LastSpread,
    AvgSpread,
    DirectionalTradeEntropy,
    DirectionalVolumeEntropy,
    TimeVelocity,
}

impl FeatureModules {
    pub fn get_module(&self) -> Box<dyn FeatureModule> {
        return match self {
            // FeatureModules::Timestamp => Box::new(ModuleTimestamp::default()),
            FeatureModules::Open => Box::new(ModuleOpen::default()),
            FeatureModules::High => Box::new(ModuleHigh::default()),
            FeatureModules::Low => Box::new(ModuleLow::default()),
            FeatureModules::Close => Box::new(ModuleClose::default()),
            FeatureModules::Volume => Box::new(ModuleVolume::default()),
            FeatureModules::AveragePrice => Box::new(ModuleAveragePrice::default()),
            FeatureModules::WeightedPrice => Box::new(ModuleWeightedPrice::default()),
            FeatureModules::NumTrades => Box::new(ModuleNumTrades::default()),
            FeatureModules::DirectionalTradeRatio => Box::new(ModuleDirectionalTradeRatio::default()),
            FeatureModules::DirectionalVolumeRatio => Box::new(ModuleDirectionalVolumeRatio::default()),
            FeatureModules::StdDevPrices => Box::new(ModuleStdDevPrices::new()),
            FeatureModules::StdDevSizes => Box::new(ModuleStdDevSizes::new()),
            FeatureModules::LastSpread => Box::new(ModuleLastSpread::default()),
            FeatureModules::AvgSpread => Box::new(ModuleAvgSpread::default()),
            FeatureModules::DirectionalTradeEntropy => Box::new(ModuleDirectionalTradeEntropy::default()),
            FeatureModules::DirectionalVolumeEntropy => Box::new(ModuleDirectionalVolumeEntropy::default()),
            FeatureModules::TimeVelocity => Box::new(ModuleTimeVelocity::default()),
        }
    }
}

pub trait FeatureModule: Debug {
    fn name(&self) -> &str;
    fn value(&self) -> f64;
    fn update(&mut self, trade: &Trade, init: bool);
}

#[cfg(test)]
mod tests {
    use crate::common::Trade;

    pub const TRADES: [Trade; 10] = [
        Trade{ timestamp: 0, price: 100.0, size: 10.0 },
        Trade{ timestamp: 1, price: 101.0, size: -10.0 },
        Trade{ timestamp: 2, price: 100.0, size: 20.0 },
        Trade{ timestamp: 3, price: 102.0, size: 10.0 },
        Trade{ timestamp: 4, price: 103.0, size: 10.0 },
        Trade{ timestamp: 5, price: 104.0, size: -20.0 },
        Trade{ timestamp: 6, price: 102.0, size: -10.0 },
        Trade{ timestamp: 7, price: 101.0, size: 10.0 },
        Trade{ timestamp: 8, price: 102.0, size: 30.0 },
        Trade{ timestamp: 9, price: 105.0, size: 10.0 },
    ];
}
