pub fn verse(n: i32) -> String {
    // We deal with the plurals and other differences here.
    let ns = n.to_string();

    let bottle_number = if n > 1 {
        let mut tmp = String::from(ns);
        tmp.push_str(" bottles of beer");
        tmp
    } else if n == 1 {
        let mut tmp = String::from(ns);
        tmp.push_str(" bottle of beer");
        tmp
    } else {
        String::from("No more bottles of beer")
    };

    let mut verse = String::from(&bottle_number);
    verse.push_str(" on the wall, ");
    verse.push_str(&bottle_number.to_lowercase());
    verse.push_str(".\n");

    let nm = if n > 0 {n - 1} else {99};
    let nms = nm.to_string();

    let second_line = if nm == 0 {
        String::from("Take it down and pass it around, no more bottles of beer on the wall.\n")
    } else if nm == 1 {
        String::from("Take one down and pass it around, 1 bottle of beer on the wall.\n")
    } else if nm == 99 {
        String::from("Go to the store and buy some more, 99 bottles of beer on the wall.\n")
    } else {
        let mut tmp = String::from("Take one down and pass it around, ");
        tmp.push_str(&nms);
        tmp.push_str(" bottles of beer on the wall.\n");
        tmp
    };

    verse.push_str(&second_line);
    verse
}

pub fn sing(start: i32, end: i32) -> String {
    let mut res = String::new();
    let start = start + 1;
    for i in (end..start).rev() {
        res.push_str(&verse(i));
        res.push_str("\n");
    }
    res.pop();
    res
}
