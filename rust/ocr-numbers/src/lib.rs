#[allow(needless_range_loop)]
pub fn convert<I: AsRef<str>>(input: I) -> Result<String, ()> {
    let input = input.as_ref();
    // convert the single string into a 2D array of char
    let lines = input.split('\n').map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    // check that every OCR line has a multiple of 4 text lines
    if lines.len() % 4 != 0 {
        return Err(());
    }

    // check that every line contains a multiple of the size of a single digit (3 chars)
    if lines.iter().any(|x| x.len() % 3 != 0) {
        return Err(());
    }

    let mut res = String::new();

    // iterate over all lines
    for i in 0..(lines.len() / 4) {
        // join with ","
        if i > 0 {
            res.push(',');
        }

        // iterate over all numbers
        for j in 0..(lines[i].len() / 3) {
            // read a character as the 3x3 grid and convert it into a tuple
            let c = match (lines[i * 4    ][j * 3    ],
                           lines[i * 4 + 1][j * 3    ],
                           lines[i * 4 + 2][j * 3    ],
                           lines[i * 4    ][j * 3 + 1],
                           lines[i * 4 + 1][j * 3 + 1],
                           lines[i * 4 + 2][j * 3 + 1],
                           lines[i * 4    ][j * 3 + 2],
                           lines[i * 4 + 1][j * 3 + 2],
                           lines[i * 4 + 2][j * 3 + 2]) {

                // match those 9-tuples to the numbers they correspond to
                (' ', ' ', ' ', ' ', ' ', ' ', ' ', '|', '|') => '1',
                (' ', ' ', '|', '_', '_', '_', ' ', '|', ' ') => '2',
                (' ', ' ', ' ', '_', '_', '_', ' ', '|', '|') => '3',
                (' ', '|', ' ', ' ', '_', ' ', ' ', '|', '|') => '4',
                (' ', '|', ' ', '_', '_', '_', ' ', ' ', '|') => '5',
                (' ', '|', '|', '_', '_', '_', ' ', ' ', '|') => '6',
                (' ', ' ', ' ', '_', ' ', ' ', ' ', '|', '|') => '7',
                (' ', '|', '|', '_', '_', '_', ' ', '|', '|') => '8',
                (' ', '|', ' ', '_', '_', '_', ' ', '|', '|') => '9',
                (' ', '|', '|', '_', ' ', '_', ' ', '|', '|') => '0',
                _ => '?',
            };
            res.push(c);
        }
    }
    Ok(res)
}
