/*
 * Exercise 9: The Word Counter
 *
 * This program counts the frequency of each word in a given text.
 * It uses a HashMap to store the words (keys) and their counts (values).
 * The logic is encapsulated within a `CounterApp` struct for better organization.
 */

// We must import HashMap from the standard library's collections module.
use std::collections::HashMap;

// We derive `Debug` to be able to print the struct for debugging.
#[derive(Debug)]
struct CounterApp {
    text: String,
}

impl CounterApp {
    /// Constructs a new CounterApp with the given text.
    fn new(text: String) -> CounterApp {
        // We create a new instance of the struct, initializing the `text` field
        // with the value passed as a parameter.
        CounterApp { text }
    }

    /// Processes the text and returns a HashMap of word frequencies.
    /// The return type is the concrete HashMap we are building.
    fn count_words(&self) -> HashMap<String, u32> {
        // 1. Create a new, empty, mutable HashMap.
        let mut word_counts = HashMap::new();

        // 2. Start the iterator chain on the text stored in the struct.
        self.text
            .split_whitespace() // Creates an iterator of `&str` slices.
            .map(|word| {
                // The normalization closure: cleans up each word.
                word.to_lowercase() // Convert to lowercase.
                    .trim_matches(|c| c == '.' || c == ',') // Remove trailing punctuation.
                    .to_string() // Convert the resulting `&str` into an owned `String`.
            })
            .for_each(|word| {
                // The aggregation closure: populates the HashMap.
                // `word` is now a clean, owned String.
                // The `entry` API gets a mutable handle to the word's count.
                let count = word_counts.entry(word).or_insert(0);
                // We dereference `count` (*) to modify the value inside the map and increment it.
                *count += 1;
            });

        // 3. The function must return the populated HashMap.
        word_counts
    }
}

fn main() {
    let phrase_app = CounterApp::new(String::from(
        "Ciao mondo, questo è un testo di prova.
        Ciao Rust, sei un linguaggio potente.
        Un testo di prova serve per provare.",
    ));

    // We call the `count_words` method to perform the calculation.
    let frequencies = phrase_app.count_words();

    // Now we print the final result, which is the HashMap itself.
    println!("Word frequencies found:");
    // Using `{:#?}` for "pretty-printing" makes the HashMap output more readable.
    println!("{:#?}", frequencies);
}
