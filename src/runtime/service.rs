use crossbeam_channel::Sender;

use crate::{components::{command::{SiemCommand, SiemResponse}, common::SiemMessage, task::{SiemTask, SiemTaskResult}}, events::SiemLog, prelude::store::DatasetStore};

use super::{actor::{Actor, Running}, address::ActorAddr};

pub type ServiceBuilder = fn(ServiceContext) -> ActorAddr;

#[allow(unused_variables)]
pub trait Service : Actor<Context = ServiceContext> {

}

/// Service execution context.
///
/// Each actor runs within a specific execution context. 
///
/// The execution context defines the type of execution, and the
/// actor communication channels (message handling).
pub struct ServiceContext {
    runtime : Sender<SiemMessage>,
    datasets : DatasetStore
}