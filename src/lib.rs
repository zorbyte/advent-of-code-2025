pub mod day_1;
pub mod day_2;
pub mod day_3;

pub mod days;
pub mod inputs;
pub mod stopwatch;
pub mod tui;

use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
