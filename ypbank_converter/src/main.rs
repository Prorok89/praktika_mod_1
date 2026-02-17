use parser::{Extension, error::ParseError};
use std::{
    env,
    io::{self, Write},
};

fn main() -> Result<(), ParseError> {
    let mut args = env::args().skip(1);

    if args.len() < 6 {
        println!(
            "Invalid command, example:  --input <file_name> --input-format <bin|csv|txt> --output-format <bin|csv|txt>"
        )
    }

    let mut file_path = String::new();
    let mut format_input_file = String::new();
    let mut format_output_file = String::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => {
                if let Some(v) = args.next() {
                    file_path = v;
                }
            }
            "--input-format" => {
                if let Some(v) = args.next() {
                    format_input_file = v;
                }
            }
            "--output-format" => {
                if let Some(v) = args.next() {
                    format_output_file = v;
                }
            }
            _ => panic!("command '{}' not found", arg),
        }
    }

    if format_input_file.is_empty() {
        panic!("Format input file not specified");
    }

    if format_output_file.is_empty() {
        panic!("Format ouptut file not specified");
    }

    if file_path.is_empty() {
        panic!("Input file not specified");
    }

    if let Err(_er) = Extension::str_to_extension(&format_input_file) {
        panic!("Format input file no found");
    }

    if let Err(_er) = Extension::str_to_extension(&format_output_file) {
        panic!("Format output file no found");
    }

    let records = parser::read_file(&format_input_file, &file_path)?;
    let file_name = parser::write_file(&format_output_file, &records)?;
    println!("> {}", file_name);
    io::stdout().flush()?;

    Ok(())
}
