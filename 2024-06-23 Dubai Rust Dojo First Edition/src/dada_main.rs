use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::prelude::{CsvReadOptions, SessionContext};

// fn main1() {
//     let file = File::open("src/NVDA.csv").expect("no such file");
//     let buf = BufReader::new(file);
//     let mut highest_price: f64 = 0f64;
//     let mut row: Vec<String> = vec![];
//     for line in buf.lines() {
//         let line = line.unwrap();
//         let string = line.clone();
//         let mut entries = string.split(",").map(|s|s.to_string()).collect::<Vec<_>>();
//         if entries.get(2) == Some(&"High".to_string()) {
//             continue;
//         }

//         let high = entries.get(2).unwrap();
//         let high: f64 = high.parse::<f64>().unwrap();
//         if high > highest_price {
//             highest_price = high;
//             row = entries;
//         }
//     }
//     dbg!(row);
// }

// fn main2() {
//     let file = File::open("/Users/dadepoaderemi/Documents/fun/dojo/src/NVDA.csv").expect("no such file");
//     let buf = BufReader::new(file);
//     let mut highest_price: f64 = 0f64;
//     let mut row: Vec<String> = vec![];
//     let mut init: Vec<String> = vec![];
//     buf.lines().skip(1usize).fold(vec![], |acc, entry| {
//         // if entry.unwrap().get(0) > acc.get(0).unwrap().
//         todo!()
//     });

//     dbg!("".to_string());

// }


#[tokio::main]
async fn main() {
    let ctx = SessionContext::new();
    // Date,Open,High,Low,Close,Adj Close,Volume
    let schema = Arc::new(Schema::new(vec![
        Field::new("date", DataType::Date64, false),
        Field::new("open", DataType::Float64, false),
        Field::new("high", DataType::Float64, false),
        Field::new("low", DataType::Float64, false),
        Field::new("close", DataType::Float64, false),
        Field::new("volume", DataType::Int64, true),
    ]));

    // let df = ctx
    //     .read_csv(
    //         "/Users/dadepoaderemi/Documents/fun/dojo/src/NVDA.csv",
    //         CsvReadOptions::default().schema(&schema),
    //     )
    //     .await.unwrap();

    ctx
        .register_csv(
            "data",
            "src/large_file50m.csv",
            CsvReadOptions::default().schema(&schema),
        )
        .await.unwrap();

    let df = ctx.sql("SELECT * FROM data ORDER by high DESC LIMIT 1").await.unwrap();
    df.show().await.unwrap();

}
