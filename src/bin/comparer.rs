use parser::error::ParseError;


fn main() -> Result<(), ParseError> {

    let path_file : &str = "file_example/records_example.csv";

    _ = parser::read_file(path_file)?;

    Ok(())

}