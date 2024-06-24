# 2024-06-23 Dubai Rust Dojo First Edition

## Event Description

Here lies all the code and resources from the first edition of the Dubai Rust Dojo, held on June 23, 2024. The kata focused on parsing NVIDIA stock historical data from Yahoo! Finance to find the maximum price and corresponding date.

## Kata Description

The main task for this Dojo was to:

1. Parse the NVIDIA stock historical data from Yahoo! Finance
2. Determine the maximum price
3. Identify the date of the maximum price

Data source: [NVIDIA Historical Data on Yahoo! Finance](https://finance.yahoo.com/quote/NVDA/history/?period1=917015400&period2=1719075041)

## Repository Contents

This repository contains multiple files, each representing a different approach to solving the Kata. The various implementations aim to explore:

- Code readability
- Execution speed
- Different levels of complexity (including some intentionally over-engineered solutions)

## Slides

The slides from the event can be found [here](https://docs.google.com/presentation/d/1yBIBWuUYq0jVPTNpp63pSjkstt5KTBIA44gRoaGTwX8/edit?usp=sharing).

## Approaches

Each file in this repository represents a different approach to solving the Kata. Participants were encouraged to consider:

1. How readable is the code for each approach?
2. How fast does each approach run?
3. Ways to "overthink" the Kata and create more complex solutions. You can use Python to run "create_test_csv.py" to create a 5GB Dataset (large_file50m.csv) to test against.

## Presentations

Participants prepared short (5-minute) presentations highlighting their takeaways from the different approaches.

## Getting Started

To run these examples:

1. Clone this repository
2. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system
3. Navigate to the project directory
4. Run individual files using `cargo run --bin <filename>`

## Contributing

If you attended the Dojo and would like to add your solution, please submit a pull request.

## Acknowledgments

Thanks to all participants and @datapythonista for organizing the First Dubai Rust Dojo!
