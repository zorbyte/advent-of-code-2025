use crate::Result;

use fancy_regex::Regex;

use std::{
    fmt::Display,
    io::{Write, stdin, stdout},
    sync::LazyLock,
};

static ALPHANUMERIC_WORDS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([\w\.]+)").unwrap());

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
            let lowercase_option_name = option.to_string().to_lowercase();
            // Turns something like "This is part 1!!" into
            // "thisispart1"
            // FIXME:
            // this is a horror, not because its complicated,
            // but because the user can't easily know about it,
            // as it is not being passed to the error outputs
            // as a candidate option.
            //
            // Also, has bugs with the-thing -> the,
            // we want thething.
            let typeable_option_name = lowercase_option_name
                .split_whitespace()
                .filter_map(|word| ALPHANUMERIC_WORDS.captures(word).ok().unwrap()?.get(0))
                .map(|word| word.as_str())
                .collect::<String>();

            if numerical_ans == idx + 1
                || answer == typeable_option_name
                || answer == lowercase_option_name
            {
                Some(option)
            } else {
                None
            }
        })
        .ok_or_else(|| {
            Box::from(format!(
                "invalid option selected, got \"{answer}\", wanted 1-{}",
                options.len()
            ))
        })
}

pub fn title_banner(title: &str) {
    println!("\n{}", title);
    println!("{}\n", "=".repeat(title.len()));
}

pub fn prompt(question: &str) -> Result<String> {
    print!("{question} > ");
    // stdout is line buffered, so unlike with println, we need
    // to manually flush before we collect input since input is
    // on the same line of the question.
    stdout().flush()?;

    let mut buffer = String::new();
    let stdin = stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

#[cfg(target_os = "windows")]
pub fn clear() {
    std::process::Command::new("cls").status().unwrap();
}

#[cfg(not(target_os = "windows"))]
pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!();
}
