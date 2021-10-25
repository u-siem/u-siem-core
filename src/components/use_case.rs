use super::task::SiemTask;
use super::alert::{SolidRule};
use super::common::UserRole;
use serde::ser::{Serializer, SerializeStruct};
use serde::Serialize;
use std::fmt;
use dyn_clone::{clone_trait_object, DynClone};

#[derive(Clone)]
pub struct SiemUseCase {
    /// Name of the Use Case
    pub name : &'static str,
    /// Description of the Use Case and what is intended
    pub description : &'static str,
    /// Abstraction of the logic involved
    pub case_logic : &'static str,
    /// What cannot detect this use case
    pub limitations : &'static str,
    /// Device requirements: Product, Service, Category => AND conditioned
    pub requirements : (Option<&'static str>,Option<&'static str>,Option<&'static str>),
    /// Rule for detecting this Use Case. Only the name
    pub rule : &'static str,
    /// Steps to perform if an incident ocurrs
    pub actions : Vec<SiemPlaybookStep>,
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
        let mut state = serializer.serialize_struct("SiemAutomatedStep", 2)?;
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
    Manual(&'static str,&'static str),
    /// Automated action if the analyst wants to: FilterIP, RemediateEmail...
    Automated(SiemAutomatedStep)
}

#[derive(Clone)]
pub struct SiemAutomatedStep {
    /// Minimum role to execute this Step
    pub min_role : UserRole,
    /// Action to be executed, The String param passed to the Task is the `aggr_key` of the alert generated by the rule
    pub action : &'static str,
    /// Name of the step
    pub name  : &'static str,
    /// Description of the step
    pub description : &'static str
}


impl Serialize for SiemAutomatedStep {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("SiemAutomatedStep", 2)?;
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