use crate::prelude::LogString;

use super::common::UserRole;
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone)]
pub struct SiemUseCase {
    /// Name of the Use Case
    pub name: LogString,
    /// Description of the Use Case and what is intended
    pub description: LogString,
    /// Abstraction of the logic involved
    pub case_logic: LogString,
    /// What cannot detect this use case
    pub limitations: LogString,
    /// Device requirements: Product, Service, Category => AND conditioned
    pub requirements: Requirements,
    /// Rule for detecting this Use Case. Only the name
    pub rule: LogString,
    /// Steps to perform if an incident ocurrs
    pub actions: Vec<SiemPlaybookStep>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Requirements {
    pub product : Option<LogString>,
    pub service : Option<LogString>,
    pub category : Option<LogString>
}

impl fmt::Debug for SiemUseCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}
impl Serialize for SiemUseCase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SiemAutomatedStep", 7)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.serialize_field("case_logic", &self.case_logic)?;
        state.serialize_field("limitations", &self.limitations)?;
        state.serialize_field("rule", &self.rule)?;
        state.serialize_field("actions", &self.actions)?;
        state.serialize_field("requirements", &self.requirements)?;
        state.end()
    }
}

#[derive(Debug, Serialize, Clone)]
pub enum SiemPlaybookStep {
    /// Manual action to be performed by an analyst: Name and description
    Manual(LogString, LogString),
    /// Automated action if the analyst wants to: FilterIP, RemediateEmail...
    Automated(SiemAutomatedStep),
}

#[derive(Clone)]
pub struct SiemAutomatedStep {
    /// Minimum role to execute this Step
    pub min_role: UserRole,
    /// Action to be executed, The String param passed to the Task is the `aggr_key` of the alert generated by the rule
    pub action: LogString,
    /// Name of the step
    pub name: LogString,
    /// Description of the step
    pub description: LogString,
}

impl Serialize for SiemAutomatedStep {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SiemAutomatedStep", 3)?;
        state.serialize_field("min_role", &self.min_role)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description)?;
        state.end()
    }
}

impl fmt::Debug for SiemAutomatedStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish()
    }
}
