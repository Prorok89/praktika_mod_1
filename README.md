# praktika_mod_1

Проект содержит workspace с тремя крейтами:

- `parser` — библиотека парсинга/сериализации форматов YPBankCSV/YPBankTXT/YPBankBIN.
- `ypbank_converter` — CLI для конвертации между форматами.
- `ypbank_compare` — CLI для сравнения двух файлов с транзакциями.

## Примеры запуска

Конвертация:

```
cargo run -p ypbank_converter -- --input file_example/records_example.csv --input-format csv --output-format bin
```

Сравнение:

```
cargo run -p ypbank_compare -- --file1 file_example/records_example.bin --format1 bin --file2 file_example/records_example.csv --format2 csv
```
