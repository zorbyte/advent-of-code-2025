use crate::Result;
use crate::tui;

use std::{ffi::OsString, fs, io, path::Path};

pub fn select_input(day_number: u8) -> Result<String> {
    let available_inputs = get_day_inputs(day_number)?;
    if available_inputs.is_empty() {
        return Err(Box::from("no inputs for day"));
    }

    let selection = tui::selection_prompt("Select the input you'd like to use", &available_inputs)?;

    Ok(fs::read_to_string(format!(
        "./static/inputs/day_{day_number}/{selection}"
    ))?)
}

fn get_day_inputs(day_number: u8) -> io::Result<Vec<String>> {
    let potential_path = format!("./static/inputs/day_{day_number}");
    let dir = Path::new(potential_path.as_str());
    if !dir.is_dir() {
        return Ok(vec![]);
    }

    let file_names: io::Result<Vec<OsString>> = fs::read_dir(dir)?
        .map(|res| res.map(|entry| entry.file_name()))
        .collect();

    Ok(file_names?
        .into_iter()
        .filter_map(|name| name.into_string().ok())
        .collect())
}
