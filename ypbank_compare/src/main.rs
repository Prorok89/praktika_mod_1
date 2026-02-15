use std::io::{self, Write};

use parser::{error::ParseError};


fn main() -> Result<(), ParseError> {

    let path_file : &str = "../file_example/records_example.csv";

    let records = parser::read_file("txt", path_file)?;

    // let file_name = parser::write_file("csv".to_string(), &records)?;
    let file_name = parser::write_file("txt", &records)?;

    println!("> {}", file_name);

    io::stdout().flush()?;

    Ok(())

}