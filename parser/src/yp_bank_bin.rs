use std::io::{BufRead, BufReader, Cursor, ErrorKind, Read, Write};

use crate::YPBankRecord;
use crate::error::ParseError;

use crate::recording_operation::{FieldRecordingOperation, RecordingOperation, Status, TxType};

const MAGIC: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];
pub struct YPBankBIN;

impl YPBankRecord for YPBankBIN {
    fn read_record<R: Read>(r: &mut R) -> Result<Vec<RecordingOperation>, ParseError> {
        let mut records: Vec<RecordingOperation> = Vec::new();

        loop {
            let mut magic = [0u8; 4];

            if let Err(er) = r.read_exact(&mut magic) {
                match er.kind() {
                    ErrorKind::UnexpectedEof => break,
                    _ =>  return  Err(ParseError::IncorrectTitle)
                }
            }

            if magic != MAGIC {
                return Err(ParseError::IncorrectTitle);
            }

            let record_size = YPBankBIN::parse_u32(r)? as usize;
            let mut body_bytes = vec![0u8; record_size];
            r.read_exact(&mut body_bytes)?;

            let record = YPBankBIN::create(&body_bytes)?;
            records.push(record);
        }

        Ok(records)
    }

    fn write_record<W: Write>(w: &mut W, records: &[RecordingOperation]) -> Result<(), ParseError> {
        
        for record in records.iter() {
            w.write_all(&MAGIC)?;

            let mut body_bytes: Vec<u8> = Vec::new();

            body_bytes.extend(record.tx_id.to_be_bytes().iter());
            body_bytes.extend([record.tx_type.as_u8()].iter());
            body_bytes.extend(record.from_user_id.to_be_bytes().iter());
            body_bytes.extend(record.to_user_id.to_be_bytes().iter());
            body_bytes.extend(record.amount.to_be_bytes().iter());
            body_bytes.extend(record.timestamp.to_be_bytes().iter());
            body_bytes.extend([record.status.as_u8()].iter());
            body_bytes.extend((record.desc_len as u32).to_be_bytes().iter());
            body_bytes.extend(format!("\"{}\"", record.description).as_bytes().iter());
            
            w.write_all(&(body_bytes.len() as u32).to_be_bytes())?;

            w.write_all(&body_bytes)?;
        }

        Ok(())
    }
}

impl YPBankBIN {
    fn parse_u8<R: Read>(reader: &mut R) -> Result<u8, ParseError> {
        let mut bytes = [0u8; 1];
        reader.read_exact(&mut bytes)?;
        Ok(u8::from_be_bytes(bytes))
    }

    fn parse_u32<R: Read>(reader: &mut R) -> Result<u32, ParseError> {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        Ok(u32::from_be_bytes(bytes))
    }

    fn parse_u64<R: Read>(reader: &mut R) -> Result<u64, ParseError> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        Ok(u64::from_be_bytes(bytes))
    }

    fn parse_i64<R: Read>(reader: &mut R) -> Result<i64, ParseError> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes)?;
        Ok(i64::from_be_bytes(bytes))
    }

    fn get_description(description: &str) -> Result<String, ParseError> {
        if description.starts_with('"') && description.ends_with('"') {
            Ok(description[1..(description.len() - 1)].to_string())
        } else {
            Err(ParseError::IncorrectDesciprtion)
        }
    }

    fn create(body: &[u8]) -> Result<RecordingOperation, ParseError> {
        let mut reader = Cursor::new(body);

        let tx_id = Self::parse_u64(&mut reader)?;
        let tx_type = TxType::from_u8(Self::parse_u8(&mut reader)?)?;
        let from_user_id = Self::parse_u64(&mut reader)?;
        let to_user_id = Self::parse_u64(&mut reader)?;
        let amount = Self::parse_i64(&mut reader)?;
        let timestamp = Self::parse_i64(&mut reader)?;
        let status = Status::from_u8(Self::parse_u8(&mut reader)?)?;

        let desc_len = Self::parse_u32(&mut reader)? as usize;

        let mut description_bytes = vec![0u8; desc_len];
        reader.read_exact(&mut description_bytes)?;

        let description = Self::get_description(&String::from_utf8(description_bytes)?)?;

        let recording_operation: RecordingOperation = RecordingOperation {
            tx_id,
            tx_type,
            from_user_id,
            to_user_id,
            amount,
            timestamp,
            status,
            desc_len,
            description,
        };

        Ok(recording_operation)
    }
}
