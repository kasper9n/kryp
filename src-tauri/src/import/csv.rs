use crate::{err, throw};
use csv::{Reader, StringRecord, StringRecordsIntoIter};
use std::fs::File;
use std::io;
use std::iter::Peekable;
use std::path::PathBuf;

pub fn read_csv(file_path: PathBuf) -> Result<Reader<File>, String> {
	let delimiter = match file_path.extension().unwrap_or_default().to_str() {
		Some("csv") => b',',
		Some("tsv") => b'\t',
		_ => throw!("Unknown file extension"),
	};
	let reader = csv::ReaderBuilder::new()
		.delimiter(delimiter)
		.has_headers(true)
		.from_path(file_path)
		.map_err(|_| "Error opening file".to_string())?;
	Ok(reader)
}

pub fn lowercase_header_contains<R: io::Read>(reader: &mut Reader<R>, s: &str) -> bool {
	match reader.headers() {
		Ok(record) => {
			for cell in record.iter() {
				if cell.to_lowercase() == s.to_lowercase() {
					return true;
				}
			}
		}
		Err(_) => {}
	}
	false
}
pub fn get_header_lowercase<R: io::Read>(reader: &mut Reader<R>) -> Result<StringRecord, String> {
	let header = match reader.headers() {
		Ok(header) => header,
		Err(e) => throw!("Unable to read headers: {}", e),
	};
	Ok(header.iter().map(|s| s.to_lowercase()).collect())
}

pub struct CsvIter {
	row_index: usize,
	records: Peekable<StringRecordsIntoIter<File>>,
}
impl Iterator for CsvIter {
	type Item = Result<StringRecord, String>;
	fn next(&mut self) -> Option<Result<StringRecord, String>> {
		self.row_index += 1;
		match self.records.next()? {
			Ok(record) => Some(Ok(record)),
			Err(e) => Some(err!("Unable to read row {}: {}", self.row_index, e)),
		}
	}
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
