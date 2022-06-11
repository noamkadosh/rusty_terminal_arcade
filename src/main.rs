mod games;
mod utils;

use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use games::{invaders::play_invaders, snake::play_snake};

fn main() -> std::io::Result<()> {
    let game_list = vec!["Invaders", "Snake", "Quit"];

    'main_menu: loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a game")
            .items(&game_list)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        let game_result = match selection {
            Some(0) => play_invaders(),
            Some(1) => play_snake(),
            Some(_index) => return Ok(()),
            None => return Ok(()),
        };

        match game_result {
            Ok(_) => {
                println!("Game finished");
            }
            Err(e) => {
                println!("Error: {}", e);
                break 'main_menu;
            }
        }
    }

    Ok(())
}
