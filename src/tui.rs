use crate::Result;

use std::fmt::Display;
use std::io::{self, Write};

pub fn selection_prompt<'a, O: Display>(question: &str, options: &'a [O]) -> Result<&'a O> {
    for (idx, option) in options.iter().enumerate() {
        println!("{}. {option}", idx + 1);
    }

    let answer = prompt(question)?.to_lowercase();
    let numerical_ans = answer.parse::<usize>().unwrap_or(0);

    options
        .iter()
        .enumerate()
        .find_map(|(idx, option)| {
            if numerical_ans == idx + 1 || answer == option.to_string().to_lowercase() {
                Some(option)
            } else {
                None
            }
        })
        .ok_or(Box::from("no valid option selected"))
}

pub fn title_banner(title: &str) {
    println!("\n{}", title);
    println!("{}\n", "=".repeat(title.len()));
}

pub fn prompt(question: &str) -> io::Result<String> {
    print!("{question} > ");
    // stdout is line buffered, so unlike with println, we need
    // to manually flush before we collect input since input is
    // on the same line of the question.
    io::stdout().flush()?;

    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}
