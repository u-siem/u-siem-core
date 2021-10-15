use std::borrow::Cow;
use std::collections::BTreeMap;
use serde::Serialize;

pub static CONNECTED_AGENTS : &'static str = "connected_agents";
pub static PROCESSING_LOGS_INPUT : &'static str = "processing_logs_input";
pub static PROCESSING_LOGS_PARSER : &'static str = "processing_logs_parser";
pub static LOGS_PARSING_TIME : &'static str = "logs_parsing_time";
pub static PROCESSING_LOGS_ENCHANCER : &'static str = "processing_logs_enchancer";
pub static PROCESSING_LOGS_INDEXER : &'static str = "processing_logs_indexer";
pub static LOGS_INDEXING_TIME : &'static str = "logs_indexing_time";
pub static PROCESSED_BYTES_INPUT : &'static str = "processed_bytes_input";
pub static PROCESSED_BYTES_INDEXER : &'static str = "processed_bytes_indexer";

#[derive(Serialize, Debug)]
pub enum SiemMetric {
    Counter(u64),
    Gauge(f64),
    Histogram(HistogramMetric),
    Summary(SummaryMetric)
}
#[derive(Serialize, Debug)]
pub struct SiemMetricEvent {
    pub metric : SiemMetric,
    pub name : Cow<'static, str>,
    pub tags : BTreeMap<Cow<'static, str>, Cow<'static, str>>,
}
#[derive(Serialize, Debug)]
pub struct SummaryMetric {
    pub sum : f64,
    pub count : u64,
    pub avg : f64,
    pub quantile0_5 : f64,
    pub quantile0_9 : f64,
    pub quantile0_99 : f64,
}
#[derive(Serialize, Debug)]
pub struct HistogramMetric {
    pub sum : f64,
    pub count : u64,
    pub avg : f64,
    pub quantile0_5 : f64,
    pub quantile0_9 : f64,
    pub quantile0_99 : f64
}