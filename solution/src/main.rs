//! This program reads a file with weather station temperature measurements, aggregates min/mean/max
//! per station, and prints the results alphabetically.
//!
//! Basic solution: Feel free to edit and iterate this!

use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Holds statistics for a single weather station
#[derive(Debug, Default)]
struct Stats {
    /// Minimum temperature for the station
    min: f64,
    /// Maximum temperature for the station
    max: f64,
    /// Sum of all temperatures for mean calculation (for mean)
    sum: f64,
    /// Count of temperature readings for mean calculation
    count: usize,
}

impl Stats {
    /// Creates a new Stats instance with default values
    fn new(temp: f64) -> Self {
        Stats {
            min: temp,
            max: temp,
            sum: 0.0,
            count: 0,
        }
    }

    /// Updates the stats with a new temperature reading
    fn update(&mut self, temp: f64) {
        self.min = self.min.min(temp);
        self.max = self.max.max(temp);
        self.sum += temp;
        self.count += 1;
    }
}

fn main() -> io::Result<()> {
    // Get the input filename from command line arguments
    let filename = env::args()
        .nth(1)
        .expect("Please provide a path to an input file");
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Map from station name to its statistics, sorted alphabetically
    let mut stations: BTreeMap<String, Stats> = BTreeMap::new();

    // Process each line in the input file
    for line in reader.lines() {
        let line = line?; // Handle possible I/O errors

        // Parse the line into (station, temperature)
        let (station, temp) = parse_line(&line)
            .expect("Failed to parse line, expected format: StationName;Temperature");

        // Insert new station and update existing stats
        let entry = stations.entry(station).or_insert_with(|| Stats::new(temp));
        entry.update(temp);
    }

    // Print the results in the required format
    print_results(&stations);
    Ok(())
}

/// Parses a line like "StationName;12.3" into (station, temperature)
fn parse_line(line: &str) -> Option<(String, f64)> {
    let mut parts = line.splitn(2, ';');
    let station = parts.next()?.to_string();
    let temp = parts.next()?.parse::<f64>().ok()?;
    Some((station, temp))
}

/// Prints the statistics for all stations in the required format
fn print_results(stations: &BTreeMap<String, Stats>) {
    println!("{{");
    let mut first = true;
    for (station, stats) in stations {
        if !first {
            print!(",\n");
        }
        // Calculate mean temperature
        let mean = stats.sum / stats.count as f64;
        // Print: Station=min/mean/max with one decimal each
        print!(
            "    {}={:.1}/{:.1}/{:.1}",
            station, stats.min, mean, stats.max
        );
        first = false;
    }
    println!("\n}}");
}

#[cfg(test)]
mod tests {
    // You can add tests here
}
