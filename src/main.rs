mod agents;
mod helpers;
mod models;
mod services;

use helpers::terminal::{get_user_response, print_welcome};

fn main() -> Result<(), anyhow::Error> {
    print_welcome()?;

    loop {
        let user_response = get_user_response("What would you like to do?")?;
        println!("You said: {}", user_response);
    }
}
