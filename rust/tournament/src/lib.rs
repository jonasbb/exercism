use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// Receives a file as input
/// Writes the results table to output
/// Return value is number of correctly parsed games
pub fn tally(input: &Path, output: &Path) -> Result<usize, std::io::Error> {
    let file = try!{File::open(&input)};
    let file = BufReader::new(file);

    // stores Name => (W, D, L)
    let mut game_state = HashMap::new();
    let mut lines = 0;

    for line in file.lines() {
        let line2 = try!{line};
        let split = line2.split(';');
        let tmp: Vec<&str> = split.collect();

        if tmp.len() != 3 {
            // ignore input line
            continue;
        }
        match tmp[2] {
            "draw" => {
                (*game_state.entry(tmp[0].to_string()).or_insert((0, 0, 0))).1 += 1;
                (*game_state.entry(tmp[1].to_string()).or_insert((0, 0, 0))).1 += 1;
            }
            "win" | "loss" => {
                // just swtich team order
                let (t0, t1) = if tmp[2] == "win" {
                    (tmp[0].to_string(), tmp[1].to_string())
                } else {
                    (tmp[1].to_string(), tmp[0].to_string())
                };
                (*game_state.entry(t0.to_string()).or_insert((0, 0, 0))).0 += 1;
                (*game_state.entry(t1.to_string()).or_insert((0, 0, 0))).2 += 1;
            }
            _ => {
                // ignore input line
                continue;
            }
        }
        // count number of valid lines
        lines += 1;
    }

    let mut game_state: Vec<_> =
        game_state.into_iter().map(|(team, (w, d, l))| (team, w, d, l)).collect();
    // convert to negative, because highest value should be sorted first
    game_state.sort_by_key(|&(ref team, w, d, _)| (-(3 * w + d), -w, team.to_string()));


    let file = try!{OpenOptions::new().write(true).open(&output)};
    let mut file = BufWriter::new(file);
    try!{writeln!(file, "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", "Team", "MP", "W", "D", "L", "P")};
    for (team, w, d, l) in game_state {
        try!{writeln!(file, "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", team, w+d+l, w, d, l, 3*w+d)};
    }
    try!{file.flush()};
    Ok(lines)
}
