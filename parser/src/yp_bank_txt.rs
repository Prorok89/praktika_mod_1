use std::io::{BufRead, BufReader, Read, Write};

use crate::YPBankRecord;
use crate::error::ParseError;

use crate::recording_operation::{FieldRecordingOperation, RecordingOperation, Status, TxType};

pub struct YPBankTXT;

impl YPBankRecord for YPBankTXT {
    fn read_record<R: Read>(r: &mut R) -> Result<Vec<RecordingOperation>, ParseError> {
        todo!()
    }

    fn write_record<W: Write>(
        w: &mut W,
        records: &[RecordingOperation],
    ) -> Result<(), ParseError> {
        todo!()
    }
}

impl YPBankTXT {
    
}
