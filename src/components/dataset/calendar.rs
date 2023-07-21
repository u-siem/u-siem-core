use crate::prelude::types::LogString;
use crossbeam_channel::Sender;
use serde::Serialize;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Enum used to Add/Remove an IP in the GeoIP dataset or full replace it
#[derive(Serialize, Debug)]
pub enum UpdateCalendar {
    Add((i64, i64, LogString)),
    Remove((i64, i64)),
    Replace(CalendarDataset),
}
#[derive(Debug, Clone)]
pub struct CalendarSynDataset {
    dataset: Arc<CalendarDataset>,
    comm: Sender<UpdateCalendar>,
}

impl CalendarSynDataset {
    pub fn new(dataset: Arc<CalendarDataset>, comm: Sender<UpdateCalendar>) -> CalendarSynDataset {
        return CalendarSynDataset { dataset, comm };
    }
    /// Used to add IP with custom information like tags.
    pub fn insert(&mut self, start: i64, end: i64, data: LogString) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateCalendar::Add((start, end, data))) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn remove(&mut self, start: i64, end: i64) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateCalendar::Remove((start, end))) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn update(&mut self, data: CalendarDataset) {
        // Todo: improve with local cache to send retries
        match self.comm.try_send(UpdateCalendar::Replace(data)) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
    pub fn get(&self, time: i64) -> Option<Vec<&LogString>> {
        // Todo improve with cached content
        self.dataset.get(time)
    }
}

#[derive(Serialize, Debug)]
pub struct CalendarDataset {
    data: BTreeMap<i64, Vec<(i64, i64, LogString)>>,
}

impl CalendarDataset {
    pub fn new() -> CalendarDataset {
        return CalendarDataset {
            data: BTreeMap::new(),
        };
    }
    pub fn insert(&mut self, start: i64, end: i64, data: LogString) {
        let start_day = start / 86400000;
        let end_day = end / 86400000;
        match start_day - end_day {
            0 => {
                match self.data.get_mut(&start_day) {
                    Some(dt) => {
                        dt.push((start, end, data));
                    }
                    None => {
                        let vc = vec![(start, end, data)];
                        self.data.insert(start_day, vc);
                    }
                };
            }
            _ => {
                for day in start_day..(end_day + 1) {
                    let (start_time, end_time) = {
                        if day == end_day {
                            (day * 86400000, end)
                        } else if day == start_day {
                            (start, day * 86400000)
                        } else {
                            (day * 86400000, ((day + 1) * 86400000))
                        }
                    };
                    let to_insert = (start_time, end_time, data.clone());
                    match self.data.get_mut(&day) {
                        Some(dt) => {
                            dt.push(to_insert);
                        }
                        None => {
                            let vc = vec![to_insert];
                            self.data.insert(day, vc);
                        }
                    };
                }
            }
        }
    }
    /// Time in millisecs
    pub fn get(&self, time: i64) -> Option<Vec<&LogString>> {
        let day = time / 86400000;
        match self.data.get(&day) {
            Some(v) => {
                let mut to_ret = Vec::new();
                for (start, end, data) in v {
                    if *start <= time && *end >= time {
                        to_ret.push(data);
                    }
                }
                if to_ret.len() == 0 {
                    return None;
                }
                return Some(to_ret);
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use chrono::prelude::{TimeZone, Utc};

    #[test]
    fn test_dataset_creation_same_day() {
        let mut dataset = CalendarDataset::new();
        let start = match Utc.datetime_from_str("2020/10/05 12:11:10", "%Y/%m/%d %H:%M:%S") {
            Ok(timestamp) => timestamp.timestamp_millis(),
            Err(_err) => {
                panic!("Must not happen")
            }
        };
        let end = match Utc.datetime_from_str("2020/10/05 22:11:10", "%Y/%m/%d %H:%M:%S") {
            Ok(timestamp) => timestamp.timestamp_millis(),
            Err(_err) => {
                panic!("Must not happen")
            }
        };
        let time = match Utc.datetime_from_str("2020/10/05 18:11:10", "%Y/%m/%d %H:%M:%S") {
            Ok(timestamp) => timestamp.timestamp_millis(),
            Err(_err) => {
                panic!("Must not happen")
            }
        };
        let time2 = match Utc.datetime_from_str("2020/10/05 23:11:10", "%Y/%m/%d %H:%M:%S") {
            Ok(timestamp) => timestamp.timestamp_millis(),
            Err(_err) => {
                panic!("Must not happen")
            }
        };
        dataset.insert(start, end, LogString::Borrowed("LOLOLO"));
        assert_eq!(
            dataset.get(time),
            Some(vec!(&LogString::Borrowed("LOLOLO")))
        );
        assert_eq!(dataset.get(0), None);
        assert_eq!(dataset.get(time2), None);
    }

    #[test]
    fn test_dataset_creation_various_day() {
        let mut dataset = CalendarDataset::new();
        let start = match Utc.datetime_from_str("2020/10/01 12:11:10", "%Y/%m/%d %H:%M:%S") {
            Ok(timestamp) => timestamp.timestamp_millis(),
            Err(_err) => {
                panic!("Must not happen")
            }
        };
        let end = match Utc.datetime_from_str("2020/10/05 22:11:10", "%Y/%m/%d %H:%M:%S") {
            Ok(timestamp) => timestamp.timestamp_millis(),
            Err(_err) => {
                panic!("Must not happen")
            }
        };
        let time = match Utc.datetime_from_str("2020/10/02 1:11:10", "%Y/%m/%d %H:%M:%S") {
            Ok(timestamp) => timestamp.timestamp_millis(),
            Err(_err) => {
                panic!("Must not happen")
            }
        };
        let time2 = match Utc.datetime_from_str("2020/10/03 2:11:10", "%Y/%m/%d %H:%M:%S") {
            Ok(timestamp) => timestamp.timestamp_millis(),
            Err(_err) => {
                panic!("Must not happen")
            }
        };
        let time3 = match Utc.datetime_from_str("2020/10/05 23:11:10", "%Y/%m/%d %H:%M:%S") {
            Ok(timestamp) => timestamp.timestamp_millis(),
            Err(_err) => {
                panic!("Must not happen")
            }
        };
        let time4 = match Utc.datetime_from_str("2020/10/01 11:11:10", "%Y/%m/%d %H:%M:%S") {
            Ok(timestamp) => timestamp.timestamp_millis(),
            Err(_err) => {
                panic!("Must not happen")
            }
        };
        dataset.insert(start, end, LogString::Borrowed("LOLOLO"));
        assert_eq!(
            dataset.get(time),
            Some(vec!(&LogString::Borrowed("LOLOLO")))
        );
        assert_eq!(
            dataset.get(time2),
            Some(vec!(&LogString::Borrowed("LOLOLO")))
        );
        assert_eq!(dataset.get(0), None);
        assert_eq!(dataset.get(time3), None);
        assert_eq!(dataset.get(time4), None);
    }
}
