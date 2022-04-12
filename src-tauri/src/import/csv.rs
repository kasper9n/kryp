use crate::throw;
use csv::{Reader, StringRecord};
use std::fs::File;
use std::iter::Enumerate;
use std::path::PathBuf;

pub fn read_csv(file_path: PathBuf) -> Result<Reader<File>, String> {
  let delimiter = match file_path.extension().unwrap_or_default().to_str() {
    Some("csv") => b',',
    Some("tsv") => b'\t',
    _ => throw!("Unknown file extension"),
  };
  let reader = csv::ReaderBuilder::new()
    .delimiter(delimiter)
    .from_path(file_path)
    .map_err(|_| "Error opening file".to_string())?;
  Ok(reader)
}

pub fn read_csv_header(
  records: &mut Enumerate<impl Iterator<Item = Result<StringRecord, String>>>,
) -> Result<StringRecord, String> {
  match records.next() {
    Some((_, Ok(header))) => Ok(header),
    Some((_, Err(e))) => throw!("Unable to read headers: {}", e),
    None => throw!("No headers found"),
  }
}

pub fn csv_rows<'a>(
  reader: &'a mut Reader<File>,
) -> Enumerate<impl Iterator<Item = Result<StringRecord, String>> + 'a> {
  let records = reader
    .records()
    .enumerate()
    .map(|(i, r)| r.map_err(|e| format!("Unable to read row {}: {}", i + 1, e)))
    .enumerate();
  records
}

pub fn get_cell_index(row: &Vec<String>, string: &[&str]) -> Result<usize, String> {
  match row.iter().position(|s| string.contains(&s.as_str())) {
    Some(i) => Ok(i),
    None => throw!("Missing column \"{}\"", string.get(0).unwrap_or(&"None")),
  }
}

pub fn get_cell<'a>(
  row: &'a StringRecord,
  col: Option<usize>,
  name: &str,
) -> Result<&'a str, String> {
  let i = col.ok_or(format!("Missing \"{}\" column", name))?;
  let cell = row.get(i).ok_or(format!("Missing \"{}\" cell", name))?;
  Ok(cell)
}
