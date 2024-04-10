use crossbeam_channel::Sender;

use crate::{components::{command::{SiemCommand, SiemResponse}, common::SiemMessage, task::{SiemTask, SiemTaskResult}}, prelude::{store::DatasetStore, ComponentError, SiemResult}};

use super::context::Context;

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

    /// Called after a actor is stopped.
    fn stopped(&mut self, ctx: &mut Self::Context) {}
    
    fn command(&mut self, command : SiemCommand, ctx: &mut Self::Context) {}

    fn response(&mut self, command : SiemResponse, ctx: &mut Self::Context) {}

    fn task(&mut self, command : SiemTask, ctx: &mut Self::Context) {}

    fn task_result(&mut self, command : SiemTaskResult, ctx: &mut Self::Context) {}

    fn work(&mut self) -> Result<(), ComponentError> {
        Ok(())
    }
}


/// Actor execution context.
///
/// Each actor runs within a specific execution context. 
///
/// The execution context defines the type of execution, and the
/// actor communication channels (message handling).
pub trait ActorContext: Sized {
    fn set_datasets(&mut self, datasets : DatasetStore) {}

    fn datasets(&self) -> &DatasetStore;
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