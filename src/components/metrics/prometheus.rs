use std::fmt::{Error, Write};

pub trait Encoder {
    fn encode<W: Write>(
        &self,
        f: &mut W,
        name: &str,
        description: &str,
        help: bool,
    ) -> Result<(), Error>;
}
