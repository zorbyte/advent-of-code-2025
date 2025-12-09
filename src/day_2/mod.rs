use std::{result::Result as StdResult, str::FromStr};

use crate::{
    Result,
    days::parts::{Part, get_selection},
    stopwatch::{FinishedStopwatch, Stopwatch},
    tui::title_banner,
};

// Had to use a third party regex package because rust's built in one
// doesn't support backreferences as a result of an opinionated design
// on best practice for regex.
use fancy_regex::Regex;

pub fn run(input: String) -> Result<FinishedStopwatch> {
    let part = get_selection("Doubled-Up Numbers", "Repeating Patterns")?;

    let sw = Stopwatch::start();

    let (part_num, re) = match part {
        Part::One(_) => (1, Regex::new(r"^(\d+)\1{1}$")?),
        Part::Two(_) => (2, Regex::new(r"^(\d+)\1+$")?),
    };

    let mut total = 0;
    for ids in produce_ids(&input, part) {
        for id in ids {
            if re.is_match(id.to_string().as_str())? {
                total += id
            }
        }
    }

    let sw = sw.stop();

    title_banner(format!("Day 2 Part {part_num} {part} Answers").as_str());
    println!("Total: {total}");

    Ok(sw)
}

fn produce_ids(input: &str, part: Part) -> impl Iterator<Item = Vec<u64>> {
    input.trim().split(',').map(move |x| {
        let parts: Vec<&str> = x.split('-').collect();
        let (start, last) = match part {
            Part::One(_) => (
                get_optimised_incl_range_value(parts[0], RoundingDirection::Up),
                get_optimised_incl_range_value(parts[1], RoundingDirection::Down),
            ),
            Part::Two(_) => (parts[0].parse(), parts[1].parse()),
        };

        let (start, last) = (
            start.expect("digits for starting number in ID range should be valid when parsed from puzzle input"),
            last.expect("digits for last number in ID range should be valid when parsed from puzzle input"),
        );

        (start..=last).collect()
    })
}

enum RoundingDirection {
    Up,
    Down,
}

fn get_optimised_incl_range_value(
    part: &str,
    dir: RoundingDirection,
) -> StdResult<u64, <u64 as FromStr>::Err> {
    let piece = part.parse::<u64>()?;
    let piece_len = part.len() as u32;

    // the problem says if something occurs twice
    // then it is an invalid ID, interestingly
    // this means that anything with an invalid ID
    // has to be a number of even length, and any part of
    // the range that has an odd length shouldn't even be considered.
    // e.g. 10 - 120 -> 10 - 99 is all that's needed
    //      5 - 17 -> 10 - 17 is all that's needed
    if piece_len % 2 == 1 {
        Ok(match dir {
            // e.g. 500 -> 10**3 -> 10 x 10 x 10 -> 1000
            RoundingDirection::Up => 10_u64.pow(piece_len),
            // e.g. 120 -> 10**(3 -1 = 2) -> 100 - 1 -> 99
            RoundingDirection::Down => 10_u64.pow(piece_len - 1) - 1,
        })
    } else {
        Ok(piece)
    }
}
