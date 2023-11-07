use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall,
    UnitTest,
    Issue,
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_position: &str, agent_statement: &str) {
        let mut stdout = stdout();
        let stetement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent {}: ", agent_position);
        stdout.execute(SetForegroundColor(stetement_color)).unwrap();
        println!("{} ", agent_statement);
        stdout.execute(ResetColor).unwrap();
    }
}

// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = std::io::stdout();
    // PRint the question in specified color
    stdout.execute(SetForegroundColor(Color::Cyan)).unwrap();
    println!("");
    println!("{} ", question);

    // Rest the color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Faild to read response");

    // Trim white spaces
    user_response.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests_prints_agent_msg() {
        PrintCommand::AICall.print_agent_message("AI Manger testing agent", "Hello");
    }
}
