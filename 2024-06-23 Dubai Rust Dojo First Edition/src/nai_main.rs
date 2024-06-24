use std::error::Error;
use csv::Reader;
use std::time::Instant;

fn find_max_value() -> Result<f64, Box<dyn Error>> {
    let mut reader = Reader::from_path("src/NVDA.csv")?;
    let headers = reader.headers()?.clone();
    
    let column_index = headers.iter().position(|h| h == "Adj Close")
        .ok_or("Column not found")?;

    let max_value = reader.records()
        .filter_map(Result::ok)
        .filter_map(|record| record.get(column_index).and_then(|s| s.parse::<f64>().ok()))
        .fold(f64::NEG_INFINITY, f64::max);

    Ok(max_value)
}

fn main() -> Result<(), Box<dyn Error>> {
    const NUM_RUNS: u32 = 100;
    
    let start = Instant::now();
    
    for _ in 0..NUM_RUNS {
        let max_value = find_max_value()?;
        // Uncomment the next line if you want to see each result
        // println!("Max value: {}", max_value);
    }
    
    let duration = start.elapsed();
    
    println!("Average time per run: {:?}", duration / NUM_RUNS);
    println!("Total time for {} runs: {:?}", NUM_RUNS, duration);

    Ok(())
}