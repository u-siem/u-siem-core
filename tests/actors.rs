use usiem::{components::{collector::{LogCollectorContext, LogCollectorHandler}, parser::{LogParsingContext, LogProcessorHandler}}, err::ComponentError, events::SiemLog, runtime::{actor::Actor, address::ActorAddr, task::{run_collector, run_parser}, Runtime}};

pub struct SuperCollector {}

impl SuperCollector {
    pub fn new() -> Self {
        Self {}
    }
}

impl Actor for SuperCollector {
    type Context = LogCollectorContext;
    fn step(&mut self, ctx : &mut Self::Context) -> Result<(), ComponentError>  {
        ctx.ingest(SiemLog::new("TEST", 1, "TST"));
        Ok(())
    }
}

impl LogCollectorHandler for SuperCollector {}

pub struct SuperParser {}

impl SuperParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Actor for SuperParser {
    type Context = LogParsingContext;
    fn step(&mut self, ctx : &mut Self::Context) -> Result<(), ComponentError>  {
        Ok(())
    }
}

impl LogProcessorHandler for SuperParser {
    #[allow(unused_variables)]
    fn parse_log(&mut self, log: usiem::prelude::SiemLog, ctx: &mut LogParsingContext) {
        println!("{:?}", log);
    }
}

fn build_parser(ctx : LogParsingContext) -> ActorAddr {
    let parser = SuperParser::new();
    run_parser(parser, ctx)
}

fn build_collector(ctx : LogCollectorContext) -> ActorAddr {
    let collector = SuperCollector::new();
    run_collector(collector, ctx)
}

#[test]
fn should_run_actor() {
    Runtime::init();
    Runtime::register_collector(build_collector);
    Runtime::register_parser(build_parser);
    Runtime::start();
}