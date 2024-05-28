use rusqlite::{Connection, types::ValueRef};
use std::fs::File;
use csv::Writer;
use anyhow::{Result};

/// Executes a SQL query on a provided SQLite connection, writing the results to a CSV file.
///
/// # Arguments
/// * `conn` - A reference to the SQLite connection to execute the query on.
/// * `query` - The SQL query string to execute.
/// * `output_csv` - The file path where the CSV output should be written.
///
/// # Errors
/// Returns an error if the query cannot be prepared, the CSV file cannot be created,
/// or there is an error in fetching data or writing to the file.
///
/// # Examples
/// ```
/// use rusqlite::{Connection, OpenFlags};
/// use tempfile::tempdir;
/// use std::fs;
///
/// // Create a temporary directory to store the database file and CSV.
/// let dir = tempdir().unwrap();
/// let db_path = dir.path().join("temp.db");
/// let csv_path = dir.path().join("output.csv");
///
/// // Connect to a new in-memory database.
/// let conn = Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();
///
/// // Execute a query to create a table and populate it.
/// conn.execute("CREATE TABLE people (name TEXT, age INTEGER)", []).unwrap();
/// conn.execute("INSERT INTO people (name, age) VALUES ('Alice', 30), ('Bob', 25)", []).unwrap();
///
/// // Call the function with the connection, a query, and a path to the output CSV.
/// execute_query(&conn, "SELECT * FROM people", csv_path.to_str().unwrap()).unwrap();
///
/// // Read and print the contents of the CSV file.
/// let contents = fs::read_to_string(csv_path).unwrap();
/// println!("{}", contents);
/// ```

pub fn execute_query(conn: &Connection, query: &str, output_csv: &str) -> Result<()> {
    let mut stmt = conn.prepare(query)?;

    let columns = stmt.column_names();
    let column_names: Vec<String> = columns.iter().map(|name| name.to_string()).collect();

    let mut rows = stmt.query([])?;

    let file = File::create(output_csv)?;
    let mut wtr = Writer::from_writer(file);

    // Write column names as the header of the CSV.
    wtr.write_record(&column_names)?;

    // Iterate through each row to process and write its data.
    while let Ok(Some(row)) = rows.next() {
        let mut csv_row = Vec::new();
        for (i, _) in column_names.iter().enumerate() {
            let value = match row.get_ref_unwrap(i) {
                ValueRef::Null => "".to_string(),
                ValueRef::Integer(int_val) => int_val.to_string(),
                ValueRef::Real(flt_val) => flt_val.to_string(),
                ValueRef::Text(text_val) => String::from_utf8_lossy(text_val).to_string(),
                ValueRef::Blob(blob_val) => String::from_utf8_lossy(blob_val).to_string(),
            };
            csv_row.push(value);
        }
        wtr.write_record(&csv_row)?;
    }

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, OpenFlags};
    use tempfile::tempdir;

    #[test]
    fn test_execute_query() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let csv_path = dir.path().join("output.csv");

        let conn = Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).unwrap();
        conn.execute("CREATE TABLE test (id INTEGER, name TEXT)", []).unwrap();
        conn.execute("INSERT INTO test (id, name) VALUES (1, 'Test'), (2, 'Rust')", []).unwrap();

        execute_query(&conn, "SELECT * FROM test", csv_path.to_str().unwrap()).unwrap();

        let contents = std::fs::read_to_string(csv_path).unwrap();
        assert!(contents.contains("Test"));
        assert!(contents.contains("Rust"));
    }
}