use std::fmt;

use crate::{
    Result,
    days::activities::{self, Activity},
    stopwatch::{FinishedStopwatch, Stopwatch},
    tui,
};

// Had to use a third party regex package because rust's built in one
// doesn't support backreferences as a result of an opinionated design
// on best practice for regex.
use fancy_regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Parts {
    Part1,
    Part2,
}

impl fmt::Display for Parts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match *self {
            Self::Part1 => "Part 1 - Doubled-Up Numbers",
            Self::Part2 => "Part 2 - Repeating Patterns",
        })
    }
}

pub fn run(input: Option<String>) -> Result<FinishedStopwatch> {
    let input = input.expect("input to solve should be available for day 2");
    let activity = activities::get_selection(&[Parts::Part1, Parts::Part2])?;

    let sw = Stopwatch::start();

    let re = match activity {
        Activity(Parts::Part1) => Regex::new(r"^(\d+)\1{1}$")?,
        Activity(Parts::Part2) => Regex::new(r"^(\d+)\1+$")?,
    };

    let mut total = 0;
    for ids in produce_ids(&input, activity) {
        total += ids.iter().fold(0, |total, id| {
            if re.is_match(&id.to_string()).unwrap() {
                total + id
            } else {
                total
            }
        });
    }

    let sw = sw.stop();

    tui::title_banner(format!("Day 2 {activity} Answers").as_str());
    println!("Total: {total}");

    Ok(sw)
}

fn produce_ids(input: &str, activity: Activity<Parts>) -> impl Iterator<Item = Vec<u64>> {
    input.trim().split(',').map(move |x| {
        let parts: Vec<&str> = x.split('-').collect();
        let (start, last) = match activity {
            Activity(Parts::Part1) => (
                get_optimised_incl_range_value(parts[0], RoundingDirection::Up),
                get_optimised_incl_range_value(parts[1], RoundingDirection::Down),
            ),
            Activity(Parts::Part2) => (parts[0].parse().unwrap(), parts[1].parse().unwrap()),
        };

        (start..=last).collect()
    })
}

enum RoundingDirection {
    Up,
    Down,
}

fn get_optimised_incl_range_value(part: &str, dir: RoundingDirection) -> u64 {
    let piece = part.parse::<u64>().unwrap();
    let piece_len = part.len() as u32;

    // the problem says if something occurs twice
    // then it is an invalid ID, interestingly
    // this means that anything with an invalid ID
    // has to be a number of even length, and any part of
    // the range that has an odd length shouldn't even be considered.
    // e.g. 10 - 120 -> 10 - 99 is all that's needed
    //      5 - 17 -> 10 - 17 is all that's needed
    if piece_len % 2 == 1 {
        match dir {
            // e.g. 500 -> 10**3 -> 10 x 10 x 10 -> 1000
            RoundingDirection::Up => 10_u64.pow(piece_len),
            // e.g. 120 -> 10**(3 -1 = 2) -> 100 - 1 -> 99
            RoundingDirection::Down => 10_u64.pow(piece_len - 1) - 1,
        }
    } else {
        piece
    }
}
