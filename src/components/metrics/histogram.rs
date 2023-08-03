use std::sync::{
    atomic::{AtomicI64, AtomicU64, Ordering},
    Arc,
};

use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

use super::prometheus::Encoder;

pub const BASIC_LE_CALCULATOR: [f64; 10] =
    [0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 2.0, 5.0, 10.0];

pub const HISTOGRAM_MODIFIER: i64 = 100_000;

#[derive(Debug, Clone)]
pub struct HistogramVec {
    pub metrics: Vec<Histogram>,
}

#[derive(Debug, Clone)]
pub struct Histogram {
    pub sum: Arc<AtomicI64>,
    pub count: Arc<AtomicU64>,
    pub counters: HistogramBucket,
    pub labels: Vec<(&'static str, &'static str)>,
}

#[derive(Debug, Clone)]
pub struct HistogramBucket {
    pub observations: Vec<Observation>,
}

#[derive(Debug, Clone)]
pub struct Observation {
    pub le: f64,
    pub value: Arc<AtomicU64>,
}
impl Observation {
    pub fn new(le: f64) -> Self {
        Self {
            value: Arc::new(AtomicU64::new(0)),
            le,
        }
    }
}

impl HistogramBucket {
    pub fn new(le: &[f64]) -> Self {
        Self {
            observations: le.iter().map(|v| Observation::new(*v)).collect(),
        }
    }
    pub fn observe(&self, value: f64) {
        for observation in &self.observations {
            if value > observation.le {
                continue;
            }
            observation.value.fetch_add(1, Ordering::SeqCst);
            
        }
    }
    pub fn get_count(&self, le: f64) -> Option<u64> {
        for observation in &self.observations {
            if le >= observation.le {
                return Some(observation.value.load(Ordering::Relaxed));
            }
        }
        None
    }
}

fn normalize_f64(numb: f64) -> i64 {
    (numb * (HISTOGRAM_MODIFIER as f64)) as i64
}
fn normalize_i64(numb: i64) -> f64 {
    (numb as f64 / HISTOGRAM_MODIFIER as f64) as f64
}

impl HistogramVec {
    /// Creates a new histogram with static labels. The metric won't accept a new label once created.
    pub fn new(labels: Vec<Vec<(&'static str, &'static str)>>) -> Self {
        Self::with_le(labels, &BASIC_LE_CALCULATOR)
    }
    /// Creates a new histogram with static labels and custom values for the LE (lesser than or equals) parameter. The metric won't accept a new label once created.
    pub fn with_le(labels: Vec<Vec<(&'static str, &'static str)>>, le: &[f64]) -> Self {
        let mut metrics = Vec::with_capacity(labels.len());
        for label in labels {
            metrics.push(Histogram::with_le(label, le));
        }
        Self { metrics }
    }
    /// Obtains a histogram with the specified labels
    pub fn with_labels(&self, labels: &[(&str, &str)]) -> Option<&Histogram> {
        'cnt: for hist in &self.metrics {
            if hist.labels.len() != labels.len() {
                continue;
            }
            for ((name1, value1), (name2, value2)) in hist.labels.iter().zip(labels.iter()) {
                if name1 != name2 || value1 != value2 {
                    continue 'cnt;
                }
            }
            return Some(hist);
        }
        None
    }
}

impl Histogram {
    /// Creates a new Histogram with static labels and the default le params: [0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 2.0, 5.0, 10.0]
    pub fn new(labels: Vec<(&'static str, &'static str)>) -> Self {
        Self::with_le(labels, &BASIC_LE_CALCULATOR[..])
    }
    /// Creates a new Histogram with static labels and a custom LE
    pub fn with_le(labels: Vec<(&'static str, &'static str)>, le: &[f64]) -> Self {
        Self {
            sum: Arc::new(AtomicI64::new(0)),
            count: Arc::new(AtomicU64::new(0)),
            counters: HistogramBucket::new(le),
            labels,
        }
    }
    /// Observe a value
    pub fn observe(&self, value: f64) {
        self.count.fetch_add(1, Ordering::SeqCst);
        self.sum.fetch_add(normalize_f64(value), Ordering::SeqCst);
        self.counters.observe(value);
    }
    /// Get the number of samples
    pub fn get_sample_count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }
    /// Get the acumulated sample value
    pub fn get_sample_sum(&self) -> f64 {
        normalize_i64(self.sum.load(Ordering::Relaxed))
    }
}

