use crossbeam_channel::Sender;

use crate::events::SiemLog;

use crate::prelude::runtime::actor::{Actor, ActorContext, ActorState};
use crate::prelude::store::DatasetStore;
use crate::runtime::address::ActorAddr;

use super::common::SiemMessage;

#[allow(unused_variables)]
pub trait LogCollectorHandler : Actor<Context = LogCollectorContext> {

}

pub type LogCollectorBuilder = fn(LogCollectorContext) -> ActorAddr;

#[derive(Clone)]
pub struct LogCollectorContext {
    sender: Sender<SiemLog>,
    runtime : Sender<SiemMessage>,
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
    
    fn update_dataset(&mut self, dataset : crate::prelude::SiemDataset) {
        self.datasets.insert(dataset);
    }
    fn reply_command(&mut self, response : crate::runtime::address::Mailed<super::command::SiemResponse>) {
        let _ = self.runtime.send(response.into());
    }
}

impl Default for LogCollectorContext {
    fn default() -> Self {
        let (sender,_) = crossbeam_channel::bounded(1);
        let (runtime,_) = crossbeam_channel::bounded(1);
        let datasets = DatasetStore::new();
        Self {
            sender,
            runtime,
            datasets
        }
    }
}
impl LogCollectorContext {
    pub fn new(runtime : Sender<SiemMessage>, sender : Sender<SiemLog>, datasets : DatasetStore) -> Self {
        Self {
            sender,
            runtime,
            datasets
        }
    }
    pub fn ingest(&mut self, log : SiemLog) {
        let _ = self.sender.send(log);
    }
}