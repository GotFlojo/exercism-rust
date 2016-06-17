use std::collections::HashSet;

const ENGL_ALPHABET: [char; 26] =['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
                                  'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                                  'w', 'x', 'y','z'];
const GER_ALPHABET: [char; 30] =['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
                                 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
                                 'w', 'x', 'y','z', 'ä', 'ü', 'ö', 'ß'];

pub fn is_pangram(sentence: &str) -> bool {
    let letters: HashSet<char> = sentence.trim().to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
    println!("{}", ENGL_ALPHABET.len());
    println!("{}", letters.len());
    letters.len() == ENGL_ALPHABET.len() || letters.len() == GER_ALPHABET.len()
}
