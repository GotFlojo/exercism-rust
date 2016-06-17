pub fn square_of_sum(num: usize) -> usize {
    (1..num+1).fold(0, |acc, x| acc + x).pow(2)
}

pub fn sum_of_squares(num: usize) -> usize {
    (1..num+1).map(|n| n.pow(2)).fold(0, |acc, x| acc +x)
}

// Probably this can not be < 0. Number theorists to the rescue
pub fn difference(num: usize) -> usize {
        square_of_sum(num) - sum_of_squares(num)
}
