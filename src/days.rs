use crate::{Result, inputs::select_input, stopwatch::FinishedStopwatch, tui};

use std::fmt;

pub fn start_day_selector(days: &[Day]) -> Result<()> {
    let day = tui::selection_prompt("Select the AoC day to run", days)?;
    day.run()
}

type Runner = fn(input: String) -> Result<FinishedStopwatch>;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Day<'a>(u8, &'a str, Runner);

impl<'a> Day<'a> {
    pub fn new(number: u8, name: &'a str, runner: Runner) -> Self {
        Day(number, name, runner)
    }

    fn run(&self) -> Result<()> {
        tui::title_banner(format!("Welcome to Day {}", self.0).as_str());

        let input = select_input(self.0).unwrap_or_else(|err| {
            panic!(
                "could not load input file for {day_num}, ensure they are present in ./static/inputs/day_{day_num}: {err}",
                day_num = self.0
            )
        });
        let sw = self.2(input)?;

        tui::title_banner("Stopwatch Results");
        println!("Operation took: {sw}");
        if cfg!(debug_assertions) {
            println!(
                "Stopwatch results are usually faster in release mode, use 'cargo run --release' to see the difference."
            );
        }

        Ok(())
    }
}

impl fmt::Display for Day<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.1)
    }
}

impl<'a> PartialEq for Day<'a> {
    fn eq(&self, other: &Self) -> bool {
        // Compare only the name & number to avoid unreliable function pointer comparisons.
        self.0 == other.0 && self.1 == other.1
    }
}

pub mod parts {
    use crate::{Result, tui};

    use std::fmt;

    #[derive(Debug, Copy, Clone)]
    pub enum Part<'a> {
        One(&'a str),
        Two(&'a str),
    }

    impl fmt::Display for Part<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let (Part::One(s) | Part::Two(s)) = *self;
            f.write_str(s)
        }
    }

    pub fn get_selection<'a>(part_1: &'a str, part_2: &'a str) -> Result<Part<'a>> {
        println!();

        let options = &[Part::One(part_1), Part::Two(part_2)];
        let opt = tui::selection_prompt("Please select what part to calculate", options)?;
        Ok(*opt)
    }
}
