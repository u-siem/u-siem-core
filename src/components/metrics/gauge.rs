use std::{sync::{atomic::AtomicI64, Arc}};

use serde::{Serialize, Serializer, ser::{SerializeSeq, SerializeStruct}};

use super::prometheus::Encoder;

#[derive(Debug, Clone)]
pub struct Gauge {
    pub metric : Arc<AtomicI64>,
    pub labels : Vec<(&'static str, &'static str)>
}
#[derive(Debug, Clone)]
pub struct GaugeVec {
    pub metrics : Vec<Gauge>
}

const GAUGE_MODIFIER : i64 = 100_000;

impl GaugeVec {
    /// Create a Gauge with fixed labels
    /// 
    /// # Example
    /// 
    /// ```
    /// use usiem::components::metrics::gauge::GaugeVec;
    /// let metric = GaugeVec::new(&[
    ///    &[("name","Paco"), ("v1","1")],
    ///    &[("name","Pepe"), ("v1","2")]
    /// ]);
    /// ```
    pub fn new(labels : &[&[(&'static str, &'static str)]]) -> Self {
        Self {
            metrics : labels.into_iter().map(|label_list| {
                Gauge::new_with_labels(label_list)
            }).collect()
        }
    }
    /// Obtains a Gauge inside this GaugeVec that matches the metrics
    /// 
    /// # Example
    /// 
    /// ```
    /// use usiem::components::metrics::gauge::{Gauge, GaugeVec};
    /// let metric = GaugeVec::new(&[
    ///    &[("name","Paco"), ("v1","1")]
    /// ]);
    /// let gauge : &Gauge = metric.with_labels(&[("name","Paco"), ("v1","1")]).unwrap();
    /// assert!(metric.with_labels(&[("name","Paco"), ("v1","2")]).is_none());
    /// ```
    pub fn with_labels(&self, labels: &[(&str, &str)]) -> Option<&Gauge> {
        'cnt: for gauge in &self.metrics {
            for ((name1, value1), (name2, value2)) in gauge.labels.iter().zip(labels.iter()) {
                if name1 != name2 || value1 != value2 {
                    continue 'cnt
                }
            }
            return Some(gauge)
        }
        None
    }
    /// Resets all Gauges
    pub fn reset(&self) {
        for gauge in &self.metrics {
            gauge.reset();
        }
    }
}

impl Gauge {
    /// Creates a Gauge with no labels
    pub fn new() -> Self {
        Self {
            metric : Arc::new(AtomicI64::new(0)),
            labels : Vec::new()
        }
    }
    /// Creates a Gauge with labels
    /// 
    /// # Example
    /// 
    /// ```
    /// use usiem::components::metrics::gauge::Gauge;
    /// Gauge::new_with_labels(&[("name","Paco"), ("v1","1")]);
    /// ```
    pub fn new_with_labels(labels : &[(&'static str, &'static str)]) -> Self {
        let mut map = Vec::new();
        for (name, value) in labels {
            map.push((*name, *value));
        }
        Self {
            metric : Arc::new(AtomicI64::new(0)),
            labels : map
        }
    }
    /// Increase Gauge by One
    pub fn inc(&self){
        self.inc_by(1.0);
    }
    pub fn dec(&self) {
        self.dec_by(1.0);
    }
    pub fn inc_by(&self, v : f64) {
        self.metric.fetch_add(normalize_f64(v), std::sync::atomic::Ordering::SeqCst);
    }
    pub fn set(&self, v : f64) {
        self.metric.store(normalize_f64(v), std::sync::atomic::Ordering::SeqCst);
    }
    pub fn dec_by(&self, v : f64) {
        self.metric.fetch_sub(normalize_f64(v), std::sync::atomic::Ordering::SeqCst);
    }
    pub fn get(&self) -> f64 {
        normalize_i64(self.metric.load(std::sync::atomic::Ordering::Relaxed))
    }
    pub fn reset(&self) {
        self.metric.store(0, std::sync::atomic::Ordering::SeqCst);
    }
    
}

fn normalize_f64(numb : f64) -> i64 {
    (numb * (GAUGE_MODIFIER as f64)) as i64
}
fn normalize_i64(numb : i64) -> f64 {
    (numb as f64 / GAUGE_MODIFIER as f64) as f64
}

impl Encoder for GaugeVec {
    fn encode<W: std::fmt::Write>(&self, f: &mut W, name : &str, description : &str, help : bool) -> Result<(), std::fmt::Error> {
        if help {
            f.write_str("# HELP ")?;
            f.write_str(name)?;
            f.write_str(" ")?;
            f.write_str(description)?;
            f.write_str("\n")?;
        }
        f.write_str("# TYPE ")?;
        f.write_str(name)?;
        f.write_str(" gauge\n")?;
        for counter in &self.metrics {
            counter.encode(f, name, description, help)?;
        }
        Ok(())
                
    }
}

impl Encoder for Gauge {
    fn encode<W: std::fmt::Write>(&self, f: &mut W, name : &str, _description : &str, _help : bool) -> Result<(), std::fmt::Error> {
        f.write_str(name)?;
        f.write_str("{")?;
        let mut i = 0;
        for (name, value) in &self.labels {
            i+=1;
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
        f.write_fmt(format_args!("{}",self.get()))?;
        f.write_str("\n")?;
        Ok(())
    }
}

impl Serialize for GaugeVec {
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
impl Serialize for Gauge {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Gauge", 2)?;
        state.serialize_field("labels", &self.labels)?;
        state.serialize_field("value", &self.get())?;
        state.end()
    }
}

#[test]
fn gauge_metric_should_work() {
    let metric = GaugeVec::new(&[
        &[("name","Paco"), ("v1","1")],
        &[("name","Pepe"), ("v1","2")]
    ]);
    let gauge = metric.with_labels(&[("name","Paco"), ("v1","1")]).unwrap();
    gauge.inc_by(222.0);
    assert_eq!(222.0,gauge.get());
    gauge.reset();
    assert_eq!(0.0, gauge.get());
    gauge.inc_by(222.0);
    metric.reset();
    assert_eq!(0.0, gauge.get());
    let gauge = metric.with_labels(&[("name","Pepe"), ("v1","2")]).unwrap();
    gauge.inc_by(222.0);
    assert_eq!(222.0,gauge.get());
    gauge.reset();
    assert_eq!(0.0, gauge.get());
    gauge.inc_by(222.0);
    metric.reset();
    assert_eq!(0.0, gauge.get());
    assert!(metric.with_labels(&[("name","Paco"), ("v1","2")]).is_none());
    gauge.set(123.0);
    assert_eq!(123.0,gauge.get());
}

#[test]
fn gauge_should_be_encoded_in_prometheus() {
    let counter = GaugeVec::new(&[
        &[("name","Nombre"), ("v1","1")],
        &[("name","Nombre"), ("v1","2")]
    ]);
    let mut st = String::with_capacity(1_000_000);
    counter.encode(&mut st, "simple_gauge", "Simple Gauge metric", true).unwrap();
    assert_eq!(r#"# HELP simple_gauge Simple Gauge metric
# TYPE simple_gauge gauge
simple_gauge{name="Nombre",v1="1"} 0
simple_gauge{name="Nombre",v1="2"} 0
"#, st);
}