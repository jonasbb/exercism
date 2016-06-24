// use std::slice::SliceConcatExt;

pub fn verse(i: u8) -> String {
    if i == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store \
                and buy some more, 99 bottles of beer on the wall.\n"
            .to_string();
    };

    let mut s = String::new();
    verse1(i, &mut s);
    verse2(i - 1, &mut s);
    s
}

fn verse1(i: u8, s: &mut String) {
    // plural s
    let plural_s = if i == 1 {
        ""
    } else {
        "s"
    };
    s.push_str(&format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\n",
                        i,
                        plural_s,
                        i,
                        plural_s))
}

fn verse2(i: u8, s: &mut String) {
    if i == 0 {
        s.push_str("Take it down and pass it around, no more bottles of beer on the wall.\n")
    } else {
        // plural s
        let plural_s = if i == 1 {
            ""
        } else {
            "s"
        };
        s.push_str(&format!("Take one down and pass it around, {} bottle{} of beer on the wall.\n",
                            i,
                            plural_s))
    }
}

pub fn sing(start: u8, end: u8) -> String {
    (end..start + 1).rev().map(verse).collect::<Vec<String>>().join("\n")
}
