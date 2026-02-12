pub mod error;
pub mod recording_operation;

use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::{error::ParseError, recording_operation::RecordingOperation, yp_bank_csv::YPBankCSV};

trait YPBankRecord {
    fn read_record<R: Read>(r: &mut R) -> Result<Vec<RecordingOperation>, ParseError>;
    fn write_record<W: Write>(w: &mut W) -> Result<String, ParseError>;
}

pub mod yp_bank_csv;

pub fn read_file(path_file: &str) -> Result<Vec<RecordingOperation>, ParseError> {
    let path = Path::new(path_file);

    if path.exists() {
        let mut file = File::open(path).map_err(ParseError::IoError)?;

        let records: Vec<RecordingOperation> = YPBankCSV::read_record(&mut file)?;

        Ok(records)
    } else {
        Err(ParseError::FileNotFound)
    }
}