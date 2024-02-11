use memmap::Mmap;
use rust_stemmers::{Algorithm, Stemmer};
use std::collections::{HashMap, HashSet};
use std::f64;
use std::fs::File;
use std::path::Path;
use strsim::levenshtein;
use walkdir::WalkDir;

pub struct Database {
    corpus: HashMap<String, HashMap<String, u32>>,
    idf: HashMap<String, f64>,
    stem_cache: HashMap<String, String>,
}

impl Database {
    pub fn new(init_path: &str) -> Self {
        let corpus = init_corpus(&init_path);
        let idf = calculate_idf(&corpus);
        let stem_cache = HashMap::new();
        Self {
            corpus,
            idf,
            stem_cache,
        }
    }

    pub fn search(
        &mut self,
        query: &[String],
    ) -> (HashMap<String, (f64, Option<String>)>, Vec<String>) {
        let mut new_query = Vec::new();
        let mut all_suggestions = Vec::new();

        for term in query {
            let stemmed_term: String = self.stem_word(term);
            if self.idf.contains_key(&stemmed_term) {
                new_query.push(stemmed_term);
            } else {
                let suggestions = self.suggest_words(&stemmed_term);
                let suggestions: Vec<String> = suggestions.into_iter().collect();
                all_suggestions.extend(suggestions.clone());
                new_query.extend(suggestions);
            }
        }

        let scores = calculate_bm25(&new_query[..], &self.idf, &self.corpus);

        // Generate snippets for each document
        let mut results = HashMap::new();
        for (doc, score) in scores {
            let content = std::fs::read_to_string(&doc).unwrap();
            let snippet = new_query
                .iter()
                .find_map(|term| generate_snippet(&content, term, 25));
            results.insert(doc, (score, snippet));
        }

        (results, all_suggestions)
    }

    fn suggest_words(&mut self, word: &str) -> Vec<String> {
        let stemmed_word = self.stem_word(word);

        let mut min_distance = usize::MAX;
        let mut suggestions = Vec::new();

        let unique_words: HashSet<String> = self
            .corpus
            .values()
            .flat_map(|doc_map| doc_map.keys().cloned())
            .collect();

        for corpus_word in unique_words {
            let stemmed_corpus_word = self.stem_word(&corpus_word);
            let distance = levenshtein(&stemmed_word, &stemmed_corpus_word);

            if distance < min_distance {
                min_distance = distance;
                suggestions.clear();
                suggestions.push(corpus_word);
            } else if distance == min_distance {
                suggestions.push(corpus_word);
            }
        }

        suggestions
    }

    fn stem_word(&mut self, word: &str) -> String {
        if let Some(stemmed_word) = self.stem_cache.get(word) {
            stemmed_word.clone()
        } else {
            let en_stemmer = Stemmer::create(Algorithm::English);
            let stemmed_word = en_stemmer.stem(word).to_owned().to_string();
            self.stem_cache
                .insert(word.to_string(), stemmed_word.clone());
            stemmed_word
        }
    }
}

fn generate_snippet(content: &str, term: &str, context: usize) -> Option<String> {
    if let Some(start) = content.find(term) {
        let start = start.saturating_sub(context);
        let end = (start + term.len() + 2 * context).min(content.len());
        Some(content[start..end].to_string())
    } else {
        None
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
    let en_stemmer = Stemmer::create(Algorithm::English);
    text.split_whitespace()
        .map(|word| en_stemmer.stem(&word.to_lowercase()).to_string()) // Convert Cow<'_, str> to String
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
