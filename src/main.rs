use std::io;
use std::fs;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // Get valid words
    let valid_words:HashSet<String> = read_file("./src/words.txt");

    // Get given letters from game
    println!("Please enter given letters:");
    let given_letters = read_string();

    // Create the sides based off the given letters
    let sides = create_side(given_letters.clone());
    print_sides(sides);

    // Reduces the valid words by the letters given then puts the new valid words
    // into a new file called words_filtered.txt
    reduce_available_letters(sides, &valid_words);
    let valid_words:HashSet<String> = read_file("./src/words_filtered.txt");

    // Reduces the valid words by the not allowing same line words then changes
    // the words_filtered file to account for the new words
    reduce_on_line(sides, &valid_words);
    let words_filtered:HashSet<String> = read_file("./src/words_filtered.txt");

    // Initiliaze global solution empty HashSet
    let mut sol: HashSet<String> = HashSet::new();

    // Solves for best solution with a greedy algorithm
    solve(words_filtered, sol);

}

fn read_string() -> String {
    // Create a new String to hold the input
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Trim any trailing newline characters from the input
            input.trim().to_string()
        }
        Err(error) => {
            println!("Error reading input: {}", error);
            // Return an empty String if an error occurs
            String::new()
        }
    }
}

fn read_file(loc: &str) -> HashSet<String> {
    println!("In file {}", loc);

    let contents = fs::read_to_string(loc)
        .expect("Should have been able to read the file");

    let mut strings: HashSet<String> = HashSet::new();
    
    // Split the contents by newline characters and insert each word into the HashSet
    for word in contents.lines() {
        strings.insert(word.to_string());
    }

    strings
}

fn create_side(given_letters: String) -> [[char; 3]; 4] {
    // Create an array of arrays to store characters
    let mut sides: [[char; 3]; 4] = [[' '; 3]; 4]; // Initialize with spaces
    
    // Convert the string into an iterator of characters
    let mut chars = given_letters.chars();
    
    // Fill the array of arrays with characters
    for i in 0..4 {
        for j in 0..3 {
            if let Some(c) = chars.next() {
                sides[i][j] = c;
            } else {
                break; // Break if there are no more characters
            }
        }
    }
    
    sides // Return the array of arrays
}

fn print_sides(sides: [[char; 3]; 4]){
    // Print all sides of the array
    for (index, array) in sides.iter().enumerate() {
        println!("Side {}: {:?}", index + 1, array);
    }
}

// Writes a given HashSet to a file seperating each string by newlines
fn write_set_to_file(set: &HashSet<String>, filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for item in set {
        writeln!(&mut writer, "{}", item)?;
    }

    writer.flush()?;
    Ok(())
}

// Creates a new file that contains the valid words that use the letter given 
// by the user.
fn reduce_available_letters(sides: [[char; 3]; 4], valid_words: &HashSet<String>){
    // Create a HashSet of all the letters that we can't use
    let mut not_available_letters: HashSet<char> = (b'a'..=b'z').map(|c| c as char).collect();

    // Initialize empty HashSet for filtered words
    let mut filtered_words: HashSet<String> = HashSet::new();

    // Remove letters we can use from the not_available_letters HashMap
    for side in &sides {
        for &letter in side.iter() {
            not_available_letters.remove(&letter);
        }
    }

    // Check each word in the valid_words to see if they only contain allowed letters
    // If so add that word to the filtered words HashSet
    for word in valid_words {
        let contains_unavailable = word.chars().any(|c| not_available_letters.contains(&c));
        if !contains_unavailable {
            filtered_words.insert(word.to_string());
        }
    }

    // Create a file with the filtered words
    let _ = write_set_to_file(&filtered_words, "src/words_filtered.txt");
}

// Creates a file that contains the valid words given but makes sure that
// we don't use letters on the same line back to back.
fn reduce_on_line(sides: [[char; 3]; 4], valid_words: &HashSet<String>) {
    // Initialize HashSet for the filtered words that is a clone of previous valid words.
    let mut filtered_words: HashSet<String> = valid_words.clone();

    // Check each side then check each word in the valid_words to see if it follows the
    // no letters of the same line next to each other rule.
    for side in &sides {
        for word in valid_words.iter() {
            let chars: Vec<char> = word.chars().collect();

            for i in 0..(chars.len() - 1) {
                let current_char = chars[i];
                let next_char = chars[i + 1];

                // Check if the characters are next to each other in the side
                if side.contains(&current_char) && side.contains(&next_char) {
                    // remove word from the filtered words
                    filtered_words.remove(word);
                    break;
                }
            }
        }
    }

    // Create a file with the filtered words
    let _ = write_set_to_file(&filtered_words, "src/words_filtered.txt");
}

// Recursive function that uses a greedy alg in order to solve the problem
fn solve(words_filtered: HashSet<String>, sol: HashSet<String>) {
    let mut to_be_added: String = String::new();
    let mut max_count = 0;
    for word in words_filtered.iter() {
        let mut count = 0;

        for c in word.chars() {

            if !sol.contains(&c.to_string()) {
                count += 1;
            }
        }

        if count > max_count {
            to_be_added = word.to_string();
            max_count = count;

        }
    }
    let mut sol: HashSet<String> = HashSet::new();
    sol.insert(to_be_added);
    println!("{:?}", sol);
    println!("{:?}", max_count);
}
