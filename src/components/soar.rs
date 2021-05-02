use super::super::events::field::{SiemIp, SiemField};
use super::super::events::SiemLog;

/// SOAR Actions work in conjuntion with SOLID rule Engine
/// Some SOLID Rules can trigger SOAR actions specifically for the device that they are designed for
/// Example: GuardDuty rules that match Kali or Parrot activity can triger a SOAR action to block the IP using WAFs
pub trait SiemSoarAction : Clone {
    /// Simpl name to register the action in the SOAR component
    fn name(&self) -> &str;
    /// Describes the actions that this component makes
    fn description(&self) -> &str;
    /// Triggers the SOAR action
    fn execute(&self, value: serde_json::Value) -> Result<String, String>;
}

/// Action to be executed by the SOAR
pub struct SoarRequestAction {
    /// Name of the action to be executed. Nomenclature: DEVICE::SERVICE::ACTION
    /// 
    /// Example: AWS::WAF::BlockIp to block an IP using a blocklist in WAFs (IPSet)
    pub name : &'static str,
    /// List of parameters to be pased to the SOAR function
    pub params : serde_json::Value
}