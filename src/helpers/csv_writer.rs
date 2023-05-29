use csv::Writer;
use std::fs::OpenOptions;

fn main() -> Result<(), csv::Error> {
    // Open the CSV file in append mode
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("data.csv")?;

    // Create a CSV writer
    let mut writer = Writer::from_writer(file);

    // Create a new record
    let record = vec!["John Doe", "john.doe@example.com", "555-1234"];

    // Write the record to the CSV file
    writer.serialize(record)?;

    // Flush the writer to ensure all data is written to the file
    writer.flush()?;

    Ok(())
}

pub fn csv_output(object: BagObject){
    todo
}