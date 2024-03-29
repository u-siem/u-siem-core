use std::sync::OnceLock;

use crate::prelude::types::LogString;
use crate::prelude::{SiemError, SiemResult};
use regex::{Regex, RegexBuilder};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use self::counter::CounterVec;
use self::gauge::GaugeVec;
use self::histogram::HistogramVec;
pub mod counter;
pub mod gauge;
pub mod histogram;
pub mod prometheus;
pub mod summary;

pub static CONNECTED_AGENTS: &str = "connected_agents";
pub static PROCESSING_LOGS_INPUT: &str = "processing_logs_input";
pub static PROCESSING_LOGS_PARSER: &str = "processing_logs_parser";
pub static LOGS_PARSING_TIME: &str = "logs_parsing_time";
pub static PROCESSING_LOGS_ENCHANCER: &str = "processing_logs_enchancer";
pub static PROCESSING_LOGS_INDEXER: &str = "processing_logs_indexer";
pub static LOGS_INDEXING_TIME: &str = "logs_indexing_time";
pub static PROCESSED_BYTES_INPUT: &str = "processed_bytes_input";
pub static PROCESSED_BYTES_INDEXER: &str = "processed_bytes_indexer";

static VALID_NAME_REGEX: OnceLock<Regex> = OnceLock::new();
static VALID_DESCRIPTION_REGEX: OnceLock<Regex> = OnceLock::new();

/// Metrics used by the components. Must have valid names and description following the prometheus format.
///
/// # Example
///
/// ```
/// use usiem::components::metrics::SiemMetricDefinition;
/// use usiem::components::metrics::counter::CounterVec;
/// SiemMetricDefinition::new("basic_event_counter",  "Events processed by the SIEM", CounterVec::new(&[
///    &[("parser","Firewall"), ("v","1")],
///    &[("parser","Linux"), ("v","1")]
/// ]).into()).unwrap();
/// ```
#[derive(Serialize, Debug, Clone)]
pub struct SiemMetricDefinition {
    metric: SiemMetric,
    name: LogString,
    description: LogString,
}

/// Metrics to be registered in the kernel.
/// All metrics labels are static, connot be created dinamically.
/// Supported metrics are: Counter, Gauge and Histogram
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SiemMetric {
    Counter(CounterVec),
    Gauge(GaugeVec),
    Histogram(HistogramVec),
}

impl SiemMetricDefinition {
    pub fn new<S: Into<LogString>>(
        name: S,
        description: S,
        metric: SiemMetric,
    ) -> SiemResult<Self> {
        let name = name.into();
        let description = description.into();
        if !valid_name(&name) {
            return Err(SiemError::Configuration(format!(
                "Invalid Metric name {}",
                name
            )));
        }
        if !valid_description(&description) {
            return Err(SiemError::Configuration(format!(
                "Invalid Metric description {}",
                description
            )));
        }
        Ok(Self {
            name,
            description,
            metric,
        })
    }
    /// Metric name. ALways in prometheus format
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Metric description. ALways in prometheus format
    pub fn description(&self) -> &str {
        &self.description
    }
    /// Metric: Counter, Gauge, Histogram
    pub fn metric(&self) -> &SiemMetric {
        &self.metric
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
                state.serialize_field("metric_type", "Counter")?;
                state.serialize_field("valuess", cnt)?;
            }
            SiemMetric::Gauge(gauge) => {
                state.serialize_field("metric_type", "Gauge")?;
                state.serialize_field("values", gauge)?;
            }
            SiemMetric::Histogram(hst) => {
                state.serialize_field("type", "Histogram")?;
                state.serialize_field("value", hst)?;
            } /*
              SiemMetric::Summary(smr) => {
                  state.serialize_field("type", "Summary")?;
                  state.serialize_field("value", smr)?;
              }*/
        }
        state.end()
    }
}

pub fn valid_name(txt: &str) -> bool {
    let regex = VALID_NAME_REGEX.get_or_init(|| {
        RegexBuilder::new("^[a-z][a-z0-9_]+[a-z]$")
            .case_insensitive(false)
            .build()
            .unwrap()
    });
    regex.is_match(txt)
}
pub fn valid_description(txt: &str) -> bool {
    let regex = VALID_DESCRIPTION_REGEX.get_or_init(|| {
        RegexBuilder::new(r"^[\w\s]+$")
            .case_insensitive(false)
            .build()
            .unwrap()
    });
    regex.is_match(txt)
}

