use parser::{Extension, error::ParseError};
use std::{
    env,
    io::{self, Write},
};

fn main() -> Result<(), ParseError> {
    let mut args = env::args().skip(1);

    if args.len() < 6 {
        panic!(
            "Invalid command, example:  --file1 <file_name> --format1 <bin|csv|txt> --file2 <file_name> --format2 <bin|csv|txt>"
        )
    }

    let mut file_path_one = String::new();
    let mut file_path_two = String::new();
    let mut format_file_one = String::new();
    let mut format_file_two = String::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--file1" => {
                if let Some(v) = args.next() {
                    file_path_one = v;
                }
            }
            "--file2" => {
                if let Some(v) = args.next() {
                    file_path_two = v;
                }
            }
            "--format1" => {
                if let Some(v) = args.next() {
                    format_file_one = v;
                }
            }
            "--format2" => {
                if let Some(v) = args.next() {
                    format_file_two = v;
                }
            }
            _ => panic!("command '{}' not found", arg),
        }
    }

    if file_path_one.is_empty() {
        panic!("File1 not specified");
    }
    /*
    
    Record number 1
     */

    if file_path_two.is_empty() {
        panic!("File2 not specified");
    }

    if format_file_one.is_empty() {
        panic!("Format file1 not specified");
    }

    if format_file_two.is_empty() {
        panic!("Format file2 not specified");
    }

    if let Err(er) = Extension::str_to_extension(&format_file_one) {
        panic!("Format file1 no found");
    }

    if let Err(er) = Extension::str_to_extension(&format_file_two) {
        panic!("Format file2 no found");
    }

    let records_one = parser::read_file(&format_file_one, &file_path_one)?;
    let records_two = parser::read_file(&format_file_two, &file_path_two)?;
    
    if records_one.len() != records_two.len() {
         panic!("The number of transactions is different: file1 - {}, file2 - {}", records_one.len(), records_two.len());
    }

    for (_i, (r1, r2)) in records_one.iter().zip(records_two.iter()).enumerate() {
        if r1 != r2 {
            panic!("Differs: file1: \n {:#?} \n file2: \n {:#?}",r1, r2);
        }
    }

    println!("The files are equal");

    Ok(())
}