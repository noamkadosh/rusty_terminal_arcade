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

    let game_list = vec!["Invaders", "Snake"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a game")
        .items(&game_list)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(0) => {
            start_invaders();
        }
        Some(index) => println!("User selected item : {}", game_list[index]),
        None => println!("User did not select anything"),
    }

    Ok(())
}
