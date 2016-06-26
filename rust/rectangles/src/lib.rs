#![allow(unknown_lints)]

///
/// # Idea
///
/// For all `+`, search for another `+` with is to the bottom-right (higher index in both
/// structures). Check all possible combinations.
///
/// If found: check that corners are `+` and horizontal lines are `-` or `+` and that vertical
/// lines are `|` or `+`.
///
#[allow(needless_range_loop)]
pub fn count(lines: &[&'static str]) -> usize {
    // convert to Vec<Vec<char>> for easier access
    let data = lines.into_iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    let sizex = data.len();
    // only works if at least one entry exists in both dimensions
    if sizex == 0 {
        return 0;
    }
    let sizey = data[0].len();
    if sizey == 0 {
        return 0;
    }

    // check horizontal line for only + and - character
    let check_horizontal = |row: usize, ystart, yend| {
        data[row]
            .as_slice()
            .iter()
            .skip(ystart)
            .take(yend - ystart + 1)
            .all(|&c| c == '+' || c == '-')
    };
    // check vertical line for only + and | character
    let check_vertical = |column, xstart: usize, xend| {
        for row in xstart..xend + 1 {
            if data[row][column] != '+' && data[row][column] != '|' {
                return false;
            }
        }
        true
    };

    let mut count_rectangles = 0;
    // determine first corner
    for xstart in 0..sizex {
        for ystart in 0..sizey {
            if data[xstart][ystart] != '+' {
                continue;
            };

            // determine second corner
            // second corner must start at +1,+1 otherwise each `+` would be a square
            for xend in xstart + 1..sizex {
                for yend in ystart + 1..sizey {
                    // check if all three other corners are `+`
                    if data[xend][yend] != '+' || data[xstart][yend] != '+' ||
                       data[xend][ystart] != '+' {
                        continue;
                    }

                    // all lines in between must be the correct characters?
                    if check_horizontal(xstart, ystart, yend) &&
                       check_horizontal(xend, ystart, yend) &&
                       check_vertical(ystart, xstart, xend) &&
                       check_vertical(yend, xstart, xend) {
                        // all conditions met, rectangle found
                        count_rectangles += 1;
                    }
                }
            }
        }
    }

    count_rectangles
}
