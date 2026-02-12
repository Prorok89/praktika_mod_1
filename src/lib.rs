pub mod recording_operation;
pub mod error;

use std::{fs::File, io::{self, Read}, path::Path};

use crate::{error::ParseError, recording_operation::RecordingOperation, yp_bank_csv::YPBankCSV};


trait YPBankRecord {
    
    fn read_record() -> Result<Vec<RecordingOperation>, ()>;
    fn write_record() -> Result<String, ()>;
}

enum AvailableExtensions {
    
}

pub mod yp_bank_csv;

pub fn check_file(path_file : &str) -> Result<bool, ParseError> {

    println!("{path_file}");

    let path = Path::new(path_file);

    if path.exists() {
        
        let file = File::open(path);

        match file {
            Ok(file) => {
                let mut reader = io::BufReader::new(file);
                let gg = YPBankCSV::read(&mut reader)?;
            }
            Err(_) => println!("222")
        }

        // if let Some(extension) =  path.extension() {
        //     match extension.to_string_lossy().to_lowercase().as_str() {
        //         "csv" => println!("1 - csv"),
        //         _ => println!(" not know")
        //     }
        // }
        Ok(true) 
    } else {
        Ok(false)
    }
}