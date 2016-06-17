pub fn hamming_distance(lhs: &str, rhs: &str) -> Result<usize, &'static str> {
    if lhs.chars().count() != rhs.chars().count() {
        Result::Err("inputs of different length")
    } else {
        let count = lhs.chars().zip(rhs.chars())
                               .filter(|&t| t.0 != t.1)
                               .count();
        Result::Ok(count)

    }
}
