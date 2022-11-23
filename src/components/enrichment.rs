use dyn_clone::{clone_trait_object, DynClone};

use crate::components::dataset::holder::DatasetHolder;
use crate::events::SiemLog;

/// A simple object with the logic to enrich Logs
pub trait LogEnrichment: DynClone + Send {
    /// Enrich the log with information from datasets
    fn enrich(&self, log: SiemLog, datasets: &DatasetHolder) -> SiemLog;
    /// Name of the enricher
    fn name(&self) -> &str;
    /// Description of the enricher
    fn description(&self) -> &str;
}
clone_trait_object!(LogEnrichment);
