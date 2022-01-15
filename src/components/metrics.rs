use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::sync::atomic::AtomicI64;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

pub static CONNECTED_AGENTS: &'static str = "connected_agents";
pub static PROCESSING_LOGS_INPUT: &'static str = "processing_logs_input";
pub static PROCESSING_LOGS_PARSER: &'static str = "processing_logs_parser";
pub static LOGS_PARSING_TIME: &'static str = "logs_parsing_time";
pub static PROCESSING_LOGS_ENCHANCER: &'static str = "processing_logs_enchancer";
pub static PROCESSING_LOGS_INDEXER: &'static str = "processing_logs_indexer";
pub static LOGS_INDEXING_TIME: &'static str = "logs_indexing_time";
pub static PROCESSED_BYTES_INPUT: &'static str = "processed_bytes_input";
pub static PROCESSED_BYTES_INDEXER: &'static str = "processed_bytes_indexer";

/// Metrics to be registered in the kernel.
#[derive(Debug, Clone)]
pub enum SiemMetric {
    Counter(Arc<AtomicI64>),
    /// Atomic reference and a multiplier
    Gauge(Arc<AtomicI64>, f32),
    /// Timer in milliseconds
    Timer(Arc<AtomicU64>),
    Histogram(HistogramMetric),
    Summary(SummaryMetric),
}

#[derive(Serialize, Debug, Clone)]
pub struct SiemMetricDefinition {
    pub metric: SiemMetric,
    pub name: Cow<'static, str>,
    pub description: Cow<'static, str>,
    pub tags: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
}
#[derive(Debug, Clone)]
pub struct SummaryMetric {
    pub sum: Arc<AtomicI64>,
    pub count: Arc<AtomicU64>,
    pub avg: Arc<AtomicI64>,
    pub multiplier: f32,
    pub quantiles: Vec<Quantile>,
}
#[derive(Debug, Clone)]
pub struct HistogramMetric {
    pub sum: Arc<AtomicI64>,
    pub count: Arc<AtomicU64>,
    pub avg: Arc<AtomicI64>,
    pub multiplier: f32,
    pub quantiles: Vec<Quantile>,
}

#[derive(Debug, Clone)]
pub struct Quantile {
    pub tag: &'static str,
    pub value: Arc<AtomicI64>,
}

impl Serialize for Quantile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Quantile", 2)?;
        state.serialize_field("tag", &self.tag)?;
        state.serialize_field("value", &(*self.value))?;
        state.end()
    }
}

impl Serialize for HistogramMetric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("HistogramMetric", 5)?;
        state.serialize_field("sum", &(*self.sum))?;
        state.serialize_field("count", &(*self.count))?;
        state.serialize_field("avg", &(*self.avg))?;
        state.serialize_field("multiplier", &self.multiplier)?;
        state.serialize_field("quantiles", &self.quantiles)?;
        state.end()
    }
}

impl Serialize for SummaryMetric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SummaryMetric", 5)?;
        state.serialize_field("sum", &(*self.sum))?;
        state.serialize_field("count", &(*self.count))?;
        state.serialize_field("avg", &(*self.avg))?;
        state.serialize_field("multiplier", &self.multiplier)?;
        state.serialize_field("quantiles", &self.quantiles)?;
        state.end()
    }
}

impl Serialize for SiemMetric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SiemMetricDefinition", 2)?;
        match self {
            SiemMetric::Counter(cnt) => {
                state.serialize_field("type", "Counter")?;
                state.serialize_field("value", &(**cnt))?;
            }
            SiemMetric::Gauge(cnt, mlt) => {
                state.serialize_field("type", "Gauge")?;
                let counter: f64 = (&(**cnt)).load(std::sync::atomic::Ordering::Relaxed) as f64;
                state.serialize_field("value", &(counter * (*mlt as f64)))?;
            }
            SiemMetric::Timer(cnt) => {
                state.serialize_field("type", "Timer")?;
                state.serialize_field("value", &(**cnt))?;
            }
            SiemMetric::Histogram(hst) => {
                state.serialize_field("type", "Histogram")?;
                state.serialize_field("value", hst)?;
            }
            SiemMetric::Summary(smr) => {
                state.serialize_field("type", "Summary")?;
                state.serialize_field("value", smr)?;
            }
        }
        state.end()
    }
}
