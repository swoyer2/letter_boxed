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

    // Initiliaze global solution empty vec
    let mut sol: String = String::new();

    // Solves for best solution with a greedy algorithm
    solve(&words_filtered, &mut sol, &given_letters);

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

// Function to reduce words to those starting with a given letter
fn reduce_first_letter(letter: char, valid_words: &HashSet<String>) -> HashSet<String> {
    // Initialize HashSet for the filtered words
    let mut filtered_words = HashSet::new();

    // Check each word in the valid_words to see if it starts with the given letter
    for word in valid_words.iter() {
        if let Some(first_char) = word.chars().next() {
            if first_char == letter {
                // Add word to filtered words if it starts with the given letter
                filtered_words.insert(word.clone());
            }
        }
    }

    // Return filtered words
    filtered_words
}

// Function to check if all letters from given_letters are present in the solution
fn all_letters_used(sol: &str, given_letters: &str) -> bool {
    let solution_str: String = sol.chars().collect();

    for c in given_letters.chars() {
        if !solution_str.contains(c) {
            return false;
        }
    }
    true
}

// Recursive function that uses a greedy algorithm to solve the problem
fn solve(words_filtered: &HashSet<String>, sol: &str, given_letters: &str) -> String {
    let mut to_be_added: String = String::new();
    let mut max_count = 0;
    // keep copy of original words filtered
    let orig_words_filtered = &words_filtered.clone();

    // Words filtered after first letter check
    let words_filtered = if let Some(last_char) = sol.chars().last() {
        reduce_first_letter(last_char, words_filtered)
    } else {
        words_filtered.clone()
    };

    // Check if all letters are used in the solution
    if all_letters_used(sol, given_letters) {
        return sol.to_string();
    }

    // Find the next best word to add to the solution
    for word in words_filtered.iter() {
        let mut count = 0;
        let unique_chars: HashSet<char> = word.chars().collect();

        for c in unique_chars.iter() {
            if !sol.contains(*c) {
                count += 1;
            }
        }

        if count > max_count {
            to_be_added = word.clone();
            max_count = count;
        }
    }

    // Print and update the solution
    println!("Solution: {:?}", to_be_added);
    let mut new_sol = sol.to_string();
    new_sol.push_str(&to_be_added);
    new_sol.push_str(&solve(&orig_words_filtered, &new_sol, given_letters));
    new_sol
}