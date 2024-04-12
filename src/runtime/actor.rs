use crate::{components::{command::{SiemCommand, SiemResponse}, task::{SiemTask, SiemTaskResult}}, datasets::{SiemDataset, SiemDatasetType}, prelude::{store::DatasetStore, ComponentError}};

use super::{address::Mailed, capabilities::ActorCapabilities};

#[allow(unused_variables)]
pub trait Actor : Sized + Send + 'static {
    type Context : ActorContext;

    /// Called when a actors is going to start execution
    fn init(&mut self, ctx: &mut Self::Context) {}

    /// Called after a actor is in `Stopping` state.
    ///
    /// A actor can return from the stopping state to the running
    /// state by returning `Running::Continue`.
    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }
    /// Notify the actor that a dataset has changed. 
    /// 
    /// The actor should check for changes in its configuration parameters or any required datasets.
    fn updated_dataset(&mut self, dataset : SiemDatasetType) {}

    /// Called after a actor is stopped.
    fn stopped(&mut self, ctx: &mut Self::Context) {}
    /// Execute a command exported by the actor
    fn command(&mut self, command : Mailed<SiemCommand>, ctx: &mut Self::Context) {}
    /// Receive a response for a command
    fn response(&mut self, command : Mailed<SiemResponse>, ctx: &mut Self::Context) {}

    fn task(&mut self, command : Mailed<SiemTask>, ctx: &mut Self::Context) {}
    /// Receive a response for a task executed
    fn task_result(&mut self, command : Mailed<SiemTaskResult>, ctx: &mut Self::Context) {}

    fn step(&mut self, ctx : &mut Self::Context) -> Result<(), ComponentError> {
        Ok(())
    }
}


/// Actor execution context.
///
/// Each actor runs within a specific execution context. 
///
/// The execution context defines the type of execution, and the
/// actor communication channels (message handling).
#[allow(unused_variables)]
pub trait ActorContext: Sized + Send {
    /// Register with the runtime the capabilities of this actor: commands that can receive, tasks to process, metrics...
    fn register(&mut self, capabilities : ActorCapabilities) {}
    /// Sets the datasets for the actor
    fn set_datasets(&mut self, datasets : DatasetStore) {}
    /// Get a reference to datasets
    fn datasets(&self) -> &DatasetStore;
    /// Updates a dataset for the actor
    fn update_dataset(&mut self, dataset : SiemDataset) {}
    /// Reply to a command executed by this component
    fn reply_command(&mut self, response : Mailed<SiemResponse>) {}
    /// Reply to a task executed by this component
    fn reply_task(&mut self, response : Mailed<SiemTaskResult>) {}
    /// Immediately stop processing incoming messages
    fn stop(&mut self);
    /// Terminate actor execution unconditionally.
    fn terminate(&mut self);
    /// Retrieve the current Actor execution state.
    fn state(&self) -> ActorState;
}

/// Actor execution state
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum ActorState {
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