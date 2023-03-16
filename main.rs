use csv::ReaderBuilder;
use ndarray_stats::QuantileExt;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use ndarray::{Array2, Axis};
use ndarray_linalg::Norm;
use rust_tokenizers::adapters::Example;
use rust_tokenizers::tokenizer::{BertTokenizer, Tokenizer, TruncationStrategy};
use rust_tokenizers::vocab::{BertVocab, Vocab};

fn main() -> Result<(), Box<dyn Error>> {
    // Read the data from a CSV file
    let data = "Coursera_cleaned.csv";
    let file = File::open(data)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));
    let mut rows: Vec<(String, String)> = Vec::new();
    for result in reader.records() {
        let record = result?;
        let course_name = record.get(0).unwrap().to_owned();
        let course_info = record.get(1).unwrap().to_owned();
        rows.push((course_name, course_info));
    }

    // Compute the TF-IDF vectors
    let (tfidf, terms) = compute_tfidf(&rows);

    // Compute the cosine similarity matrix
    let similarity_matrix = compute_similarity(&tfidf);

    // Get recommendations based on a given course name
    let course_name = "Finance for Managers";
    let n_recommendations = 10;
    let recommended_courses = get_recommendations(&rows, &similarity_matrix, &terms, course_name, n_recommendations);

    // Print the recommendations
    println!("Recommendations for {}: ", course_name);
    for course in recommended_courses {
        println!("{}", course);
    }

    // Save the data into a text file
    let path = Path::new("data.txt");
    let file = File::create(&path)?;
    let mut writer = BufWriter::new(file);
    for (_, info) in &rows {
        writeln!(writer, "{}", info)?;
    }

    Ok(())
}

fn compute_tfidf(rows: &[(String, String)]) -> (Array2<f64>, Vec<String>) {
    let mut corpus: Vec<String> = Vec::new();
    for (_, info) in rows {
        corpus.push(info.clone());
    }

    // Load the BERT vocabulary and tokenizer
    let vocab = BertVocab::from_file("bert-base-uncased-vocab.txt").unwrap();
    let tokenizer = BertTokenizer::from_existing_vocab(vocab, true, true);

    // Tokenize the text
    let tokenized: Vec<Vec<String>> = corpus
        .iter()
        .map(|x| tokenizer.encode(x, None, 128, &TruncationStrategy::LongestFirst, 0))
        .map(|example| example.tokenized_input.iter().map(|t| t.to_owned()).collect())
        .collect();

    // Compute the document frequency for each term
    let mut df: Vec<u32> = vec![0; tokenized[0].len()];
    for tokens in &tokenized {
        for (i, term) in tokens.iter().enumerate() {
            if df[i] == 0 {
                df[i] = tokenized.iter().filter(|x| x.contains(term)).count() as u32;
            }
        }
    }

    // Compute the inverse document frequency for each term
    let idf = df.iter().map(|x| (rows.len() as f64 / (*x as f64)).ln()).collect::<Vec<f64>>();

    // Compute the TF-IDF scores
    let mut tfidf_scores = Array2::zeros((rows.len(), df.len()));
    for (i, tokens) in tokenized.iter().enumerate() {
        let tf = tokens.iter().fold(vec![0; df.len()], |mut acc, term| {
            if let Some(idx) = acc.iter().position(|&x| x == 0) {
                acc[idx] = tokens.iter().filter(|&x| x == term).count() as u32;
            }
            acc
        });
        let tfidf = tf.iter().enumerate().map(|(j, &x)| x as f64 * idf[j]).collect::<Vec<f64>>();
        tfidf_scores.row_mut(i).assign(&Array2::from_shape_vec((1, df.len()), tfidf).unwrap().row(0));

// Tokenize the text
let mut tokenizer = SimpleTokenizer::default();
let tokenized: Vec<Vec<String>> = corpus.iter().map(|x| tokenizer.tokenize(x)).collect();

// Load the BERT vocabulary and tokenizer
let vocab = BertVocab::from_file("bert-base-uncased-vocab.txt")?;
let tokenizer = BertTokenizer::from_existing_vocab(vocab, true, true);

// Tokenize the text
let tokenized: Vec<Vec<String>> = corpus
    .iter()
    .map(|x| tokenizer.encode(x, None, 128, &TruncationStrategy::LongestFirst, 0))
    .map(|example| example.tokenized_input.iter().map(|t| t.to_owned()).collect())
    .collect();
    }
}

fn compute_similarity(tfidf: &Array2<f64>) -> Array2<f64> {
    let mut similarity_matrix = Array2::zeros((tfidf.nrows(), tfidf.nrows()));
    for i in 0..tfidf.nrows() {
        for j in 0..tfidf.nrows() {
            let a = tfidf.row(i);
            let b = tfidf.row(j);
            let similarity = a.dot(&b) / (a.norm() * b.norm());
            similarity_matrix[[i, j]] = similarity;
        }
    }
    similarity_matrix
}
fn get_recommendations(
    rows: &[(String, String)],
    similarity_matrix: &Array2<f64>,
    terms: &Vec<String>,
    course_name: &str,
    n_recommendations: usize,
) -> Vec<String> {
    let course_idx = rows.iter().position(|x| x.0 == course_name).unwrap();
    let mut similarities = similarity_matrix.row(course_idx).to_owned();
    similarities[[course_idx]] = 0.0;
    let mut indices = similarities
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect::<Vec<(f64, usize)>>();
    indices.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut recommended_courses = Vec::new();
    for i in 0..n_recommendations {
        let idx = indices[i].1;
        recommended_courses.push(rows[idx].0.clone());
    }
    recommended_courses
}






