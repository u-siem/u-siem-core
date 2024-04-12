use crate::prelude::store::DatasetStore;
use crate::prelude::types::LogString;
use crate::prelude::{SiemDatasetType, SiemResult};
use crossbeam_channel::{Receiver, Sender};

use super::events::SiemLog;
use common::{SiemComponentCapabilities, SiemMessage};
use std::boxed::Box;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use storage::SiemComponentStateStorage;

pub mod command;
pub mod command_types;
pub mod common;
pub mod metrics;
pub mod messages;
pub mod query;
pub mod storage;
pub mod task;
pub mod use_case;

pub mod enricher;
pub mod parser;
pub mod collector;

pub trait SiemComponent: Send {
    fn name(&self) -> &'static str {
        "SiemComponent"
    }
    /// Get the channel to this component
    fn local_channel(&self) -> Sender<SiemMessage>;
    /// Sets the channel used to receive/send logs. It's the kernel who sets the channel
    fn set_log_channel(&mut self, sender: Sender<SiemLog>, receiver: Receiver<SiemLog>);

    /// Execute the logic of this component in an infinite loop. Must be stopped using Commands sent using the channel.
    fn run(&mut self) -> SiemResult<()>;

    /// Allow to store information about this component like the state or configurations.
    fn set_storage(&mut self, conn: Box<dyn SiemComponentStateStorage>);

    /// Capabilities and actions that can be performed by this component
    fn capabilities(&self) -> SiemComponentCapabilities;

    /// Allows the Kernel to duplicate this component
    fn duplicate(&self) -> Box<dyn SiemComponent>;

    /// Initialize the component with the datasets before executing run
    fn set_datasets(&mut self, datasets: DatasetStore);
}

pub trait SiemDatasetManager: Send {
    fn set_id(&mut self, id: u64);

    fn name(&self) -> &str {
        "SiemDatasetManager"
    }
    /// Get the channel to this component
    fn local_channel(&self) -> Sender<SiemMessage>;

    /// Execute the logic of this component in an infinite loop. Must be stopped using Commands sent using the channel.
    fn run(&mut self) -> SiemResult<()>;

    /// The kernel registers the datasets of the components
    fn register_dataset(&mut self, dataset: SiemDatasetType);
    /// The kernel registers the datasets of the components
    fn register_datasets(&mut self, datasets: Vec<SiemDatasetType>);

    /// Get the list of datasets to initialize components.
    /// This must be the live version of the datasets shared only between the DatasetManager and the Kernel
    fn get_datasets(&self) -> Arc<Mutex<DatasetStore>>;
}

pub trait SiemRuleEngine: SiemComponent {
    /// Sets the dictionary of languages to generate the different alerts of the rules
    fn set_languages(&mut self, languages: BTreeMap<LogString, BTreeMap<LogString, LogString>>);
}