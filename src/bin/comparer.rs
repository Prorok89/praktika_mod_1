use std::io::Read;

use parser::yp_bank_csv::YPBankCSV;


fn main() {

    let path_file : &str = "file_example/records_example.csv";

    let res_check = parser::check_file(path_file).unwrap_or(false);

    // let csv = YPBankCSV::new(path_file.to_string().clone());

    // csv.read(buf);


    println!("1 - {path_file}");

}