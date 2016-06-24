// just hard code all special cases
const VERSES: [&'static str; 3] =
    ["No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy \
      some more, 99 bottles of beer on the wall.\n",
     "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more \
      bottles of beer on the wall.\n",
     "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 \
      bottle of beer on the wall.\n"];

pub fn verse(i: usize) -> String {
    match i {
        0 | 1 | 2 => VERSES[i].to_string(),
        _ => {
            format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and \
                     pass it around, {1} bottles of beer on the wall.\n",
                    i,
                    i - 1)
        }
    }
}

pub fn sing(start: usize, end: usize) -> String {
    (end..start + 1).rev().map(verse).collect::<Vec<String>>().join("\n")
}
