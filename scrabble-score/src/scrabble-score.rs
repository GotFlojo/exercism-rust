fn letter_to_points(letter: char) -> i32 {
    match letter {
        'a' | 'e'| 'i'| 'o' | 'u'| 'l'| 'n'| 'r'| 's'| 't'  => 1,
        'd'| 'g'                                            => 2,
        'b'| 'c'| 'm'| 'p'                                  => 3,
        'f' | 'h' | 'v' | 'w' | 'y'                         => 4,
        'k'                                                 => 5,
        'j'| 'x'                                            => 8,
        'q'| 'z'                                            => 10,
        _                                                   => 0,
    }
}

pub fn score (word: &str) -> i32 {
    word.to_lowercase().chars().map(|c| letter_to_points(c)).fold(0, |acc, n| acc + n)
}
