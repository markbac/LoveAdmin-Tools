use csv::Writer;
use std::fs::File;
use rusqlite::{Connection, Error as SqliteError, Result as SqliteResult};


pub fn execute_query(conn: &Connection, query: &str, output_csv: &str) -> Result<()> {
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query([])?; 

    // Create a new CSV writer that writes to a file
    let file = File::create(output_csv)
        .map_err(|e| SqliteError::Other(Box::new(e)))?;  
    let mut wtr = Writer::from_writer(file);

    // Assuming we might not know the column names ahead of time, fetch and write headers
    //let file = File::create(output_csv)
    //    .map_err(|e| SqliteError::Other(Box::new(e)))?;  
    //wtr.write_record(columns)?;

    // Iterate through the SQLite rows
    while let Some(row) = rows.next()? {
        let mut csv_row = vec![];
        for i in 0..row.column_count() {
            // Safely handle potential NULL values
            let value: Option<String> = row.get(i)?;
            csv_row.push(value.unwrap_or_default());
        }
        // Write the row to the CSV
        wtr.write_record(csv_row)?;
    }

    // Flush the writer to ensure all data is written to the file
    wtr.flush()?;
    Ok(())
}
