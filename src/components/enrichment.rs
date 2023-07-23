use dyn_clone::{clone_trait_object, DynClone};

use crate::components::dataset::holder::DatasetHolder;
use crate::events::SiemLog;

/// A simple object with the logic to enrich Logs
pub trait LogEnrichment: Send + DynClone {
    /// Enrich the log with information from datasets
    fn enrich(&self, log: SiemLog, datasets: &DatasetHolder) -> SiemLog;
    /// Name of the enricher
    fn name(&self) -> &'static str;
    /// Description of the enricher
    fn description(&self) -> &'static str;
}
clone_trait_object!(LogEnrichment);

#[test]
fn check_basic_enricher_clone() {
    #[derive(Clone)]
    struct BasicLogEnricher{}
    impl LogEnrichment for BasicLogEnricher {
        fn enrich(&self, log: SiemLog, _datasets: &DatasetHolder) -> SiemLog {
            log
        }

        fn name(&self) -> &'static str {
            "a"
        }

        fn description(&self) -> &'static str {
            "a"
        }
    }
    fn test_boxed_enricher(enricher : &Box<dyn LogEnrichment>) -> Box<dyn LogEnrichment> {
        let _ = enricher.name();
        let enricher2 = enricher.clone();
        enricher2
    }
    let enricher1 : Box<dyn LogEnrichment> = Box::new(BasicLogEnricher{});
    let _enricher2 = test_boxed_enricher(&enricher1);
    
}