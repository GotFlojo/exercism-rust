pub fn verse(num: usize) -> String {

    match num {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", num, num, num -1),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", num, num, num -1),
    }
}

pub fn sing(start: usize, end: usize) -> String {
    let mut song: Vec<String> = Vec::new();
    for line in (end..start+1).rev() {
        song.push(verse(line))
    }
    song.join("\n")
}
