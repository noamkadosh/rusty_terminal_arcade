// use crossterm::{
//     cursor::{Hide, Show},
//     event::{self, Event, KeyCode},
//     terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
//     ExecutableCommand,
// };

mod games;

use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use games::invaders::start_invaders;

// use std::{error::Error, io, time::Duration};

fn main() -> std::io::Result<()> {
    // Terminal
    // let mut stdout = io::stdout();
    // terminal::enable_raw_mode()?;
    // stdout.execute(EnterAlternateScreen)?;
    // let _ = stdout.execute(Hide);

    let game_list = vec!["Invaders", "Snake", "Quit"];

    'main_menu: loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a game")
            .items(&game_list)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        let game_result = match selection {
            Some(0) => start_invaders(),
            Some(1) => start_invaders(),
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
