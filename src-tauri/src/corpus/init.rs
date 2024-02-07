use memmap::Mmap;
use std::collections::HashMap;
use std::f64;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;

pub struct Database {
    corpus: HashMap<String, HashMap<String, u32>>,
    idf: HashMap<String, f64>,
}

impl Database {
    pub fn new(init_path: &str) -> Self {
        let corpus = init_corpus(&init_path);
        let idf = calculate_idf(&corpus);
        Self { corpus, idf }
    }

    pub fn search(&self, query: &[String]) -> HashMap<String, f64> {
        calculate_bm25(query, &self.idf, &self.corpus)
    }
}

fn init_corpus(init_path: &str) -> HashMap<String, HashMap<String, u32>> {
    let path: &str = init_path;
    let mut corpus: HashMap<String, HashMap<String, u32>> = HashMap::new();

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() && is_text_file(path) {
            println!("Processing file: {}", path.display());

            let file = File::open(path).unwrap();
            let content = unsafe { Mmap::map(&file).unwrap() };
            let content_str = match std::str::from_utf8(&content) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Invalid UTF-8 sequence: {}", e);
                    continue;
                }
            };
            let words = process_text(content_str);
            let file_path = path.to_string_lossy().to_string();
            let word_entry = corpus.entry(file_path).or_default();

            for word in words {
                *word_entry.entry(word.clone()).or_default() += 1;
                //println!("Added word '{}' from file '{}' to corpus", word, file_path);
            }
        }
    }

    corpus
}

fn is_text_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => {
            matches!(ext.to_str(), Some(ext) if ["txt", "rs", "cs", "js", "html", "css", "json", "xml", "md", "py", "java", "c", "cpp", "h", "hpp"].contains(&ext))
        }
        None => false,
    }
}

fn process_text(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| word.to_lowercase())
        .collect()
}

fn calculate_idf(corpus: &HashMap<String, HashMap<String, u32>>) -> HashMap<String, f64> {
    let total_documents = corpus.len() as f64;
    let mut idf: HashMap<String, f64> = HashMap::new();

    for doc_map in corpus.values() {
        for term in doc_map.keys() {
            let term_freq = corpus
                .iter()
                .filter(|(_, doc_map)| doc_map.contains_key(term))
                .count() as f64;
            let idf_value = f64::ln((total_documents + 1.0) / (term_freq + 1.0));
            idf.insert(term.clone(), idf_value);
        }
    }

    idf
}
fn calculate_bm25(
    query: &[String],
    idf: &HashMap<String, f64>,
    corpus: &HashMap<String, HashMap<String, u32>>,
) -> HashMap<String, f64> {
    println!("Query: {:?}, Length: {}", query, query.len());

    let k1 = 1.2;
    let b = 0.75;
    let avgdl = corpus // Average document length
        .values()
        .map(|doc| doc.values().sum::<u32>() as f64)
        .sum::<f64>()
        / corpus.len() as f64;

    let mut scores = HashMap::new();

    for term in query {
        if let Some(idf_value) = idf.get(term) {
            println!("Term: {}, IDF Value: {}", term, idf_value);

            let mut term_found = false;

            for (doc, doc_map) in corpus {
                if let Some(freq) = doc_map.get(term) {
                    term_found = true;

                    let len_d = doc_map.values().sum::<u32>() as f64;
                    let f = *freq as f64;
                    let numerator = f * (k1 + 1.0);
                    let denominator = f + k1 * (1.0 - b + b * len_d / avgdl);
                    let score = idf_value * numerator / denominator;

                    println!(
                        "Document: {}, Frequency: {}, Document Length: {}, Score: {}",
                        doc, freq, len_d, score
                    );
                    println!(
                        "Numerator: {}, Denominator: {}, Fraction: {}",
                        numerator,
                        denominator,
                        numerator / denominator
                    );

                    *scores.entry(doc.clone()).or_insert(0.0) += score;
                }
            }

            if !term_found {
                println!("Term '{}' not found in any document in the corpus", term);
            }
        } else {
            println!("Term '{}' not found in idf", term);
        }
    }

    scores
}
