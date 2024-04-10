pub mod channel;
pub mod actor;
pub mod context;
pub mod address;
pub mod task;
pub mod mailbox;
pub mod service;

use std::cell::RefCell;

use crate::{components::{collector::{LogCollectorBuilder, LogCollectorContext}, parser::LogParsingContext}, datasets, prelude::store::DatasetStore};

use self::actor::Actor;

thread_local!(
    static CURRENT: RefCell<Option<Runtime>> = RefCell::new(None);
);

/// SIEM actor runtime
#[derive(Default)]
pub struct Runtime {
    pub collectors : Vec<LogCollectorBuilder>,
    pub datasets : DatasetStore
}

impl Runtime {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn datasets(&mut self, datasets : DatasetStore) {
        self.datasets = datasets;
    }
    pub fn add_collector(&mut self, builder : LogCollectorBuilder) {
        for coll in self.collectors.iter() {
            if *coll == builder {
                return
            }
        }
        self.collectors.push(builder);
    }
    /// Adds a collector to the GLOBAL instance
    pub fn register_collector(builder : LogCollectorBuilder) {
        CURRENT.with_borrow_mut(|b|{
            b.as_mut().map(|b| {
                b.add_collector(builder);
            });
        })
    }
    /// Initializes the GLOBAL instance
    pub fn init() {
        CURRENT.with_borrow_mut(|b|{
            if b.is_none() {
                *b = Some(Runtime::new());
            }
        })
    }

    /// Starts the GLOBAL instance
    pub fn start() {
        CURRENT.with_borrow_mut(|b|{
            if let Some(r) = b {
                r.run();
            }
        })
    }

    pub fn run(&mut self) {
        for collector in self.collectors.iter() {
            let clc = collector(self.datasets.clone());
            
        }
    }
}

