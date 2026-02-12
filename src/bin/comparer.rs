use std::io::Read;
 

use parser::{error::ParseError, yp_bank_csv::YPBankCSV};


fn main() -> Result<(), ParseError>{

    let path_file : &str = "file_example/records_example.csv";

    let res_check = parser::check_file(path_file)?;

    // let csv = YPBankCSV::new(path_file.to_string().clone());

    // csv.read(buf);


    println!("1 - {path_file}");

    Ok(())

}