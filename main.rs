use csv::ReaderBuilder;
// use ndarray::Array2;
// use ndarray_stats::QuantileExt;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
//, BufWriter, Write};
// use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the cosine similarity scores from a CSV file
    let data = "cosine_similarities.csv";
    let file = File::open(data)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));
    let mut items: Vec<String> = Vec::new();
    let mut scores: Vec<f64> = Vec::new();
    for result in reader.records() {
        let record = result?;
        let item = record.get(0).unwrap().to_owned();
        let score: f64 = record.get(1).unwrap().parse()?;
        items.push(item);
        scores.push(score);
    }

    // Find the indices of the top 3 items with the largest cosine similarity score
    let mut top_indices: Vec<usize> = Vec::new();
    let mut top_scores: Vec<f64> = Vec::new();
    for _ in 0..4 {
        let max_score = scores.iter().cloned().fold(f64::NAN, f64::max);
        let max_index = scores.iter().position(|&x| x == max_score).unwrap();
        top_indices.push(max_index);
        top_scores.push(max_score);
        scores[max_index] = f64::NAN;
    }

    // Print the top 3 items with the largest cosine similarity score
    println!("Top 3 items with the largest cosine similarity score:");
    println!("The original item is: {}", items[0]);
    for i in 1..4 {
        println!("{}: {}", items[top_indices[i]], top_scores[i]);
    }

    Ok(())
}
