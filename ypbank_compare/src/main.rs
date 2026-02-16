use std::io::{self, Write};

use parser::{error::ParseError};


fn main() -> Result<(), ParseError> {

    let path_file : &str = "file_example/records_example.bin";

    let records = parser::read_file("bin", path_file)?;

    // println!("{:#?}", records);

    // let file_name = parser::write_file("csv", &records)?;
    let file_name = parser::write_file("bin", &records)?;
    // let file_name = parser::write_file("txt", &records)?;

    println!("> {}", file_name);

    io::stdout().flush()?;

    Ok(())

}