impl Encoder for HistogramVec {
    fn encode<W: std::fmt::Write>(
        &self,
        f: &mut W,
        name: &str,
        description: &str,
        help: bool,
    ) -> Result<(), std::fmt::Error> {
        if self.metrics.len() == 0 {
            return Ok(())
        }
        if help {
            f.write_str("# HELP ")?;
            f.write_str(name)?;
            f.write_str(" ")?;
            f.write_str(description)?;
            f.write_str("\n")?;
        }
        f.write_str("# TYPE ")?;
        f.write_str(name)?;
        f.write_str(" histogram \n")?;
        for counter in &self.metrics {
            counter.encode(f, name, description, help)?;
        }
        Ok(())
    }
}

impl Encoder for Histogram {
    fn encode<W: std::fmt::Write>(
        &self,
        f: &mut W,
        name: &str,
        _description: &str,
        _help: bool,
    ) -> Result<(), std::fmt::Error> {
        let count = self.get_sample_count();
        if count == 0 {
            return Ok(());
        }
        for observation in &self.counters.observations {
            f.write_str(name)?;
            f.write_str("_bucket{")?;
            for (name, value) in &self.labels {
                if value.is_empty() {
                    continue;
                }
                f.write_str(name)?;
                f.write_str("=\"")?;
                f.write_str(value)?;
                f.write_str("\",")?;
            }
            f.write_str("le=")?;
            f.write_fmt(format_args!("{}", observation.le))?;
            f.write_str("} ")?;
            f.write_fmt(format_args!(
                "{}",
                observation.value.load(Ordering::Relaxed)
            ))?;
            f.write_str("\n")?;
        }
        f.write_str(name)?;
        f.write_str("_sum{")?;
        let mut i = 0;
        for (name, value) in &self.labels {
            i += 1;
            if value.is_empty() {
                continue;
            }
            f.write_str(name)?;
            f.write_str("=\"")?;
            f.write_str(value)?;
            f.write_str("\"")?;
            if i != self.labels.len() {
                f.write_str(",")?;
            }
        }
        f.write_str("} ")?;
        f.write_fmt(format_args!("{}", self.get_sample_sum()))?;
        f.write_str("\n")?;
        f.write_str(name)?;
        f.write_str("_count{")?;
        let mut i = 0;
        for (name, value) in &self.labels {
            i += 1;
            if value.is_empty() {
                continue;
            }
            f.write_str(name)?;
            f.write_str("=\"")?;
            f.write_str(value)?;
            f.write_str("\"")?;
            if i != self.labels.len() {
                f.write_str(",")?;
            }
        }
        f.write_str("} ")?;
        f.write_fmt(format_args!("{}", count))?;
        f.write_str("\n")?;
        Ok(())
    }
}

impl Serialize for HistogramVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_seq(Some(self.metrics.len()))?;
        for metric in &self.metrics {
            state.serialize_element(metric)?;
        }
        state.end()
    }
}

impl Serialize for Histogram {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Histogram", 4)?;
        state.serialize_field("labels", &self.labels)?;
        state.serialize_field("count", &self.get_sample_count())?;
        state.serialize_field("sum", &self.get_sample_sum())?;
        state.serialize_field("buckets", &self.counters)?;
        state.end()
    }
}

impl Serialize for HistogramBucket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_seq(Some(self.observations.len()))?;
        for observation in &self.observations {
            state.serialize_element(observation)?;
        }
        state.end()
    }
}

impl Serialize for Observation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Observation", 2)?;
        state.serialize_field("v", &self.value.load(Ordering::Relaxed))?;
        state.serialize_field("le", &self.le)?;
        state.end()
    }
}
#[cfg(test)]
mod tst {
    use crate::components::metrics::{label_combinations, histogram::HistogramVec, prometheus::Encoder};
    #[test]
    fn should_create_complex_static_metric() {
        let name_values = vec!["a", "b", "c"];
        let v1_values = vec!["d", "e", "f"];
        let v2_values = vec!["g", "h", "i"];

        let labels = vec![
            ("name", &name_values[..]),
            ("v1", &v1_values[..]),
            ("v2", &v2_values[..]),
        ];
        let labels = label_combinations(&labels[..]);
        let metric = HistogramVec::new(labels);
        let hist = metric
            .with_labels(&[("name", "a"), ("v1", "d"), ("v2", "g")])
            .unwrap();
        hist.observe(0.001);
        assert_eq!(1, hist.get_sample_count());
        assert_eq!(0.001, hist.get_sample_sum());

        assert_eq!(Some(1), hist.counters.get_count(0.001));
    }

