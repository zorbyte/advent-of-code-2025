use advent_of_code_2025::{
    day_1, day_2,
    days::{self, Day},
    tui,
};

fn main() {
    let completed_days = vec![
        Day::new(1, "Secret Entrance", day_1::run),
        Day::new(2, "Gift Shop", day_2::run),
    ];

    tui::title_banner("Welcome to AoC 2025!");
    if let Err(e) = days::start_day_selector(&completed_days) {
        eprintln!("An error occurred: {e}");
    }
}
