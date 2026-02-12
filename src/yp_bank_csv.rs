use std::io::{BufRead, BufReader, Read};

use crate::error::ParseError;

use crate::recording_operation::{FieldRecordingOperation, RecordingOperation, Status, TxType};

const HEADER: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

const EXTENSION: &str = "csv";

pub struct YPBankCSV {
    pub path_file: String,
}

impl YPBankCSV {
    pub fn read<R: std::io::Read>(r: &mut R) -> Result<Vec<RecordingOperation>, ParseError> {
        let mut records: Vec<RecordingOperation> = Vec::new();

        let mut buf_reader = BufReader::new(r);

        let mut header = String::new();
        let first_line_size = buf_reader.read_line(&mut header)?;

        if first_line_size == 0 {
            return Err(ParseError::FileIsEmpty);
        }

        if header.trim() != HEADER {
            return Err(ParseError::IncorrectTitle);
        }

        let lines = buf_reader.lines();

        for line in lines {
            match line {
                Ok(l) => {
                    if !l.is_empty() {
                        let record = Self::create(&l)?;
                        records.push(record);
                    }
                }
                Err(_) => return Err(ParseError::NotKnow),
            }
        }

        Ok(records)
    }

    fn create(line: &str) -> Result<RecordingOperation, ParseError> {
        let elem: Vec<&str> = line.trim().split(',').collect();

        if elem.len() == 8 {
            let description = Self::get_description(elem[7])?;

            let d: RecordingOperation = RecordingOperation {
                tx_id: Self::parse_u64(elem[0], FieldRecordingOperation::TxId)?,
                tx_type: TxType::from_str(elem[1])?,
                from_user_id: Self::parse_u64(elem[2], FieldRecordingOperation::FromUserId)?,
                to_user_id: Self::parse_u64(elem[3], FieldRecordingOperation::ToUserId)?,
                amount: Self::parse_i64(elem[4], FieldRecordingOperation::Amount)?,
                timestamp: Self::parse_i64(elem[5], FieldRecordingOperation::Timestamp)?,
                status: Status::from_str(elem[6])?,
                desc_len: description.len(),
                description: description,
            };

            return Ok(d);
        }

        Err(ParseError::NotKnow)
    }

    fn parse_u64(i: &str, field: FieldRecordingOperation) -> Result<u64, ParseError> {
        match i.parse::<u64>() {
            Ok(u) => Ok(u),
            Err(_) => Err(ParseError::IncorrectFields {
                field,
                value: i.to_string(),
            }),
        }
    }

    fn parse_i64(i: &str, field: FieldRecordingOperation) -> Result<i64, ParseError> {
        match i.parse::<i64>() {
            Ok(u) => Ok(u),
            Err(_) => Err(ParseError::IncorrectFields {
                field,
                value: i.to_string(),
            }),
        }
    }

    fn get_description(description: &str) -> Result<String, ParseError> {
        if description.starts_with('"') && description.ends_with('"') {
            Ok(description[1..(description.len() - 1)].to_string())
        } else {
            Err(ParseError::IncorrectDesciprtion)
        }
    }

    pub fn new(path_file: String) -> Self {
        Self {
            path_file: path_file,
        }
    }

    pub fn check_extension(extension: &str) -> bool {
        extension == EXTENSION
    }
}
