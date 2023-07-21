use std::fmt::{Write, Error};

pub trait Encoder {
    fn encode<W: Write>(&self, f: &mut W, name : &str, description : &str, help : bool) -> Result<(), Error>;
}