/// Generate all posible combinations of static labels to initialize the metrics. Remember: dynamic labels are not supported yet.
pub fn label_combinations(
    labels: &[(&'static str, &[&'static str])],
) -> Vec<Vec<(&'static str, &'static str)>> {
    let mut combinations = Vec::with_capacity(64);
    let mut counters = Vec::with_capacity(16);
    let mut posible_combinations = 1;
    let mut labels_v = Vec::with_capacity(32);
    labels.iter().for_each(|(tag, values)| {
        let tag_values: Vec<(&'static str, &'static str)> =
            values.iter().map(|v| (*tag, *v)).collect();
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
            obs.push(combination_i[counter % ln]);
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

    posible_tag_names
}

impl From<GaugeVec> for SiemMetric {
    fn from(value: GaugeVec) -> Self {
        SiemMetric::Gauge(value)
    }
}
impl From<&GaugeVec> for SiemMetric {
    fn from(value: &GaugeVec) -> Self {
        SiemMetric::Gauge(value.clone())
    }
}
impl TryInto<GaugeVec> for SiemMetric {
    type Error = SiemError;

    fn try_into(self) -> Result<GaugeVec, Self::Error> {
        match self {
            SiemMetric::Counter(_) => Err(SiemError::Other(
                "Cannot transform Counter to Gauge Metric".into(),
            )),
            SiemMetric::Gauge(v) => Ok(v),
            SiemMetric::Histogram(_) => Err(SiemError::Other(
                "Cannot transform Histogram to Gauge Metric".into(),
            )),
        }
    }
}
impl TryInto<GaugeVec> for &SiemMetric {
    type Error = SiemError;

    fn try_into(self) -> Result<GaugeVec, Self::Error> {
        match self {
            SiemMetric::Counter(_) => Err(SiemError::Other(
                "Cannot transform Counter to Gauge Metric".into(),
            )),
            SiemMetric::Gauge(v) => Ok(v.clone()),
            SiemMetric::Histogram(_) => Err(SiemError::Other(
                "Cannot transform Histogram to Gauge Metric".into(),
            )),
        }
    }
}

impl From<CounterVec> for SiemMetric {
    fn from(value: CounterVec) -> Self {
        SiemMetric::Counter(value)
    }
}
impl From<&CounterVec> for SiemMetric {
    fn from(value: &CounterVec) -> Self {
        SiemMetric::Counter(value.clone())
    }
}
impl TryInto<CounterVec> for SiemMetric {
    type Error = SiemError;

    fn try_into(self) -> Result<CounterVec, Self::Error> {
        match self {
            SiemMetric::Counter(v) => Ok(v),
            SiemMetric::Gauge(_) => Err(SiemError::Other(
                "Cannot transform Gauge to Counter Metric".into(),
            )),
            SiemMetric::Histogram(_) => Err(SiemError::Other(
                "Cannot transform Histogram to Counter Metric".into(),
            )),
        }
    }
}
impl TryInto<CounterVec> for &SiemMetric {
    type Error = SiemError;

    fn try_into(self) -> Result<CounterVec, Self::Error> {
        match self {
            SiemMetric::Counter(v) => Ok(v.clone()),
            SiemMetric::Gauge(_) => Err(SiemError::Other(
                "Cannot transform Gauge to Counter Metric".into(),
            )),
            SiemMetric::Histogram(_) => Err(SiemError::Other(
                "Cannot transform Histogram to Counter Metric".into(),
            )),
        }
    }
}

impl From<HistogramVec> for SiemMetric {
    fn from(value: HistogramVec) -> Self {
        SiemMetric::Histogram(value)
    }
}
impl From<&HistogramVec> for SiemMetric {
    fn from(value: &HistogramVec) -> Self {
        SiemMetric::Histogram(value.clone())
    }
}
impl TryInto<HistogramVec> for SiemMetric {
    type Error = SiemError;

    fn try_into(self) -> Result<HistogramVec, Self::Error> {
        match self {
            SiemMetric::Counter(_) => Err(SiemError::Other(
                "Cannot transform Counter to Histogram Metric".into(),
            )),
            SiemMetric::Gauge(_) => Err(SiemError::Other(
                "Cannot transform Gauge to Histogram Metric".into(),
            )),
            SiemMetric::Histogram(v) => Ok(v),
        }
    }
}
impl TryInto<HistogramVec> for &SiemMetric {
    type Error = SiemError;

    fn try_into(self) -> Result<HistogramVec, Self::Error> {
        match self {
            SiemMetric::Counter(_) => Err(SiemError::Other(
                "Cannot transform Counter to Histogram Metric".into(),
            )),
            SiemMetric::Gauge(_) => Err(SiemError::Other(
                "Cannot transform Gauge to Histogram Metric".into(),
            )),
            SiemMetric::Histogram(v) => Ok(v.clone()),
        }
    }
}

#[test]
fn should_generate_all_label_combinations() {
    let name_values = vec!["a", "b", "c"];
    let v1_values = vec!["d", "e", "f"];
    let v2_values = vec!["g", "h", "i"];
    let v22_values = vec!["g", "h"];

    let labels = vec![
        ("name", &name_values[..]),
        ("v1", &v1_values[..]),
        ("v2", &v2_values[..]),
    ];
    let combinations = label_combinations(&labels[..]);
    assert_eq!(27, combinations.len());

    let labels = vec![("name", &name_values[..]), ("v1", &v1_values[..])];
    let combinations = label_combinations(&labels[..]);
    assert_eq!(9, combinations.len());

    let labels = vec![
        ("name", &name_values[..]),
        ("v1", &v1_values[..]),
        ("v2", &v22_values[..]),
    ];
    let combinations = label_combinations(&labels[..]);
    assert_eq!(18, combinations.len());
    assert_eq!(
        &vec![("name", "a"), ("v1", "d"), ("v2", "g")],
        combinations.get(0).unwrap()
    );
}

#[test]
fn should_validate_names() {
    assert!(valid_name("this_is_a_valid_name"));
    assert!(!valid_name("this_is_NOT_a_valid_name"));
    assert!(!valid_name("this is NOT a valid name"));
}

#[test]
fn should_validate_description() {
    assert!(valid_description("this_is_a_valid_description"));
    assert!(valid_description("This is a valid description"));
    assert!(valid_description("this is NOT\na valid description"));
}
