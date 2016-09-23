use std::fmt;
use std::error::Error as StdError;


/// An error related to the rdf-rs module.
#[derive(Debug)]
pub enum Error {
  InvalidWriterOutput,
  InvalidReaderInput,
  EndOfInput,
  InvalidByteEncoding,
}


impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::InvalidWriterOutput => write!(f, "Invalid writer output"),
      Error::InvalidReaderInput => write!(f, "Invalid reader input"),
      Error::EndOfInput => write!(f, "End of input"),
      Error::InvalidByteEncoding => write!(f, "Cannot decode bytes"),
    }
  }
}


impl StdError for Error {
  fn description(&self) -> &str {
    match *self {
      Error::InvalidWriterOutput => "Invalid writer output",
      Error::InvalidReaderInput => "Invalid reader input",
      Error::EndOfInput => "End of input",
      Error::InvalidByteEncoding => "Cannot decode bytes",
    }
  }
}