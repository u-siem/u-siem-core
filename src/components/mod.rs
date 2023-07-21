use crate::prelude::types::LogString;
use crate::prelude::SiemResult;
use crossbeam_channel::{Receiver, Sender};

use self::dataset::holder::DatasetHolder;
use self::kernel_message::KernelMessager;

use super::events::SiemLog;
use common::{SiemComponentCapabilities, SiemMessage};
use dataset::SiemDatasetType;
use std::boxed::Box;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use storage::SiemComponentStateStorage;

pub mod alert;
pub mod command;
pub mod command_types;
pub mod common;
pub mod dataset;
pub mod enrichment;
pub mod kernel_message;
pub mod metrics;
pub mod mitre;
pub mod parsing;
pub mod query;
pub mod rule;
pub mod storage;
pub mod task;
pub mod use_case;

pub trait SiemComponent: Send {
    fn id(&self) -> u64 {
        return 0;
    }
    fn set_id(&mut self, id: u64);
    fn name(&self) -> &'static str {
        return &"SiemComponent";
    }
    /// Get the channel to this component
    fn local_channel(&self) -> Sender<SiemMessage>;
    /// Sets the channel used to receive/send logs. It's the kernel who sets the channel
    fn set_log_channel(&mut self, sender: Sender<SiemLog>, receiver: Receiver<SiemLog>);

    /// Sets the channel to communicate with the kernel.
    fn set_kernel_sender(&mut self, sender: KernelMessager);

    /// Execute the logic of this component in an infinite loop. Must be stopped using Commands sent using the channel.
    fn run(&mut self) -> SiemResult<()>;

    /// Allow to store information about this component like the state or configurations.
    fn set_storage(&mut self, conn: Box<dyn SiemComponentStateStorage>);

    /// Capabilities and actions that can be performed by this component
    fn capabilities(&self) -> SiemComponentCapabilities;

    /// Allows the Kernel to duplicate this component
    fn duplicate(&self) -> Box<dyn SiemComponent>;

    /// Initialize the component with the datasets before executing run
    fn set_datasets(&mut self, datasets: DatasetHolder);
}

pub trait SiemDatasetManager: Send {
    fn id(&self) -> u64 {
        return 0;
    }
    fn set_id(&mut self, id: u64);

    fn name(&self) -> &str {
        return &"SiemDatasetManager";
    }
    /// Get the channel to this component
    fn local_channel(&self) -> Sender<SiemMessage>;

    /// Sets the channel to communicate with the kernel.
    fn set_kernel_sender(&mut self, sender: KernelMessager);

    /// Execute the logic of this component in an infinite loop. Must be stopped using Commands sent using the channel.
    fn run(&mut self) -> SiemResult<()>;

    /// The kernel registers the datasets of the components
    fn register_dataset(&mut self, dataset: SiemDatasetType);
    /// The kernel registers the datasets of the components
    fn register_datasets(&mut self, datasets: Vec<SiemDatasetType>);

    /// Get the list of datasets to initialize components.
    /// This must be the live version of the datasets shared only between the DatasetManager and the Kernel
    fn get_datasets(&self) -> Arc<Mutex<DatasetHolder>>;
}

pub trait SiemRuleEngine: SiemComponent {
    /// Sets the dictionary of languages to generate the different alerts of the rules
    fn set_languages(&mut self, languages: BTreeMap<LogString, BTreeMap<LogString, LogString>>);
}
