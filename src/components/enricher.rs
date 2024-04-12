use crossbeam_channel::Sender;

use crate::events::SiemLog;

use crate::prelude::runtime::actor::{Actor, ActorContext, ActorState};
use crate::prelude::store::DatasetStore;
use crate::prelude::SiemDataset;

#[allow(unused_variables)]
pub trait LogProcessorHandler : Actor<Context = LogParsingContext> {    
    /// Called for every log emitted by the stream.
    fn enrich_log(&mut self, log: SiemLog, ctx: &mut LogParsingContext);
}


pub struct LogParsingContext {
    channel: Sender<SiemLog>,
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
}

impl LogParsingContext {
    pub fn new() -> Self {
        let (channel,_) = crossbeam_channel::bounded(128);
        Self {
            channel,
            datasets : DatasetStore::new()
        }
    }
    pub fn ingest(&mut self, log : SiemLog) {
        let _ = self.channel.send(log);
    }
}