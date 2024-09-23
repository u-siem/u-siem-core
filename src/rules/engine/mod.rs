use crate::events::SiemLog;

pub mod memory;

pub struct RuleEngine {
}

pub trait RuleEngineStore {
    fn correlate(&self, event : &mut SiemLog){

    }
    
    fn duplicate(&self) -> Box<dyn RuleEngineStore> {
        panic!("")
    }
}