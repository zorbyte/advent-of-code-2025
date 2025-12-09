use crate::{Result, tui};

use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string},
    io::Result as IoResult,
    path::Path,
};

pub fn select_input(day_number: u8) -> Result<String> {
    let dir_path = format!("./static/inputs/day_{day_number}");
    let available_inputs = get_day_inputs(&dir_path)?;

    if available_inputs.is_empty() {
        return Err(Box::from(format!(
            "no inputs found for day {day_number} in directory {}",
            dir_path
        )));
    }

    let selection = tui::selection_prompt("Select the input you'd like to use", &available_inputs)?;

    Ok(read_to_string(format!("{dir_path}/{selection}"))?)
}

fn get_day_inputs(dir_path: &str) -> Result<Vec<String>> {
    let dir = Path::new(dir_path);
    if !dir.is_dir() {
        return Ok(vec![]);
    }

    let file_names: IoResult<Vec<OsString>> = read_dir(dir)?
        .map(|res| res.map(|entry| entry.file_name()))
        .collect();

    Ok(file_names?
        .into_iter()
        .filter_map(|name| name.into_string().ok())
        .collect())
}
