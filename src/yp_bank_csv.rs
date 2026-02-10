use std::io::{BufRead, BufReader, Read};

use crate::recording_operation::{RecordingOperation, Status, TxType};

const HEADER: &str = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";
const EXTENSION: &str = "csv";

pub struct YPBankCSV {
    pub path_file: String,
}

impl YPBankCSV {
    pub fn read<R: std::io::Read>(r: &mut R) -> Result<bool, ()> {
        let mut records: Vec<RecordingOperation> = Vec::new();
        let buf_reader = BufReader::new(r);

        let mut lines = buf_reader.lines();

        if let Some(Ok(first_line)) = lines.next() {
            if first_line == HEADER {
                let mut gg = 0;
                println!("Файл корректный");
                for line in lines {
                    if let Ok(l) = line {
                        if !l.is_empty() {
                            let ff = Self::create(&l)?;
                            gg+=1;
                            println!("{:#?}", ff);
                        }
                    }
                    if gg > 10 {

                        break;
                    }
                }
            } else {
                println!("Файл не корректный");
            }
        } else {
            // вернуть ошибку, файл пустой
        }

        // r.lines();

        // println!("{}", lines);

        Ok(false)
    }

    fn create(line: &str) -> Result<RecordingOperation, ()> {
        let elem: Vec<&str> = line.trim().split(',').collect();

        if elem.len() == 8 {

            let description = Self::get_description(elem[7])?;

            let d: RecordingOperation = RecordingOperation {
                tx_id: Self::parse_u64(elem[0])?,
                tx_type: TxType::from_str(elem[1])?,
                from_user_id: Self::parse_u64(elem[2])?,
                to_user_id: Self::parse_u64(elem[3])?,
                amount: Self::parse_i64(elem[4])?,
                timestamp: Self::parse_i64(elem[5])?,
                status: Status::from_str(elem[6])?,
                desc_len: description.len(),
                description: description,
            };

            return Ok(d);
        }

        Err(())
    }

    fn parse_u64(i: &str) -> Result<u64, ()> {
        match i.parse() {
            Ok(u) => Ok(u),
            Err(_) => {
                println!("Error parse_u64");
                Err(())
            }
        }
    }

    fn parse_i64(i: &str) -> Result<i64, ()> {
        match i.parse() {
            Ok(u) => Ok(u),
            Err(_) => {
                println!("Error parse_i64");
                Err(())
            }
        }
    }

    fn get_description(description: &str) -> Result<String, ()> {
        
        if description.starts_with('"') && description.ends_with('"') {
            Ok(description[1..(description.len()-1)].to_string())
        } else {
            Err(())
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
