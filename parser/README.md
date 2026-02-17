# praktika_mod_1

Библиотека для парсинга и сериализации форматов YPBankCSV, YPBankTXT, YPBankBIN.

## Использование

```rust
    use parser::{self, error::ParseError};

    let data = "records_example.txt";
    let records = parser::read_file("txt", data)?;
    
    let file_output = parser::write_file("csv", &records)?;    
```