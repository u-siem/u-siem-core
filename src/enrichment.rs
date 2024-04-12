use crate::{events::SiemLog, prelude::{store::DatasetStore, LogEnrichmentError}};

pub type LogEnrichment = fn(&mut SiemLog, &DatasetStore) -> Result<(), LogEnrichmentError>;


#[test]
fn should_enrich_logs() {
    #[allow(unused_variables)]
    fn super_enricher(log : &mut SiemLog, datasets: &DatasetStore) -> Result<(), LogEnrichmentError> {
        log.add_field("test", crate::events::field::SiemField::Null);
        Ok(())
    }
    #[allow(unused_variables)]
    fn super_bugged_enricher(log : &mut SiemLog, datasets: &DatasetStore) -> Result<(), LogEnrichmentError> {
        Err(LogEnrichmentError::Discard)
    }

    let mut enrichment_array : Vec<LogEnrichment> = Vec::new();
    enrichment_array.push(super_enricher);
    let mut log = SiemLog::new("test", 1234, "TST");
    let datasets = DatasetStore::new();
    enrichment_array.iter().try_for_each(|f| f(&mut log,&datasets)).expect("Should add field");
    assert!(log.has_field("test"));

    enrichment_array.push(super_bugged_enricher);
    let err = enrichment_array.iter().try_for_each(|f| f(&mut log,&datasets)).expect_err("Should return discard error");
    assert_eq!(LogEnrichmentError::Discard, err);
}
