pub mod error;
pub mod recording_operation;
mod yp_bank_csv;
mod yp_bank_txt;
mod yp_bank_bin;

use std::{
    fs::File,
    io::{BufWriter, Read, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    error::ParseError, recording_operation::RecordingOperation, yp_bank_bin::YPBankBIN, yp_bank_csv::YPBankCSV, yp_bank_txt::YPBankTXT
};

/// Поддерживаемые расширения файлов
pub enum Extension {
    Csv,
    Bin,
    Txt,
}
impl Extension {
    pub fn as_str(&self) -> &'static str {
        match self {
            Extension::Bin => "bin",
            Extension::Csv => "csv",
            Extension::Txt => "txt",
        }
    }

    pub fn str_to_extension(extension: &str) -> Result<Extension, ParseError> {
        match extension {
            "csv" => Ok(Extension::Csv),
            "txt" => Ok(Extension::Txt),
            "bin" => Ok(Extension::Bin),
            _ => Err(ParseError::ExtensionNotSupported),
        }
    }
}

trait YPBankRecord {
    fn read_record<R: Read>(r: &mut R) -> Result<Vec<RecordingOperation>, ParseError>;
    fn write_record<W: Write>(w: &mut W, records: &[RecordingOperation]) -> Result<(), ParseError>;
}

///Чтение операции из файла
pub fn read_file(format: &str, path_file: &str) -> Result<Vec<RecordingOperation>, ParseError> {
    let path = Path::new(path_file);

    if path.exists() {
        let mut file = File::open(path).map_err(ParseError::IoError)?;

        let records: Vec<RecordingOperation> = match Extension::str_to_extension(format)? {
            Extension::Csv => YPBankCSV::read_record(&mut file)?,
            Extension::Bin => YPBankBIN::read_record(&mut file)?,
            Extension::Txt => YPBankTXT::read_record(&mut file)?,
        };

        Ok(records)
    } else {
        Err(ParseError::FileNotFound)
    }
}

///Запись операции в файл
pub fn write_file(format: &str, records: &[RecordingOperation]) -> Result<String, ParseError> {
    let file_name = format!("output_file_{}.{}", get_file_name()?, format);

    let f = File::create(&file_name)?;
    let mut buf = BufWriter::new(f);

    match Extension::str_to_extension(format)? {
        Extension::Csv => YPBankCSV::write_record(&mut buf, records)?,
        Extension::Bin => YPBankBIN::write_record(&mut buf, records)?,
        Extension::Txt => YPBankTXT::write_record(&mut buf, records)?,
    }

    buf.flush()?;

    Ok(file_name)
}

fn get_file_name() -> Result<String, ParseError> {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH)?;
    let unix_timestamp_millis =
        duration.as_secs() * 1000 + duration.subsec_nanos() as u64 / 1_000_000;

    Ok(unix_timestamp_millis.to_string())
}