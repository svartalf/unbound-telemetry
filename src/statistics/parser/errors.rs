use std::error;
use std::fmt;
use std::num;

#[derive(Debug)]
pub enum ParseError {
    UnknownKey {
        key: String,
    },
    MissingKey,
    MissingValue {
        key: String,
    },
    ParseInt(num::ParseIntError),
    ParseFloat(num::ParseFloatError),
    ParseStr(String),

    /// Generic error kind if none of other variants can suit to the case.
    InvalidFormat,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnknownKey { key } => f.write_fmt(format_args!("Unknown key {}", key)),
            ParseError::MissingKey => f.write_str("Text line is missing a key definition"),
            ParseError::MissingValue { key } => f.write_fmt(format_args!("Missing value for key {}", key)),
            ParseError::ParseInt(e) => fmt::Display::fmt(e, f),
            ParseError::ParseFloat(e) => fmt::Display::fmt(e, f),
            ParseError::ParseStr(value) => f.write_fmt(format_args!("Unable to parse '{}' value", value)),
            ParseError::InvalidFormat => f.write_str("Invalid data format"),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParseError::ParseInt(e) => Some(e),
            ParseError::ParseFloat(e) => Some(e),
            _ => None,
        }
    }
}

impl From<num::ParseFloatError> for ParseError {
    fn from(e: num::ParseFloatError) -> Self {
        ParseError::ParseFloat(e)
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(e: num::ParseIntError) -> Self {
        ParseError::ParseInt(e)
    }
}
