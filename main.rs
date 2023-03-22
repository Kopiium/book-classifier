use std::collections::HashMap;

fn classify_books(books: &[(String, String)]) -> HashMap<String, Vec<String>> {
    let mut genres: HashMap<String, Vec<String>> = HashMap::new();

    for (book, description) in books {
        let description = description.to_lowercase();
        if description.contains("romance") {
            genres.entry("Romance".to_string()).or_default().push(book.to_string());
        } else if description.contains("mystery") {
            genres.entry("Mystery".to_string()).or_default().push(book.to_string());
        } else if description.contains("science fiction") {
            genres.entry("Science Fiction".to_string()).or_default().push(book.to_string());
        } else {
            genres.entry("Other".to_string()).or_default().push(book.to_string());
        }
    }

    genres
}

fn main() {
    let books = vec![
        ("Pride and Prejudice".to_string(), "A romance novel by Jane Austen.".to_string()),
        ("The Time Machine".to_string(), "A science fiction novel by H.G. Wells.".to_string()),
        ("The Maltese Falcon".to_string(), "A mystery novel by Dashiell Hammett.".to_string()),
        ("Inspecting Carol".to_string(), "A comedy book by Daniel J. Sullivan.".to_string()),
    ];
    
    let classified_books = classify_books(&books);
    
    for (genre, books) in classified_books {
        println!("{}:", genre);
        for book in books {
            println!("\t{}", book);
        }
    }
}
