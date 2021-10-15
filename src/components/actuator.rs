use super::dataset::{SiemDatasetType, SiemDataset};

/// Actators work in conjuntion with SOLID rule Engine
/// Some SOLID Rules can trigger actions specifically for the device that they are designed for
/// Example: GuardDuty rules that match Kali or Parrot activity can triger an action to block the IP using WAFs
pub trait SiemActuator : Clone {
    /// Simpl name to register the action in the SOAR component
    fn name(&self) -> &str;
    /// Describes the actions that this component makes
    fn description(&self) -> &str;
    /// Updates the internal reference to the dataset
    fn update_dataset(&mut self, dataset : SiemDataset);
    /// List of datasets needed by this action
    fn needed_dataset(&self) -> Vec<SiemDatasetType>;
    /// Triggers the SOAR action
    fn execute(&self, value: serde_json::Value) -> Result<String, String>;
}

/// Action to be executed by the Actuator
pub struct ActuatorRequest {
    /// Name of the action to be executed. Nomenclature: DEVICE::SERVICE::ACTION
    /// 
    /// Example: AWS::WAF::BlockIp to block an IP using a blocklist in WAFs (IPSet)
    pub name : &'static str,
    /// List of parameters to be pased to the SOAR function
    pub params : serde_json::Value
}