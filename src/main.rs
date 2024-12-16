// Word list from: https://github.com/dwyl/english-words/blob/master/words.txt
// Other word list at https://github.com/raun/Scrabble/blob/master/words.txt (doesn't contain words with more than 8 letters)

use std::fs;

fn is_valid(word: &str, sides: [&str; 4]) -> bool {
    let valid_letters = sides.join("");

    // Check for length
    if word.len() < 3 {
        return false;
    }
    // Check all letters allowed
    if !word.chars().all(|c| valid_letters.contains(c)) {
        return false;
    }

    for (i, c) in word.char_indices() {
        // Check for letters not allowed
        if i + 1 < word.len() {
            // Check that next letter is possible to get to
            let this_side = sides
                .iter()
                .filter(|&side| side.contains(c))
                .nth(0)
                .unwrap();
            if this_side.contains(word.chars().nth(i + 1).unwrap()) {
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let start = std::time::Instant::now();
    let sides = ["LTO", "PEW", "CMN", "KRI"];
    let valid_letters = sides.join("");

    let wordfile_contents =
        fs::read_to_string("src/words.txt").expect("Should have been able to read the file");

    let valid_words = wordfile_contents
        .split("\n")
        .map(|word| word.to_uppercase())
        .filter(|word| is_valid(&word, sides))
        .collect::<Vec<_>>();

    println!("There are {} valid words", valid_words.len());

    for first_word in &valid_words {
        'second_word_loop: for second_word in &valid_words {
            if first_word.chars().last() != second_word.chars().nth(0) {
                continue;
            }

            for c in valid_letters.chars() {
                if !(first_word.contains(c) || second_word.contains(c)) {
                    continue 'second_word_loop;
                }
            }

            println!("{first_word}, {second_word}");
        }
    }
    let elapsed = start.elapsed();
    println!("the elapsed time is: {elapsed:?}");
}
