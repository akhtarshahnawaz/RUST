use std::error::Error;
use std::io;

use csv;

/// Reads data from `stdin` into a reader and prints all records.
/// If an error occurs, the error is returned to `main`.
fn read_from_stdin() -> Result<(), Box<dyn Error>> {
    // Creates a new csv `Reader` from `stdin`
    let mut reader = csv::Reader::from_reader(io::stdin());

    // `.headers` return header of the csv
    let headers = reader.headers()?;
    println!("Headers: {:?}", headers);

    // `.records` return an iterator of the internal
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    // If an error occurs print error
    if let Err(e) = read_from_stdin() {
        eprintln!("{}", e);
    }
}