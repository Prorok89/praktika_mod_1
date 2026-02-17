use std::{error::Error, fmt, io, string::FromUtf8Error, time::SystemTimeError};

use crate::recording_operation::FieldRecordingOperation;

#[derive(Debug)]
pub enum ParseError {
    IncorrectTitle,
    FileIsEmpty,
    FileNotFound,
    StringIncorrect {
        value: String,
    },
    IncorrectDesciprtion,
    IncorrectFields {
        field: FieldRecordingOperation,
        value: String,
    },
    IoError(io::Error),
    IncorrectStatus {
        status: String,
    },
    IncorrectOperation {
        operation: String,
    },
    SystemTimeError {
        value: String,
    },
    ExtensionNotSupported,
    MissingField {
        field: String,
    },
    InvalidString {
        value: String,
    },
    CreateOperation
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IncorrectTitle => write!(f, "incorrect title"),
            ParseError::FileIsEmpty => write!(f, "file is empty"),
            ParseError::FileNotFound => write!(f, "file not found"),
            ParseError::StringIncorrect {value} => write!(f, "string incorrect: {}", value),
            ParseError::IncorrectDesciprtion => write!(f, "incorrect desciprtion"),
            ParseError::IncorrectFields {field, value} => write!(f, "incorrect fields {} = {}", field, value),
            ParseError::IoError(e) => write!(f, "error IO : {}", e),
            ParseError::IncorrectStatus {status} => write!(f, "incorrect status: {}", status),
            ParseError::IncorrectOperation  {operation}=> write!(f, "incorrect operation : {}", operation),
            ParseError::SystemTimeError {value}=> write!(f, "system time error: {}", value),
            ParseError::ExtensionNotSupported => write!(f, "extension not supported"),
            ParseError::MissingField {field}=> write!(f, "missing field : {}", field),
            ParseError::InvalidString {value}=> write!(f, "invalid string: {}", value),
            ParseError::CreateOperation => writeln!(f, "error create operation"),
        }
    }
}

impl Error for ParseError {}

impl From<std::io::Error> for ParseError {
    fn from(value: std::io::Error) -> Self {
        ParseError::IoError(value)
    }
}

impl From<SystemTimeError> for ParseError {
    fn from(value: SystemTimeError) -> Self {
        ParseError::SystemTimeError {
            value: value.to_string(),
        }
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(value: FromUtf8Error) -> Self {
        ParseError::InvalidString {
            value: value.to_string(),
        }
    }
}
