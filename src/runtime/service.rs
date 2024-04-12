use crate::{components::{command::{SiemCommand, SiemResponse}, common::SiemMessage, task::{SiemTask, SiemTaskResult}}, prelude::store::DatasetStore};

use super::actor::Running;


/// The service dos not run like a normal component. It only reacts 
#[allow(unused_variables)]
pub trait Service : Sized + Send {
    type Context : ServiceContext;
    
    fn command(&mut self, command : SiemCommand, ctx: &mut Self::Context) {}

    fn response(&mut self, command : SiemResponse, ctx: &mut Self::Context) {}

    fn task(&mut self, command : SiemTask, ctx: &mut Self::Context) {}

    fn task_result(&mut self, command : SiemTaskResult, ctx: &mut Self::Context) {}

}

/// Service execution context.
///
/// Each actor runs within a specific execution context. 
///
/// The execution context defines the type of execution, and the
/// actor communication channels (message handling).
pub trait ServiceContext: Sized {
    fn datasets(&self) -> DatasetStore;
    fn send_command(&self, command : SiemCommand);
    fn send_command_with_cb(&self, command : SiemCommand, cb : Option<()>);
    fn send_task(&self, command : SiemTask);
    fn send_task_with_cb(&self, command : SiemTask, cb : Option<()>);
}