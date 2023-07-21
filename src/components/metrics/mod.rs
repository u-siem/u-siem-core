use crate::prelude::types::LogString;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
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

pub const BASIC_LE_CALCULATOR: [f64; 10] =
    [0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 2.0, 5.0, 10.0];

/// Metrics to be registered in the kernel.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SiemMetric {
    Counter(Arc<AtomicI64>),
    /// Atomic reference and a multiplier
    Gauge(Arc<AtomicI64>, f32),
    /// Timer in milliseconds
    Timer(Arc<AtomicU64>),
}

#[derive(Serialize, Debug, Clone)]
pub struct SiemMetricDefinition {
    pub metric: SiemMetric,
    pub name: LogString,
    pub description: LogString,
    pub labels: BTreeMap<LogString, LogString>,
}

// Work in progress...

#[derive(Debug, Clone)]
struct SummaryMetric {
    pub sum: Arc<AtomicI64>,
    pub count: Arc<AtomicU64>,
    pub multiplier: f64,
    pub quantiles: Vec<Quantile>,
}
#[derive(Debug, Clone)]
pub struct HistogramMetric {
    pub sum: Arc<AtomicI64>,
    pub count: Arc<AtomicU64>,
    pub multiplier: f64,
    pub labels: Vec<&'static str>,
    pub counters: HistogramBucket,
}

#[derive(Debug, Clone, Serialize)]
pub enum HistogramBucket {
    Static(StaticBucket),
}

impl HistogramBucket {
    pub fn get_metric_with_label_values<'a>(
        &'a self,
        labels: &'a [&str],
    ) -> Option<&'a Vec<Observation>> {
        match self {
            HistogramBucket::Static(bucket) => bucket.get_metric_with_label_values(labels),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Observation {
    pub multiplier: f64,
    pub le: f64,
    pub sum: Arc<AtomicI64>,
    pub count: Arc<AtomicU64>,
}

impl Observation {
    pub fn new(le: f64, multiplier: f64) -> Self {
        Self {
            multiplier,
            le,
            sum: Arc::new(AtomicI64::new(0)),
            count: Arc::new(AtomicU64::new(0)),
        }
    }
    pub fn observe(&self, value: f64) {
        if value <= self.le {
            self.count
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            self.sum.fetch_add(
                (value * self.multiplier) as i64,
                std::sync::atomic::Ordering::Relaxed,
            );
        }
    }
    pub fn get_sample_count(&self) -> u64 {
        self.count.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn get_sample_sum(&self) -> f64 {
        (self.sum.load(std::sync::atomic::Ordering::Relaxed) as f64) / self.multiplier
    }
}

#[derive(Debug, Clone)]
pub struct StaticBucket {
    pub observations: Arc<BTreeMap<Vec<&'static str>, Vec<Observation>>>,
}

impl StaticBucket {
    pub fn new(observations: BTreeMap<Vec<&'static str>, Vec<Observation>>) -> Self {
        Self {
            observations: Arc::new(observations),
        }
    }
    pub fn observe(&self, value: f64, labels: &Vec<&'static str>) {
        self.observations.get(labels).and_then(|v| {
            for obs in v {
                obs.observe(value);
            }
            Some(v)
        });
    }
    pub fn get_metric_with_label_values<'a>(
        &'a self,
        labels: &'a [&str],
    ) -> Option<&'a Vec<Observation>> {
        let metrics = self.observations.get(labels)?;
        Some(metrics)
    }
}

#[derive(Debug, Clone)]
pub struct Quantile {
    pub tag: &'static str,
    pub quantile: f64,
    pub value: Arc<AtomicU64>,
}

