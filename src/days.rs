use crate::{Result, inputs::select_input, stopwatch::FinishedStopwatch, tui};

use std::fmt;

pub fn start_day_selector(days: &[Day]) -> Result<()> {
    let day = tui::selection_prompt("Select the AoC day to run", days)?;
    day.run()
}

type Runner = fn(input: Option<String>) -> Result<FinishedStopwatch>;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Day<'a>(u8, &'a str, Runner);

impl<'a> Day<'a> {
    pub fn new(number: u8, name: &'a str, runner: Runner) -> Self {
        Day(number, name, runner)
    }

    fn run(&self) -> Result<()> {
        tui::title_banner(format!("Welcome to Day {}", self.0).as_str());

        let input = select_input(self.0).ok();
        let sw = self.2(input)?;

        tui::title_banner("Stopwatch Results");
        println!("Operation took: {sw}");

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

pub mod activities {
    use crate::{Result, tui};

    use std::fmt::{self, Display};

    #[derive(Debug, Copy, Clone)]
    pub struct Activity<T: Display>(pub T);

    impl<T: Display> Display for Activity<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    pub fn get_selection<T: Display + Copy>(options: &[T]) -> Result<Activity<T>> {
        println!();
        let opt = tui::selection_prompt("Please select the activity", options).copied()?;

        Ok(Activity(opt))
    }
}
