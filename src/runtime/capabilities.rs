use serde::Serialize;

use crate::{components::{command::CommandDefinition, command_types::TaskDefinition, common::DatasetDefinition, metrics::MetricDefinition}, prelude::LogString};

/// Inform the runtime of the 'capabilities' of the actor. What the actor can do or it requires.
#[derive(Serialize, Debug, Clone)]
pub struct ActorCapabilities {
    /// Actor/component name
    pub name: LogString,
    /// Actor/component description
    pub description: LogString,
    /// Required datasets
    pub datasets: Vec<DatasetDefinition>,
    /// Exported commands
    pub commands: Vec<CommandDefinition>,
    /// Exported tasks
    pub tasks: Vec<TaskDefinition>,
    /// Exported metrics
    pub metrics: Vec<MetricDefinition>,
}