use crossbeam_channel::Sender;

use crate::events::SiemLog;

use crate::prelude::runtime::actor::{Actor, ActorContext, ActorState};
use crate::prelude::store::DatasetStore;

use super::common::SiemMessage;

#[allow(unused_variables)]
pub trait LogCollectorHandler : Actor<Context = LogCollectorContext> {

}

pub struct LogCollectorAddr {
    pub sender : Sender<SiemMessage>
}

pub type LogCollectorBuilder = fn(DatasetStore) -> LogCollectorAddr;

pub struct LogCollectorContext {
    channel: Sender<SiemLog>,
    datasets : DatasetStore
}

impl ActorContext for LogCollectorContext {
    fn stop(&mut self) {
    }

    fn terminate(&mut self) {
    }

    fn state(&self) -> ActorState {
        ActorState::Running
    }
    fn datasets(&self) -> &DatasetStore {
        &self.datasets
    }
}

impl LogCollectorContext {
    pub fn new() -> Self {
        let (channel,_) = crossbeam_channel::bounded(128);
        let datasets = DatasetStore::new();
        Self {
            channel,
            datasets
        }
    }
    pub fn ingest_log(&mut self, log : SiemLog) {
        self.channel.send(log);
    }
}