pub fn hamming_distance(lhs: &str, rhs: &str) -> Result<usize, &'static str> {
    if lhs.chars().count() != rhs.chars().count() {
        Result::Err("inputs of different length")
    } else {
        let mut rhs_chars = rhs.chars();
        let mut count = 0;
        for left_char in lhs.chars() {
            if left_char != rhs_chars.next().unwrap() {
                count = count + 1
            }
        }
        Result::Ok(count)
    }
}
