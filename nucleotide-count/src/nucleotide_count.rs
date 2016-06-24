use std::collections::HashMap;

static NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, sequence: &str) -> usize {
    sequence.chars().filter(|&c| c == nucleotide).count()
}

pub fn nucleotide_counts(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::<char, usize>::with_capacity(4);
    for n in NUCLEOTIDES.into_iter() {
        counts.insert(*n, count(*n, &s.to_uppercase()));
    }
    counts
}