    #[test]
    fn gauge_should_be_encoded_in_prometheus() {
        let name_values = vec!["a", "b", "c"];
        let v1_values = vec!["d", "e", "f"];
        let v2_values = vec!["g", "h", "i"];

        let labels = vec![
            ("name", &name_values[..]),
            ("v1", &v1_values[..]),
            ("v2", &v2_values[..]),
        ];
        let labels = label_combinations(&labels[..]);
        let metric = HistogramVec::new(labels);
        let hist = metric
            .with_labels(&[("name", "a"), ("v1", "d"), ("v2", "g")])
            .unwrap();
        hist.observe(0.001);
        let mut st = String::with_capacity(1_000_000);
        metric
            .encode(&mut st, "simple_gauge", "Simple Gauge metric", true)
            .unwrap();
        assert_eq!(
            r#"# HELP simple_gauge Simple Gauge metric
# TYPE simple_gauge histogram 
simple_gauge_bucket{name="a",v1="d",v2="g",le=0.001} 1
simple_gauge_bucket{name="a",v1="d",v2="g",le=0.005} 1
simple_gauge_bucket{name="a",v1="d",v2="g",le=0.01} 1
simple_gauge_bucket{name="a",v1="d",v2="g",le=0.05} 1
simple_gauge_bucket{name="a",v1="d",v2="g",le=0.1} 1
simple_gauge_bucket{name="a",v1="d",v2="g",le=0.5} 1
simple_gauge_bucket{name="a",v1="d",v2="g",le=1} 1
simple_gauge_bucket{name="a",v1="d",v2="g",le=2} 1
simple_gauge_bucket{name="a",v1="d",v2="g",le=5} 1
simple_gauge_bucket{name="a",v1="d",v2="g",le=10} 1
simple_gauge_sum{name="a",v1="d",v2="g"} 0.001
simple_gauge_count{name="a",v1="d",v2="g"} 1
"#,
            st
        );
    }

    #[test]
    fn gauge_should_be_encoded_in_prometheus_without_label_name() {
        let name_values = vec!["", "b", "c"];
        let v1_values = vec!["d", "e", "f"];
        let v2_values = vec!["g", "h", "i"];

        let labels = vec![
            ("name", &name_values[..]),
            ("v1", &v1_values[..]),
            ("v2", &v2_values[..]),
        ];
        let labels = label_combinations(&labels[..]);
        let metric = HistogramVec::new(labels);
        let hist = metric
            .with_labels(&[("name", ""), ("v1", "d"), ("v2", "g")])
            .unwrap();
        hist.observe(0.001);
        let mut st = String::with_capacity(1_000_000);
        metric
            .encode(&mut st, "simple_gauge", "Simple Gauge metric", true)
            .unwrap();
        assert_eq!(
            r#"# HELP simple_gauge Simple Gauge metric
# TYPE simple_gauge histogram 
simple_gauge_bucket{v1="d",v2="g",le=0.001} 1
simple_gauge_bucket{v1="d",v2="g",le=0.005} 1
simple_gauge_bucket{v1="d",v2="g",le=0.01} 1
simple_gauge_bucket{v1="d",v2="g",le=0.05} 1
simple_gauge_bucket{v1="d",v2="g",le=0.1} 1
simple_gauge_bucket{v1="d",v2="g",le=0.5} 1
simple_gauge_bucket{v1="d",v2="g",le=1} 1
simple_gauge_bucket{v1="d",v2="g",le=2} 1
simple_gauge_bucket{v1="d",v2="g",le=5} 1
simple_gauge_bucket{v1="d",v2="g",le=10} 1
simple_gauge_sum{v1="d",v2="g"} 0.001
simple_gauge_count{v1="d",v2="g"} 1
"#,
            st
        );
    }
}
