
pub fn raindrops(num: usize) -> String {
    let mut dropstring = String::new();
    if num % 3 == 0 {
            dropstring.push_str("Pling")
    }
    if num % 5 == 0 {
                dropstring.push_str("Plang")
    }
    if num % 7 == 0 {
            dropstring.push_str("Plong")
    }
    if dropstring.is_empty() {
        dropstring.push_str(& num.to_string())
    }
    dropstring
}
