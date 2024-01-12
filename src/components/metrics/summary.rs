use serde::ser::SerializeStruct;
use serde::Serialize;
use serde::Serializer;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

#[derive(Debug, Clone)]
struct SummaryMetric {
    pub sum: Arc<AtomicI64>,
    pub count: Arc<AtomicU64>,
    pub multiplier: f64,
    pub quantiles: Vec<Quantile>,
}

impl Serialize for SummaryMetric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SummaryMetric", 4)?;
        state.serialize_field("sum", &(*self.sum))?;
        state.serialize_field("count", &(*self.count))?;
        state.serialize_field("multiplier", &self.multiplier)?;
        state.serialize_field("quantiles", &self.quantiles)?;
        state.end()
    }
}

#[derive(Debug, Clone)]
pub struct Quantile {
    pub tag: &'static str,
    pub quantile: f64,
    pub value: Arc<AtomicU64>,
}

impl Serialize for Quantile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Quantile", 3)?;
        state.serialize_field("tag", &self.tag)?;
        state.serialize_field("value", &(*self.value))?;
        state.serialize_field("quantile", &self.quantile)?;
        state.end()
    }
}