impl HistogramMetric {
    /// Creates a new histogram with static labels. The metric won't accept a new label once created.
    pub fn new_static(multiplier: f64, labels: &Vec<(&'static str, Vec<&'static str>)>) -> Self {
        Self::static_with_le_calculator(multiplier, labels, |i: usize| {
            BASIC_LE_CALCULATOR.get(i).and_then(|v| Some(*v))
        })
    }
    /// Creates a new histogram with static labels and custom values for the LE (lesser than or equals) parameter. The metric won't accept a new label once created.
    pub fn static_with_le_calculator(
        multiplier: f64,
        labels: &Vec<(&'static str, Vec<&'static str>)>,
        le: fn(usize) -> Option<f64>,
    ) -> Self {
        let mut observations = BTreeMap::new();

        let mut combinations = Vec::with_capacity(64);
        let mut counters = Vec::with_capacity(16);
        let mut posible_combinations = 1;
        let mut labels_v = Vec::with_capacity(32);
        labels.iter().for_each(|(tag, values)| {
            let tag_values: Vec<&'static str> = values.iter().map(|v| *v).collect();
            labels_v.push(*tag);
            posible_combinations *= tag_values.len();
            combinations.push(tag_values);
            counters.push(0);
        });

        let mut posible_tag_names = Vec::with_capacity(combinations.len());
        let counters_len = counters.len();

        for _i in 0..posible_combinations {
            let mut obs = Vec::with_capacity(8);
            for counter_i in 0..counters.len() {
                let counter = counters[counter_i];
                let combination_i = &combinations[counter_i];
                let ln = combination_i.len();
                let tag = combination_i[counter % ln];
                obs.push(tag);
                if counter_i >= counters_len - 1 {
                    counters[counter_i] += 1;
                }
                if counter_i != 0 {
                    for counter_ii in (0..counter_i).rev() {
                        if counters[counter_ii + 1] >= ln {
                            counters[counter_ii] += 1;
                            counters[counter_ii + 1] = 0;
                        }
                    }
                }
            }
            posible_tag_names.push(obs);
        }

        for tag_names in posible_tag_names {
            let mut counter = 0;
            let mut obs_vec = Vec::with_capacity(32);
            loop {
                match le(counter) {
                    Some(v) => {
                        obs_vec.push(Observation::new(v, multiplier));
                    }
                    None => break,
                }
                counter += 1;
            }
            observations.insert(tag_names, obs_vec);
        }
        Self {
            multiplier,
            labels: labels_v,
            sum: Arc::new(AtomicI64::new(0)),
            count: Arc::new(AtomicU64::new(0)),
            counters: HistogramBucket::Static(StaticBucket::new(observations)),
        }
    }

    pub fn observe(&self, value: f64, labels: &Vec<&'static str>) {
        self.count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.sum.fetch_add(
            (value * self.multiplier) as i64,
            std::sync::atomic::Ordering::Relaxed,
        );
        match &self.counters {
            HistogramBucket::Static(bucket) => {
                bucket.observe(value, labels);
            }
        }
    }
    pub fn get_metric_with_label_values<'a>(
        &'a self,
        labels: &'a [&str],
    ) -> Option<&'a Vec<Observation>> {
        self.counters.get_metric_with_label_values(labels)
    }
    pub fn get_all_labels(&self) -> &Vec<&'static str> {
        &self.labels
    }
    pub fn get_sample_count(&self) -> u64 {
        self.count.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn get_sample_sum(&self) -> f64 {
        (self.sum.load(std::sync::atomic::Ordering::Relaxed) as f64) / self.multiplier
    }
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
impl Serialize for Observation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Bucket", 4)?;
        state.serialize_field(
            "count",
            &self.count.load(std::sync::atomic::Ordering::Relaxed),
        )?;
        state.serialize_field("sum", &self.sum.load(std::sync::atomic::Ordering::Relaxed))?;
        state.serialize_field("le", &self.le)?;
        state.serialize_field("multiplier", &self.multiplier)?;
        state.end()
    }
}

impl Serialize for HistogramMetric {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("HistogramMetric", 4)?;
        state.serialize_field("sum", &(*self.sum))?;
        state.serialize_field("count", &(*self.count))?;
        state.serialize_field("multiplier", &self.multiplier)?;
        state.serialize_field("counters", &self.counters)?;
        state.end()
    }
}

impl Serialize for StaticBucket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("StaticBucket", 1)?;
        state.serialize_field("observations", &(*self.observations))?;
        state.end()
    }
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
            } /*
              SiemMetric::Histogram(hst) => {
                  state.serialize_field("type", "Histogram")?;
                  state.serialize_field("value", hst)?;
              }
              SiemMetric::Summary(smr) => {
                  state.serialize_field("type", "Summary")?;
                  state.serialize_field("value", smr)?;
              }*/
        }
        state.end()
    }
}

#[test]
fn should_create_complex_static_metric() {
    let labels = vec![
        ("name", vec!["a", "b", "c"]),
        ("v1", vec!["d", "e", "f"]),
        ("v2", vec!["g", "h", "i"]),
    ];
    let metric = HistogramMetric::new_static(1000.0, &labels);
    metric.observe(0.001, &vec!["a", "d", "g"]);
    assert_eq!(1, metric.get_sample_count());
    assert_eq!(0.001, metric.get_sample_sum());
    let observations = metric
        .get_metric_with_label_values(&["a", "d", "g"])
        .unwrap();
    for o in &observations[0..7] {
        assert_eq!(0.001, o.get_sample_sum());
        assert_eq!(1, o.get_sample_count());
    }
}
