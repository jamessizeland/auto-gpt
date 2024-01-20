use std::io::stdin;

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

#[derive(Debug, PartialEq)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(
        &self,
        agent_role: &str,
        agent_statement: &str,
    ) -> Result<(), anyhow::Error> {
        let mut stdout = std::io::stdout();
        // Decide on print colour
        let colour = match self {
            PrintCommand::AICall => Color::Cyan,
            PrintCommand::UnitTest => Color::Magenta,
            PrintCommand::Issue => Color::Red,
        };
        // print the agent role in a specific color.
        stdout.execute(SetForegroundColor(Color::Green))?;
        print!("Agent[{}]: ", agent_role);
        stdout.execute(SetForegroundColor(colour))?;
        print!("{}", agent_statement);
        stdout.execute(ResetColor)?;
        Ok(())
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_agent_message() -> Result<(), anyhow::Error> {
        let print_command = PrintCommand::AICall;
        let agent_role = "Managing Agent";
        let agent_statement = "Hello, I am a Managing Agent.";
        print_command.print_agent_message(agent_role, agent_statement)?;
        Ok(())
    }
}
