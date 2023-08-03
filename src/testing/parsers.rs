use crate::prelude::{LogParser, SiemLog, holder::DatasetHolder, LogParsingError, FieldSchema, LogGenerator, SiemField, GeneratorConfig};

pub struct DummyLogGenerator {}

impl LogGenerator for DummyLogGenerator {
    fn log(&self) -> String {
        "This is a dummy log".to_string()
    }

    fn weight(&self) -> u8 {
        1
    }

    fn configure(&mut self, _config: GeneratorConfig) {}
}


/// Parser that only parses a log if the message contains the word "DUMMY".
/// 
/// Adds an extra field called "parser" with the content "DummyParserText"
#[derive(Clone)]
pub struct DummyParserText {
    schema : FieldSchema
}
impl DummyParserText {
    pub fn new() -> Self {
        Self {
            schema : FieldSchema::new()
        }
    }
}

impl LogParser for DummyParserText {
    fn parse_log(
        &self,
        mut log: SiemLog,
        _datasets: &DatasetHolder,
    ) -> Result<SiemLog, LogParsingError> {
        if !log.message().contains("DUMMY") {
            return Err(LogParsingError::NoValidParser(log));
        }
        log.add_field("parser", SiemField::from_str("DummyParserText"));
        Ok(log)
    }
    fn name(&self) -> &'static str {
        "DummyParserText"
    }
    fn description(&self) -> &'static str {
        "This is a dummy that parsers if contains DUMMY in text"
    }
    fn schema(&self) -> & FieldSchema {
        &self.schema
    }

    fn generator(&self) -> Box<dyn LogGenerator> {
        return Box::new(DummyLogGenerator {});
    }
}

/// A simple parser that always parses logs.
/// 
/// Adds an extra field called "parser" with the content "DummyParserAll"
#[derive(Clone)]
pub struct DummyParserAll {
    schema : FieldSchema
}
impl DummyParserAll {
    pub fn new() -> Self {
        Self {
            schema : FieldSchema::new()
        }
    }
}

impl LogParser for DummyParserAll {
    fn parse_log(
        &self,
        mut log: SiemLog,
        _datasets: &DatasetHolder,
    ) -> Result<SiemLog, LogParsingError> {
        log.add_field("parser", SiemField::from_str("DummyParserAll"));
        Ok(log)
    }
    fn name(&self) -> &'static str {
        "DummyParserAll"
    }
    fn description(&self) -> &'static str {
        "This is a dummy parser that always parses logs"
    }
    fn schema(&self) -> & FieldSchema {
        &self.schema
    }

    fn generator(&self) -> Box<dyn LogGenerator> {
        return Box::new(DummyLogGenerator {});
    }
}

/// Parser that always returns a parser error
#[derive(Clone)]
pub struct DummyParserError {    
    schema : FieldSchema
}
impl DummyParserError {
    pub fn new() -> Self {
        Self {
            schema : FieldSchema::new()
        }
    }
}

impl LogParser for DummyParserError {
    fn parse_log(
        &self,
        log: SiemLog,
        _datasets: &DatasetHolder,
    ) -> Result<SiemLog, LogParsingError> {
        return Err(LogParsingError::ParserError(log, format!("Bug in parser")));
    }
    fn name(&self) -> &'static str {
        "DummyParserError"
    }
    fn description(&self) -> &'static str {
        "This is a parser that cannot parse because it has a bug"
    }
    fn schema(&self) ->  &FieldSchema {
        &self.schema
    }

    fn generator(&self) -> Box<dyn LogGenerator> {
        return Box::new(DummyLogGenerator {});
    }
}
