use std::{sync::{atomic::AtomicI64, Arc}};

use serde::{Serialize, Serializer, ser::{SerializeSeq, SerializeStruct}};

use super::prometheus::Encoder;

#[derive(Debug, Clone)]
pub struct Counter {
    metric : Arc<AtomicI64>,
    labels : Vec<(&'static str, &'static str)>
}
#[derive(Debug, Clone)]
pub struct CounterVec {
    metrics : Vec<Counter>
}



impl CounterVec {
    /// Create a Counter with fixed labels
    /// 
    /// # Example
    /// 
    /// ```
    /// use usiem::components::metrics::counter::CounterVec;
    /// let metric = CounterVec::new(&[
    ///    &[("name","Paco"), ("v1","1")],
    ///    &[("name","Pepe"), ("v1","2")]
    /// ]);
    /// ```
    pub fn new(labels : &[&[(&'static str, &'static str)]]) -> Self {
        Self {
            metrics : labels.into_iter().map(|label_list| {
                Counter::new_with_labels(label_list)
            }).collect()
        }
    }
    /// Obtains a Counter inside this CounterVec that matches the metrics
    /// 
    /// # Example
    /// 
    /// ```
    /// use usiem::components::metrics::counter::{Counter, CounterVec};
    /// let metric = CounterVec::new(&[
    ///    &[("name","Paco"), ("v1","1")]
    /// ]);
    /// let counter : &Counter = metric.with_labels(&[("name","Paco"), ("v1","1")]).unwrap();
    /// assert!(metric.with_labels(&[("name","Paco"), ("v1","2")]).is_none());
    /// ```
    pub fn with_labels(&self, labels: &[(&str, &str)]) -> Option<&Counter> {
        'cnt: for counter in &self.metrics {
            for ((name1, value1), (name2, value2)) in counter.labels.iter().zip(labels.iter()) {
                if name1 != name2 || value1 != value2 {
                    continue 'cnt
                }
            }
            return Some(counter)
        }
        None
    }
    /// Resets all Counters
    pub fn reset(&self) {
        for counter in &self.metrics {
            counter.reset();
        }
    }
}

impl Counter {
    /// Creates a counter with no labels
    pub fn new() -> Self {
        Self {
            metric : Arc::new(AtomicI64::new(0)),
            labels : Vec::new()
        }
    }
    /// Creates a counter with labels
    /// 
    /// # Example
    /// 
    /// ```
    /// use usiem::components::metrics::counter::Counter;
    /// Counter::new_with_labels(&[("name","Paco"), ("v1","1")]);
    /// ```
    pub fn new_with_labels(labels : &[(&'static str, &'static str)]) -> Self {
        let mut map = Vec::new();
        for (name, value) in labels {
            if name.contains("\n") || value.contains("\n") {
                panic!("Labels must not contain new lines");
            }
            map.push((*name, *value));
        }
        Self {
            metric : Arc::new(AtomicI64::new(0)),
            labels : map
        }
    }
    /// Increase counter by One
    pub fn inc(&self){
        self.inc_by(1);
    }
    pub fn inc_by(&self, v : i64) {
        self.metric.fetch_add(v, std::sync::atomic::Ordering::Relaxed);
    }
    pub fn get(&self) -> i64 {
        self.metric.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn reset(&self) {
        self.metric.store(0, std::sync::atomic::Ordering::Relaxed);
    }
}

impl Encoder for CounterVec {
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
        f.write_str(" counter\n")?;
        for counter in &self.metrics {
            counter.encode(f, name, description, help)?;
        }
        Ok(())
                
    }
}

impl Encoder for Counter {
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

impl Serialize for CounterVec {
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
impl Serialize for Counter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Counter", 2)?;
        state.serialize_field("labels", &self.labels)?;
        state.serialize_field("value", &self.get())?;
        state.end()
    }
}

#[test]
fn counter_metric_should_work() {
    let metric = CounterVec::new(&[
        &[("name","Paco"), ("v1","1")],
        &[("name","Pepe"), ("v1","2")]
    ]);
    let counter = metric.with_labels(&[("name","Paco"), ("v1","1")]).unwrap();
    counter.inc_by(222);
    assert_eq!(222,counter.get());
    counter.reset();
    assert_eq!(0, counter.get());
    counter.inc_by(222);
    metric.reset();
    assert_eq!(0, counter.get());
    let counter = metric.with_labels(&[("name","Pepe"), ("v1","2")]).unwrap();
    counter.inc_by(222);
    assert_eq!(222,counter.get());
    counter.reset();
    assert_eq!(0, counter.get());
    counter.inc_by(222);
    metric.reset();
    assert_eq!(0, counter.get());
    assert!(metric.with_labels(&[("name","Paco"), ("v1","2")]).is_none());
}

#[test]
fn counter_should_be_encoded_in_prometheus() {
    use crate::components::metrics::counter::CounterVec;
    let counter = CounterVec::new(&[
        &[("name","Nombre"), ("v1","1")],
        &[("name","Nombre"), ("v1","2")]
    ]);
    let mut st = String::with_capacity(1_000_000);
    counter.encode(&mut st, "simple_counter", "Simple Counter metric", true).unwrap();
    assert_eq!(r#"# HELP simple_counter Simple Counter metric
# TYPE simple_counter counter
simple_counter{name="Nombre",v1="1"} 0
simple_counter{name="Nombre",v1="2"} 0
"#, st);
}