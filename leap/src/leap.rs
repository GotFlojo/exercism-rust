pub fn is_leap_year(year: i64) -> bool {
    // Pattern matching FTW!!!
    let leap_mask: (bool, bool, bool) = (year % 4 == 0, year % 100 == 0, year % 400 == 0);
    match leap_mask {
        (_, _, true) => true,
        (_,true,_) => false,
        (true,_,_) => true,
        (false,false,false) => false,
    }
}
