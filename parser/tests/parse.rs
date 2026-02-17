#[cfg(test)]
mod tests {
    
    use std::fs;

    use parser::{read_file, write_file};


    #[test]
    fn csv_parse() {
        
        let data = "../file_example/records_example.csv";
        
        let rs = read_file("csv", data).expect("parse error");

        assert_eq!(rs.len(), 1000);
        assert_eq!(rs[0].amount, 100)
    }

    #[test]
    fn bin_parse() {
        
        let data = "../file_example/records_example.bin";
        
        let rs = read_file("bin", data).expect("parse error");

        assert_eq!(rs.len(), 1000);
        assert_eq!(rs[0].amount, 100)
    }

    #[test]
    fn txt_parse() {
        
        let data = "../file_example/records_example.txt";
        
        let rs = read_file("txt", data).expect("parse error");

        assert_eq!(rs.len(), 1000);
        assert_eq!(rs[0].amount, 100)
    }

    #[test]
    fn txt_to_csv() {
        
        let txt = "../file_example/records_example.txt";
        let rs = read_file("txt", txt).expect("parse error");
        
        let file_output = write_file("csv", &rs).expect("parse error");

        
        let rs1 = read_file("csv", &format!("{}", file_output)).expect("parse error");        

        assert_eq!(rs1.len(), 1000);
        assert_eq!(rs1[0].amount, 100);

        fs::remove_file(file_output).expect("delete error");
    }
}
