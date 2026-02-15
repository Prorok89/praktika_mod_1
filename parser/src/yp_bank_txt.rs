use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};

use crate::YPBankRecord;
use crate::error::ParseError;

use crate::recording_operation::{FieldRecordingOperation, RecordingOperation, Status, TxType};

pub struct YPBankTXT;

impl YPBankRecord for YPBankTXT {
    fn read_record<R: Read>(r: &mut R) -> Result<Vec<RecordingOperation>, ParseError> {
        let mut records: Vec<RecordingOperation> = Vec::new();

        let mut buf_reader = BufReader::new(r);

        let lines = buf_reader.lines();

        let mut fields: HashMap<String, String> = HashMap::new();

        let mut new_record: bool = true;

        for line in lines {
            match line {
                Ok(l) => {
                    if l.is_empty() {
                        new_record = true;

                        if fields.len() == 8 {
                            records.push(Self::create(&fields)?);
                        }

                        continue;
                    }
                    
                    if l.starts_with("#") {
                        continue;
                    }
                    
                    if new_record {
                        fields.clear();
                    }
                    
                    new_record = false;
                    
                    let parts: Vec<&str> = l.splitn(2, ':').collect();

                    if parts.len() == 2 {
                        let key = parts[0].trim().to_string();
                        let value = parts[1].trim().to_string();
                        fields.insert(key, value);
                    } else {
                        return Err(ParseError::StringIncorrect {
                            value: l.to_string(),
                        });
                    }
                }
                Err(_) => return Err(ParseError::NotKnow),
            }
        }

        Ok(records)
    }

    fn write_record<W: Write>(w: &mut W, records: &[RecordingOperation]) -> Result<(), ParseError> {
        todo!()
    }
}

impl YPBankTXT {
    fn create(fields: &HashMap<String, String>) -> Result<RecordingOperation, ParseError> {
        let description = Self::get_description(&Self::get_field(&fields,FieldRecordingOperation::Description)?)?;

        let recording_operation: RecordingOperation = RecordingOperation {
            tx_id: Self::parse_u64(&Self::get_field(&fields,FieldRecordingOperation::TxId)?, FieldRecordingOperation::TxId)?,
            tx_type: TxType::str_to_tx_type(&Self::get_field(&fields,FieldRecordingOperation::TxType)?)?,
            from_user_id: Self::parse_u64(&Self::get_field(&fields,FieldRecordingOperation::FromUserId)?, FieldRecordingOperation::FromUserId)?,
            to_user_id: Self::parse_u64(&Self::get_field(&fields,FieldRecordingOperation::ToUserId)?, FieldRecordingOperation::ToUserId)?,
            amount: Self::parse_i64(&Self::get_field(&fields,FieldRecordingOperation::Amount)?, FieldRecordingOperation::Amount)?,
            timestamp: Self::parse_i64(&Self::get_field(&fields,FieldRecordingOperation::Timestamp)?, FieldRecordingOperation::Timestamp)?,
            status: Status::str_to_status(&Self::get_field(&fields,FieldRecordingOperation::Status)?)?,
            desc_len: description.len(),
            description,
        };

        Ok(recording_operation)
    }

    fn get_field(fields: &HashMap<String, String>, name_field : FieldRecordingOperation) -> Result<String, ParseError> {

        match  fields.get(name_field.as_str()) {
            Some(f) => Ok(f.to_string()),
            _ => Err(ParseError::MissingField { field : name_field.as_str().to_string()}),
        }
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
}
