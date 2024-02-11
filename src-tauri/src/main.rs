// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use corpus::init::Database;
use once_cell::sync::OnceCell;
use serde_json::json;
use std::sync::Mutex;

static DATABASE: OnceCell<Mutex<Database>> = OnceCell::new();

#[tauri::command]
async fn directory(path: String) -> bool {
    println!("The path is : {}", path);
    if std::path::Path::new(&path).is_dir() {
        println!("The path is a directory");
        let db = Database::new(&path);
        DATABASE.set(Mutex::new(db)).is_ok()
    } else {
        println!("The path is not a directory");
        false
    }
}

use std::time::Instant;

#[tauri::command]
async fn search_query(query: String) -> Result<serde_json::Value, String> {
    let value = &query;
    println!("{}", query);

    // Split the input string into words
    let query: Vec<String> = value.split_whitespace().map(|s| s.to_string()).collect();

    // Get a lock on the database
    let mut db = DATABASE
        .get()
        .ok_or("Database not initialized")?
        .lock()
        .unwrap();

    // Start the timer
    let start = Instant::now();

    // Perform the search
    let (scores, suggestions) = db.search(&query);

    // Stop the timer and print the elapsed time
    let duration = start.elapsed();
    println!("Time elapsed in search is: {:?}", duration);

    println!("All the scores are : {}", scores.len());

    // Sort the results by score in descending order
    let mut results: Vec<(&String, &(f64, Option<String>))> = scores.iter().collect();
    results.sort_by(|a, b| (b.1).0.partial_cmp(&(a.1).0).unwrap());

    // Convert the results to JSON
    let results_json: Vec<serde_json::Value> = results
        .into_iter()
        .map(|(doc, (score, snippet))| {
            json!({
                "document": doc,
                "score": score,
                "snippet": snippet
            })
        })
        .collect();

    let response = json!({
        "time_taken": duration.as_millis().to_string(),
        "results": results_json,
        "suggestions": suggestions
    });

    Ok(response)
}

mod corpus;
fn main() {
    //let database = Arc::new(Database::new());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_query, directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
