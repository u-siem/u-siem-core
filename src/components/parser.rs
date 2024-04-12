use crossbeam_channel::{Receiver, Sender};

use crate::events::SiemLog;

use crate::prelude::runtime::actor::{Actor, ActorContext, ActorState};
use crate::prelude::store::DatasetStore;
use crate::prelude::SiemDataset;
use crate::runtime::address::{ActorAddr, ActorAddrRecv, Mailed};

use super::command::SiemResponse;
use super::common::SiemMessage;

pub type LogParserBuilder = fn(LogParsingContext) -> ActorAddr;

#[allow(unused_variables)]
pub trait LogProcessorHandler : Actor<Context = LogParsingContext> {    
    /// Called for every log emitted by the stream.
    fn parse_log(&mut self, log: SiemLog, ctx: &mut LogParsingContext);
}

pub struct LogParsingContext {
    sender: Sender<SiemLog>,
    receiver: Receiver<SiemLog>,
    runtime : Sender<SiemMessage>,
    datasets : DatasetStore
}

impl ActorContext for LogParsingContext {
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

impl Clone for LogParsingContext {
    fn clone(&self) -> Self {
        Self { sender: self.sender.clone(), receiver: self.receiver.clone(), runtime: self.runtime.clone(), datasets: self.datasets.clone()}
    }
}
impl Default for LogParsingContext {
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
impl LogParsingContext {
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