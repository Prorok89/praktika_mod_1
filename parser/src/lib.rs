pub mod error;
pub mod recording_operation;

use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::Path, time::{SystemTime, UNIX_EPOCH},
};

use crate::{error::ParseError, recording_operation::{RecordingOperation}, yp_bank_csv::YPBankCSV};

trait YPBankRecord {
    fn read_record<R: Read>(r: &mut R) -> Result<Vec<RecordingOperation>, ParseError>;
    fn write_record<W: Write>(w: &mut W, records : &[RecordingOperation]) -> Result<(), ParseError>;
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

pub fn write_file(format : String, records : &[RecordingOperation]) -> Result<String, ParseError>{

    /*
        1. создать файл 
        2. записать в файл
        3. вывести название файла в консоль
    */

    let file_name = format!("output_file_{}.{}", get_file_name()?, format);

    let f = File::create(&file_name)?;
    let mut buf = BufWriter::new(f);

    YPBankCSV::write_record(&mut buf, records)?;

    buf.flush()?;

    Ok(file_name)
}

fn get_file_name() -> Result<String, ParseError>{
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH)?;
    let unix_timestamp_millis = duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000;

    Ok(unix_timestamp_millis.to_string())
}