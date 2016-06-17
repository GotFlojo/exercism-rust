
pub fn reply(we_say : &str) -> &'static str {
    if we_say.is_empty() {
        "Fine. Be that way!"
    } else {
        let inquiring = we_say.trim().ends_with("?");
        let loudly = we_say.trim().chars().filter(|&c| c.is_alphabetic()).all(|c| c.is_uppercase());
        if loudly {
            "Whoa, chill out!"
        } else if inquiring {
            "Sure."
        } else {
            "Whatever."
        }
    }
}
