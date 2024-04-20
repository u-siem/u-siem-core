use crossbeam_channel::{Receiver, Sender};

use crate::events::SiemLog;

use crate::prelude::runtime::actor::{Actor, ActorContext, ActorState};
use crate::prelude::store::DatasetStore;
use crate::prelude::SiemDataset;
use crate::runtime::address::{ActorAddr, Mailed};

use super::command::SiemResponse;
use super::common::SiemMessage;

pub type LogCorrelatorBuilder = fn(LogCorrelationContext) -> ActorAddr;

#[allow(unused_variables)]
pub trait LogCorrelatorHandler : Actor<Context = LogCorrelationContext> {    
    /// Called for every log emitted by the stream.
    fn parse_log(&mut self, log: SiemLog, ctx: &mut LogCorrelationContext);
}

pub struct LogCorrelationContext {
    sender: Sender<SiemLog>,
    receiver: Receiver<SiemLog>,
    runtime : Sender<SiemMessage>,
    datasets : DatasetStore
}

impl ActorContext for LogCorrelationContext {
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
    
    fn set_datasets(&mut self, datasets : DatasetStore) {
        self.datasets = datasets;
    }
    fn update_dataset(&mut self, dataset : SiemDataset) {
        self.datasets.insert(dataset);
    }

    fn reply_command(&mut self, response : Mailed<SiemResponse>) {
        let _ = self.runtime.send(response.into());
    }
}

impl Clone for LogCorrelationContext {
    fn clone(&self) -> Self {
        Self { sender: self.sender.clone(), receiver: self.receiver.clone(), runtime: self.runtime.clone(), datasets: self.datasets.clone()}
    }
}
impl Default for LogCorrelationContext {
    fn default() -> Self {
        let (sender,_) = crossbeam_channel::bounded(1);
        let (runtime,_) = crossbeam_channel::bounded(1);
        let (_, receiver) = crossbeam_channel::bounded(1);
        let datasets = DatasetStore::new();
        Self {
            sender,
            runtime,
            receiver,
            datasets
        }
    }
}
impl LogCorrelationContext {
    pub fn new(runtime : Sender<SiemMessage>,sender : Sender<SiemLog>, receiver: Receiver<SiemLog>, datasets : DatasetStore) -> Self {
        Self {
            runtime,
            sender,
            receiver,
            datasets
        }
    }
    pub fn ingest(&mut self, log : SiemLog) {
        let _ = self.sender.send(log);
    }
    pub fn receiver(&self) -> &Receiver<SiemLog> {
        &self.receiver
    }
}