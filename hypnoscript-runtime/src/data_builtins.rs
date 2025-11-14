use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use thiserror::Error;

/// Options for querying JSON structures.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JsonQueryOptions {
    /// Dot/array notation, e.g. `data.items[0].name`.
    pub path: String,
    /// Optional default value returned if the path is missing.
    pub default_value: Option<String>,
}

/// Options applied when parsing/writing CSV data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsvOptions {
    pub delimiter: char,
    pub has_header: bool,
}

impl Default for CsvOptions {
    fn default() -> Self {
        Self {
            delimiter: ',',
            has_header: true,
        }
    }
}

/// Result of parsing CSV content.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct CsvParseResult {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Error)]
pub enum DataError {
    #[error("JSON-Fehler: {0}")]
    Json(String),
    #[error("CSV-Fehler: {0}")]
    Csv(String),
    #[error("I/O-Fehler: {0}")]
    Io(String),
}

impl From<serde_json::Error> for DataError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value.to_string())
    }
}

impl From<csv::Error> for DataError {
    fn from(value: csv::Error) -> Self {
        Self::Csv(value.to_string())
    }
}

impl From<std::io::Error> for DataError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

/// Builtins for data-centric workloads (JSON/CSV utilities).
pub struct DataBuiltins;

impl DataBuiltins {
    /// Returns a pretty-printed JSON string.
    pub fn json_pretty(raw: &str) -> Result<String, DataError> {
        let value: JsonValue = serde_json::from_str(raw)?;
        Ok(serde_json::to_string_pretty(&value)?)
    }

    /// Queries a JSON document for the provided path.
    pub fn json_query(raw: &str, options: &JsonQueryOptions) -> Result<Option<String>, DataError> {
        let value: JsonValue = serde_json::from_str(raw)?;
        if options.path.trim().is_empty() {
            return Ok(Some(value.to_string()));
        }
        Ok(json_path(&value, &options.path)
            .map(|v| stringify_json_value(v))
            .or_else(|| options.default_value.clone()))
    }

    /// Merges two JSON documents (second overrides the first).
    pub fn json_merge(primary: &str, secondary: &str) -> Result<String, DataError> {
        let mut left: JsonValue = serde_json::from_str(primary)?;
        let right: JsonValue = serde_json::from_str(secondary)?;
        merge_values(&mut left, &right);
        Ok(left.to_string())
    }

    /// Loads CSV text into a structured representation.
    pub fn parse_csv(raw: &str, options: CsvOptions) -> Result<CsvParseResult, DataError> {
        let mut reader = ReaderBuilder::new()
            .delimiter(options.delimiter as u8)
            .has_headers(options.has_header)
            .from_reader(raw.as_bytes());

        let headers = if options.has_header {
            reader
                .headers()
                .map(|h| h.iter().map(|s| s.to_string()).collect())?
        } else {
            Vec::new()
        };

        let mut rows = Vec::new();
        for record in reader.records() {
            let record = record?;
            rows.push(record.iter().map(|cell| cell.to_string()).collect());
        }

        Ok(CsvParseResult { headers, rows })
    }

    /// Builds CSV text from headers + rows.
    pub fn to_csv(table: &CsvParseResult, options: CsvOptions) -> Result<String, DataError> {
        let mut output = Vec::new();
        {
            let mut writer = WriterBuilder::new()
                .delimiter(options.delimiter as u8)
                .has_headers(false)
                .from_writer(&mut output);

            if options.has_header && !table.headers.is_empty() {
                writer.write_record(&table.headers)?;
            }
            for row in &table.rows {
                writer.write_record(row)?;
            }
            writer.flush()?;
        }

        Ok(String::from_utf8_lossy(&output).to_string())
    }

    /// Selects a subset of columns (by header name) from a CSV text.
    pub fn csv_select_columns(
        raw: &str,
        columns: &[String],
        options: CsvOptions,
    ) -> Result<CsvParseResult, DataError> {
        let table = Self::parse_csv(raw, options.clone())?;
        if table.headers.is_empty() {
            return Ok(CsvParseResult {
                headers: Vec::new(),
                rows: table.rows,
            });
        }

        let mut indices = Vec::new();
        for column in columns {
            if let Some(index) = table.headers.iter().position(|h| h == column) {
                indices.push(index);
            }
        }

        let mut projected_rows = Vec::new();
        for row in table.rows {
            let projected: Vec<String> = indices
                .iter()
                .filter_map(|&idx| row.get(idx).cloned())
                .collect();
            projected_rows.push(projected);
        }

        let projected_headers = indices
            .iter()
            .filter_map(|&idx| table.headers.get(idx).cloned())
            .collect();

        Ok(CsvParseResult {
            headers: projected_headers,
            rows: projected_rows,
        })
    }
}

fn stringify_json_value(value: &JsonValue) -> String {
    match value {
        JsonValue::String(s) => s.clone(),
        JsonValue::Number(num) => num.to_string(),
        JsonValue::Bool(b) => b.to_string(),
        JsonValue::Null => "null".to_string(),
        _ => value.to_string(),
    }
}

fn json_path<'a>(value: &'a JsonValue, path: &str) -> Option<&'a JsonValue> {
    let mut current = value;
    let mut token = String::new();
    let mut chars = path.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '.' => {
                if !token.is_empty() {
                    current = current.get(&token)?;
                    token.clear();
                }
            }
            '[' => {
                if !token.is_empty() {
                    current = current.get(&token)?;
                    token.clear();
                }
                let mut index = String::new();
                while let Some(&c) = chars.peek() {
                    chars.next();
                    if c == ']' {
                        break;
                    }
                    index.push(c);
                }
                let idx: usize = index.parse().ok()?;
                current = current.get(idx)?;
            }
            _ => token.push(ch),
        }
    }

    if !token.is_empty() {
        current = current.get(&token)?;
    }
    Some(current)
}

fn merge_values(left: &mut JsonValue, right: &JsonValue) {
    match (left, right) {
        (JsonValue::Object(left_map), JsonValue::Object(right_map)) => {
            for (key, value) in right_map {
                merge_values(
                    left_map.entry(key.clone()).or_insert(JsonValue::Null),
                    value,
                );
            }
        }
        (JsonValue::Array(left_arr), JsonValue::Array(right_arr)) => {
            left_arr.extend(right_arr.clone());
        }
        (left_slot, right_value) => {
            *left_slot = right_value.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_query_extracts_nested_value() {
        let json = r#"{"user":{"profile":{"name":"Hypno"}}}"#;
        let options = JsonQueryOptions {
            path: "user.profile.name".to_string(),
            default_value: None,
        };
        let value = DataBuiltins::json_query(json, &options).unwrap();
        assert_eq!(value, Some("Hypno".to_string()));
    }

    #[test]
    fn csv_parse_and_to_csv_roundtrip() {
        let csv_text = "name,age\nAda,32\nBob,30";
        let options = CsvOptions::default();
        let table = DataBuiltins::parse_csv(csv_text, options.clone()).unwrap();
        assert_eq!(table.headers, vec!["name", "age"]);
        assert_eq!(table.rows.len(), 2);

        let serialized = DataBuiltins::to_csv(&table, options).unwrap();
        assert!(serialized.contains("Ada"));
    }
}
