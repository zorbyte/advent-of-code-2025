use crate::{
    Result,
    days::parts::{Part, get_selection},
    stopwatch::{FinishedStopwatch, Stopwatch},
};

pub fn run(input: String) -> Result<FinishedStopwatch> {
    let part = get_selection("Emergency Power", "Joltage Override!")?;

    let sw = Stopwatch::start();
    let digits = match part {
        Part::One(_) => 2,
        Part::Two(_) => 12,
    };

    println!("Maximum Joltage: {}", calc_joltage(input, digits));

    Ok(sw.stop())
}

fn calc_joltage(input: String, digits: usize) -> u64 {
    // Thanks to Madelyn-of-Hell's solution, which helped me
    // learn more about using comparisons on iterators in Rust
    // and all the funky things you can do with that.
    let mut maximum_joltage = 0_u64;
    for battery_bank in input.lines() {
        let mut battery_bank_largest_joltage = 0;
        let mut skipped_batteries_offset = 0_usize;
        // Start from the largest digit as we are trying to maximise the size of the number
        // and we are going left->right.
        for remaining_digits in (1_usize..=digits).rev() {
            let digits_consumed = digits - remaining_digits;
            // To start the window at the next set of batteries to check,
            // we add the number of batteries we've consumed and the number
            // of batteries skipped together.
            //
            // This helps us consider situations of a battery bank of
            // "987654321" in which we skip none, but we must consume more digits
            // in order to add to our battery bank's highest joltage.
            let window_start_idx = skipped_batteries_offset + digits_consumed;
            // The end of the window expands over each iteration in order to consume
            // closer and closer to the end of the battery bank as the start gets closer
            // towards the end and thus keeps the number of batteries available to add to
            // our max joltage consistent with how many digits are left in the number to
            // accumulate.
            let window_end_idx = battery_bank.len() - remaining_digits;
            let shrinking_window = &battery_bank[window_start_idx..=window_end_idx];

            let highest_joltage_battery = shrinking_window
                .as_bytes()
                .iter()
                .map(|b| b.wrapping_sub(b'0'))
                .enumerate()
                // If any given window has a reoccurrence of a highest joltage (e.g. two '9's),
                // we want the first one in order to prevent the window from shrinking too quickly.
                // All `max` functions for math iterate left->right, and as such if two maximum
                // values are the same the newest one is returned (i.e. the one that occurs after
                // all the others). To ensure that our digits stay chronological with left->right
                // ordering, we want the first incident of the highest number, not the last,
                // so we reverse the iterator to achieve this.
                .rev()
                // Get the highest joltage battery,
                .max_by(|(_, battery_a), (_, battery_b)| battery_a.cmp(battery_b))
                .inspect(|(battery_windowed_idx, _)| {
                    // The index of the battery in this window signifies how many
                    // batteries we had to skip over until we got this highest value.
                    skipped_batteries_offset += battery_windowed_idx
                })
                .map(|(_, battery)| battery as u64)
                .expect("battery bank should have comparable values");

            battery_bank_largest_joltage +=
                highest_joltage_battery * 10_u64.pow(remaining_digits as u32 - 1);
        }

        maximum_joltage += battery_bank_largest_joltage;
    }

    maximum_joltage
}
