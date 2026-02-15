use std::{error::Error, fmt, io, time::SystemTimeError};

use crate::recording_operation::FieldRecordingOperation;

#[derive(Debug)]
pub enum ParseError {
    FileIsEmpty,
    FileNotFound,
    IncorrectTitle,
    IncorrectDesciprtion,
    IncorrectFields {
        field : FieldRecordingOperation,
        value : String
    },
    IoError (io::Error),
    IncorrectStatus {
        status : String
    },
    IncorrectOperation {
        operation : String
    },
    SystemTimeError,
    ExtensionNotSupported,
    NotKnow, //заменить все вхождения
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IncorrectTitle => write!(f, "incorrect csv title"),
            _ => write!(f, "ERROR not foung")
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
        ParseError::SystemTimeError
    }
}