pub mod channel;
pub mod actor;
pub mod context;
pub mod address;
pub mod task;
pub mod mailbox;
pub mod service;
pub mod capabilities;

use std::cell::RefCell;

use crate::{components::{collector::{LogCollectorBuilder, LogCollectorContext}, parser::{LogParserBuilder, LogParsingContext}}, datasets, prelude::store::DatasetStore};

use self::actor::Actor;

thread_local!(
    static CURRENT: RefCell<Option<Runtime>> = RefCell::new(None);
);

/// SIEM actor runtime
#[derive(Default)]
pub struct Runtime {
    pub collectors : Vec<LogCollectorBuilder>,
    pub parsers : Vec<LogParserBuilder>,
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
    pub fn add_parser(&mut self, builder : LogParserBuilder) {
        for coll in self.parsers.iter() {
            if *coll == builder {
                return
            }
        }
        self.parsers.push(builder);
    }

    /// Adds a collector to the GLOBAL instance
    pub fn register_collector(builder : LogCollectorBuilder) {
        CURRENT.with_borrow_mut(|b|{
            b.as_mut().map(|b| {
                b.add_collector(builder);
            });
        })
    }
    /// Adds a parser to the GLOBAL instance
    pub fn register_parser(builder : LogParserBuilder) {
        CURRENT.with_borrow_mut(|b|{
            b.as_mut().map(|b| {
                b.add_parser(builder);
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
        let (collector_sender, parser_receiver) = crossbeam_channel::bounded(128);
        let (parser_sender, enricher_receiver) = crossbeam_channel::bounded(128);
        let (runtime, receiver) = crossbeam_channel::bounded(1024);

        let ctx = LogCollectorContext::new(runtime.clone(), collector_sender, self.datasets.clone());
        let collectors : Vec<_> = self.collectors.iter().map(|c| c(ctx.clone())).collect();
        let ctx = LogParsingContext::new(runtime.clone(), parser_sender, parser_receiver, self.datasets.clone());
        let parsers : Vec<_> = self.parsers.iter().map(|c| c(ctx.clone())).collect();
        //TODO: interconnect datasets
        for _ in 0..10 {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

