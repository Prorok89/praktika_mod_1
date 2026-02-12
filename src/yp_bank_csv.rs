use std::io::{BufRead, BufReader, Read, Write};

use crate::YPBankRecord;
use crate::error::ParseError;

use crate::recording_operation::{FieldRecordingOperation, RecordingOperation, Status, TxType};

const HEADER: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

const EXTENSION: &str = "csv";

pub struct YPBankCSV;

impl YPBankRecord for YPBankCSV {
    fn read_record<R: Read>(r: &mut R) -> Result<Vec<RecordingOperation>, ParseError> {
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
                    if l.is_empty() {
                        continue;
                    }

                    let elems = Self::parse_csv_line(&l);

                    if elems.len() == 8 {
                        let record = Self::create(elems)?;
                        records.push(record);
                    } else {
                        return Err(ParseError::NotKnow);
                    }
                }
                Err(_) => return Err(ParseError::NotKnow),
            }
        }

        Ok(records)
    }

    fn write_record<W: Write>(w: &mut W) -> Result<String, ParseError> {
        todo!()
    }
}

impl YPBankCSV {
    fn create(elems: Vec<&str>) -> Result<RecordingOperation, ParseError> {
        let description = Self::get_description(elems[7])?;

        let d: RecordingOperation = RecordingOperation {
            tx_id: Self::parse_u64(elems[0], FieldRecordingOperation::TxId)?,
            tx_type: TxType::str_to_tx_type(elems[1])?,
            from_user_id: Self::parse_u64(elems[2], FieldRecordingOperation::FromUserId)?,
            to_user_id: Self::parse_u64(elems[3], FieldRecordingOperation::ToUserId)?,
            amount: Self::parse_i64(elems[4], FieldRecordingOperation::Amount)?,
            timestamp: Self::parse_i64(elems[5], FieldRecordingOperation::Timestamp)?,
            status: Status::str_to_status(elems[6])?,
            desc_len: description.len(),
            description,
        };

        Ok(d)
    }

    fn parse_csv_line(line: &str) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        let mut start = 0;

        for (i, char) in line.char_indices() {
            if char == '"' {
                break;
            } else if char == ',' {
                result.push(&line[start..i]);
                start = i + 1;
            }
        }

        result.push(&line[start..]);
        result
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

    pub fn check_extension(extension: &str) -> bool {
        extension == EXTENSION
    }
}
