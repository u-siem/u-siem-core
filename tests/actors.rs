use usiem::{components::{collector::{LogCollectorAddr, LogCollectorContext, LogCollectorHandler}, parser::{LogParsingContext, LogProcessorHandler}}, datasets::store::DatasetStore, err::{ComponentError, SiemResult}, runtime::{actor::Actor, task::run_collector, Runtime}};

pub struct SuperCollector {
    
}

impl SuperCollector {
    pub fn new() -> Self {
        Self {}
    }
}

impl Actor for SuperCollector {
    type Context = LogCollectorContext;
    fn work(&mut self) -> Result<(), ComponentError>  {
        Ok(())
    }
}

impl LogCollectorHandler for SuperCollector {}

fn build_parser(datasets : DatasetStore) -> LogCollectorAddr {
    let parser = SuperCollector::new();
    //parser.start();
    run_collector(parser, datasets)
}

#[test]
fn should_run_actor() {
    Runtime::init();
    Runtime::register_collector(build_parser);
    Runtime::start();
}