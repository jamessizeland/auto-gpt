use std::io::stdin;

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

/// Get user request
pub fn get_user_response(question: &str) -> Result<String, anyhow::Error> {
    let mut stdout = std::io::stdout();

    // print the question in a specific color.
    stdout.execute(SetForegroundColor(Color::Green))?;
    println!("");
    println!("{}", question);

    // reset the terminal color to default.
    stdout.execute(ResetColor)?;

    // read the user input.
    let mut user_response = String::new();
    stdin().read_line(&mut user_response)?;

    // trim whitespace and return response
    Ok(user_response.trim().to_string())
}

/// Print welcome message
pub fn print_welcome() -> Result<(), anyhow::Error> {
    let mut stdout = std::io::stdout();
    stdout.execute(SetForegroundColor(Color::White))?;
    println!("");
    println!("----------------------------------");
    stdout.execute(SetForegroundColor(Color::Red))?;
    println!("P r o j e c t   V i t r u v i u s");
    stdout.execute(SetForegroundColor(Color::White))?;
    println!("----------------------------------");
    stdout.execute(SetForegroundColor(Color::Green))?;
    println!("");
    println!("Welcome...");
    println!("I am your personal ai assistant, Vitruvius.");
    stdout.execute(ResetColor)?;
    Ok(())
}
