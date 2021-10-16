use std::borrow::Cow;
use crossbeam_channel::{Sender, Receiver};
use super::events::SiemLog;
pub mod common;
pub mod dataset;
use common::{SiemMessage, SiemComponentStateStorage, SiemComponentCapabilities};
use std::boxed::Box;
pub mod mitre;
pub mod alert;
pub mod actuator;
pub mod metrics;
pub mod task;
use dataset::{SiemDataset};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub trait SiemComponent : Send {
    fn id(&self) -> u64 {
        return 0
    }
    fn set_id(&mut self, id: u64);
    fn name(&self) -> Cow<'static, str>{
        return Cow::Borrowed("SiemComponent")
    }
    /// Get the channel to this component
    fn local_channel(&self) -> Sender<SiemMessage>;
    /// Sets the channel of this component. It's the kernel who sets the channel
    fn set_log_channel(&mut self, sender : Sender<SiemLog>, receiver : Receiver<SiemLog>);

    /// Sets the channel to communicate with the kernel.
    fn set_kernel_sender(&mut self, sender : Sender<SiemMessage>);

    /// Execute the logic of this component in an infinite loop. Must be stopped using Commands sent using the channel.
    fn run(&mut self);

    /// Allow to store information about this component like the state or conigurations.
    fn set_storage(&mut self, conn : Box<dyn SiemComponentStateStorage>);

    /// Capabilities and actions that can be performed by this component
    fn capabilities(&self) -> SiemComponentCapabilities;

    /// Allows the Kernel to duplicate this component
    fn duplicate(&self) -> Box<dyn SiemComponent>;
    
    /// Initialize the component with the datasets before executing run
    fn set_datasets(&mut self, datasets : Vec<SiemDataset>);
}

pub trait SiemDatasetManager : Send {
    fn name(&self) -> Cow<'static, str>{
        return Cow::Borrowed("SiemDatasetManager")
    }
    /// Get the channel to this component
    fn local_channel(&self) -> Sender<SiemMessage>;

    /// Sets the channel to communicate with the kernel.
    fn set_kernel_sender(&mut self, sender : Sender<SiemMessage>);

    /// Execute the logic of this component in an infinite loop. Must be stopped using Commands sent using the channel.
    fn run(&mut self);

    /// Get the list of datasets to initialize components
    fn get_datasets(&self) -> Arc<Mutex<Vec<SiemDataset>>>;

    /// Sets the map of components that need dataset updates
    fn set_dataset_channels(&mut self, channels : Arc<Mutex<BTreeMap<String,Vec<Sender<SiemMessage>>>>>);

    /// Allows the Kernel to duplicate this component
    fn duplicate(&self) -> Box<dyn SiemComponent>;
}