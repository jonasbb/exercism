pub fn annotate(input: &Vec<&str>) -> Vec<String> {
    let data: Vec<Vec<bool>> =
        input.into_iter().map(|s| s.chars().map(|c| c == '*').collect()).collect();
    let sizex = data.len() as isize;
    let sizey = data[0].len() as isize;


    // looks up a single position
    let is_mine = |x, y| {
        // invalid positions return false
        // assumes all sub arrays to be the same length
        if x < 0 || x >= sizex || y < 0 || y >= sizey {
            false
        } else {
            // x,y cannot be negative
            data[x as usize][y as usize]
        }
    };


    // store all offsets to look for mines
    let mut offs = Vec::with_capacity(9);
    // all possible offsets to count
    for offx in -1..2 {
        for offy in -1..2 {
            offs.push((offx, offy))
        }
    }
    let offs = offs.as_slice();


    // build output
    let mut res = Vec::with_capacity(sizex as usize);
    for x in 0..sizex {
        let mut tmp = String::with_capacity(sizey as usize);

        for y in 0..sizey {
            match is_mine(x, y) {
                true => tmp.push('*'),
                false => {
                    tmp.push_str(&match offs.into_iter()
                        .filter(|&&(a, b)| is_mine(x + a, y + b))
                        .count() {
                        0 => " ".to_string(),
                        x => format!("{}", x),
                    })
                }
            }
        }

        res.push(tmp);
    }
    res
}
