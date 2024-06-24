use std::{error::Error, process, fs::File};

fn find_highest_price(filename: &str) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut current_highest_price = 0.0;
    let mut current_date_with_highest_price = String::new();
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let record_price = record.get(5).unwrap().parse::<f32>().unwrap();
        if record_price > current_highest_price {
            current_highest_price = record_price;
            current_date_with_highest_price = record.get(0).unwrap().to_string();
        }
    }
    println!("{:?}", current_date_with_highest_price);
    Ok(())
}

fn main() {
    if let Err(err) = find_highest_price("src/NVDA.csv") {
        println!("error running example: {}", err);
        process::exit(1);
    }
}