pub fn verse(n: u32) -> String {
    if n == 0 {
        return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }

    if n == 1 {
        return format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n);
    }

    if n == 2 {
    return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1);
    }
     return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1);
}

pub fn sing(start: u32, end: u32) -> String {

    let mut final_string = String::new();
    for i in (end..=start).rev() {
        final_string.push_str(&verse(i));
        if (i != end) {
            final_string.push_str(&String::from("\n"));
        }
    }
    
    return final_string;

}

// Better solution
pub fn verse(n: u32) -> String {
    match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut final_string = String::new();
    for i in (end..=start).rev() {
        final_string.push_str(&verse(i));
        if i != end {
            final_string.push_str("\n");
        }
    }
    final_string
}
