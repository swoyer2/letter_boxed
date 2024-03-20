use std::io;
use std::fs;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let valid_words:HashSet<String> = read_file("./src/words.txt");
    println!("Please enter given letters:");
    let given_letters = read_string();
    let sides = create_side(given_letters.clone());

    print_sides(sides);

    reduce_available_letters(sides, &valid_words);

    let valid_words:HashSet<String> = read_file("./src/words_filtered.txt");

    reduce_on_line(sides, &valid_words);

    let words_filtered:HashSet<String> = read_file("./src/words_filtered.txt");

    solve(words_filtered);

}

fn read_string() -> String {
    let mut input = String::new(); // Create a new String to hold the input

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Trim any trailing newline characters from the input
            input.trim().to_string()
        }
        Err(error) => {
            println!("Error reading input: {}", error);
            String::new() // Return an empty String if an error occurs
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

fn write_set_to_file(set: &HashSet<String>, filename: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    for item in set {
        writeln!(&mut writer, "{}", item)?;
    }

    writer.flush()?;
    Ok(())
}

fn reduce_available_letters(sides: [[char; 3]; 4], valid_words: &HashSet<String>){
    let mut not_available_letters: HashSet<char> = (b'a'..=b'z').map(|c| c as char).collect();
    let mut filtered_words: HashSet<String> = HashSet::new();
    for side in &sides {
        for &letter in side.iter() {
            not_available_letters.remove(&letter);
        }
    }

    for word in valid_words {
        let contains_unavailable = word.chars().any(|c| not_available_letters.contains(&c));
        if !contains_unavailable {
            filtered_words.insert(word.to_string());
        }
    }
    let _ = write_set_to_file(&filtered_words, "src/words_filtered.txt");
}

fn reduce_on_line(sides: [[char; 3]; 4], valid_words: &HashSet<String>) {
    let mut filtered_words: HashSet<String> = valid_words.clone();

    for side in &sides {
        for word in valid_words.iter() {
            let chars: Vec<char> = word.chars().collect();

            for i in 0..(chars.len() - 1) {
                let current_char = chars[i];
                let next_char = chars[i + 1];

                // Check if the characters are next to each other in the side
                if side.contains(&current_char) && side.contains(&next_char) {
                    filtered_words.remove(word);
                    break;
                }
            }
        }
    }
    let _ = write_set_to_file(&filtered_words, "src/words_filtered.txt");
}

fn solve(words_filtered: HashSet<String>) {
    let mut sol: HashSet<String> = HashSet::new();
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
    sol.insert(to_be_added);
    println!("{:?}", sol);
    println!("{:?}", max_count);
}
