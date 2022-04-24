use dyn_clone::{clone_trait_object, DynClone};
use serde::Serialize;

use crate::events::{SiemLog, schema::FieldSchema};

use super::dataset::holder::DatasetHolder;


/// A simple object with the logic to parse Logs.
pub trait LogParser: DynClone + Send {
    /// Parse the log. If it fails it must give a reason why. This allow optimization of the parsing process.
    fn parse_log(&self, log: SiemLog, datasets : &DatasetHolder) -> Result<SiemLog, LogParsingError>;
    /// Name of the parser
    fn name(&self) -> &str;
    /// Description of the parser
    fn description(&self) -> &str;
    /// Get parser schema
    fn schema(&self) -> &'static FieldSchema;
    /// Get a log generator to test this parser
    fn generator(&self) -> Box<dyn LogGenerator>;
}
clone_trait_object!(LogParser);

/// This is the most complex type of parser. It's statefull to store past logs.
/// Think of the USB event in linux, we need the rest of the logs to extract all information.
/// The Parser component which uses this parsers must be able to store and load past Logs
/// if the user connects to a different SIEM node (LoadBalancing).
pub trait MultilineLogParser: DynClone + Send {
    /// Parse the log. If it fails it must give a reason why. This allow optimization of the parsing process.
    fn parse_log(&mut self, log: SiemLog, datasets : &DatasetHolder) -> Result<Option<SiemLog>, LogParsingError>;
    /// Name of the parser
    fn name(&self) -> &str;
    /// Description of the parser
    fn description(&self) -> &str;
    /// The connection with the origin has been closed. We must preserve the logs stored inside this parser
    /// so another node can use them to parse the logs of the same machine.
    fn cleaning(&mut self) -> Vec<SiemLog>;
    /// Return those logs that would not be used by the parser, or are older as to reduce the memmory usage.
    fn unused(&mut self) -> Vec<SiemLog>;
    /// Get parser schema
    fn schema(&self) -> &'static FieldSchema;
}

clone_trait_object!(MultilineLogParser);

/// Error at parsing a log
#[derive(Serialize, Debug)]
pub enum LogParsingError {
    /// The parser can't be used with this log
    NoValidParser(SiemLog),
    /// The log is for this parser but there is a bug in the code
    ParserError(SiemLog, String),
    /// The log is for this parser but the submodule has not been implemented.
    NotImplemented(SiemLog),
    /// The log has change format the parser cant process it.
    FormatError(SiemLog, String)
}


pub trait LogGenerator {
    /// Generate a random log
    fn log(&self) -> String;
    /// Of the total overall logs that are generated in an organization, 
    /// whats the procentage of logs generated by this source?
    /// The bigger, the most probability of being generated
    fn weight(&self) -> u8;
}
