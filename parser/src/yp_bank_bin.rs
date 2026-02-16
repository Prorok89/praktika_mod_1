use std::io::{BufRead, BufReader, Read, Write};

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
        
            r.read_exact(&mut magic)?;

            if magic != MAGIC {
                return Err(ParseError::IncorrectTitle);
            }
            
            let mut record_size_bytes = [0u8; 4];

            r.read_exact(&mut record_size_bytes)?;
            let record_size = u32::from_be_bytes(record_size_bytes);
            
            


            break;

        }

        // for line in lines {
        //     match line {
        //         Ok(l) => {
        //             if l.is_empty() {
        //                 continue;
        //             }

        //             let elems = Self::parse_csv_line(&l);

        //             if elems.len() == 8 {
        //                 let record = Self::create(elems)?;
        //                 records.push(record);
        //             } else {
        //                 return Err(ParseError::NotKnow);
        //             }
        //         }
        //         Err(_) => return Err(ParseError::NotKnow),
        //     }
        // }

        Ok(records)
    }

    fn write_record<W: Write>(w: &mut W, records: &[RecordingOperation]) -> Result<(), ParseError> {
        todo!()
    }
}

impl YPBankBIN {}
