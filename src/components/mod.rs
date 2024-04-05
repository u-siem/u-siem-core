use crate::prelude::store::DatasetStore;
use crate::prelude::types::LogString;
use crate::prelude::{SiemDatasetType, SiemResult};
use crossbeam_channel::{Receiver, Sender};

use self::command::SiemCommandCall;

use super::events::SiemLog;
use common::{SiemComponentCapabilities, SiemMessage};
use std::boxed::Box;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use storage::SiemComponentStateStorage;

pub mod command;
pub mod command_types;
pub mod common;
pub mod kernel_message;
pub mod metrics;
pub mod messages;
pub mod mailbox;

pub mod query;
pub mod storage;
pub mod task;
pub mod use_case;
pub mod simplified;

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

#[allow(unused_variables)]
pub trait Component : Sized + Send {
    type Context : ComponentContext;

    /// Called when a components is going to start execution
    fn init(&mut self, ctx: &mut Self::Context) {}

    /// Called before initializing the component. 
    /// The component would need the datasets when initializing itself.
    fn datasets(&mut self, datasets : ()) {}

    /// Called after a component is in `Stopping` state.
    ///
    /// A component can return from the stopping state to the running
    /// state by returning `Running::Continue`.
    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }

    /// Called after a component is stopped.
    fn stopped(&mut self, ctx: &mut Self::Context) {}
}


/// Component execution context.
///
/// Each component runs within a specific execution context. 
///
/// The execution context defines the type of execution, and the
/// component communication channels (message handling).
pub trait ComponentContext: Sized {
    /// Immediately stop processing incoming messages
    fn stop(&mut self);

    /// Terminate component execution unconditionally.
    fn terminate(&mut self);

    /// Retrieve the current Component execution state.
    fn state(&self) -> ComponentState;
}

/// Component execution state
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ComponentState {
    Started,
    Running,
    Stopping,
    Stopped,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Running {
    Stop,
    Continue,
}