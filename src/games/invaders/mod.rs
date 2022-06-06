use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

pub mod frame;
pub mod invaders;
pub mod player;
pub mod render;
pub mod shot;

use frame::{new_frame, Drawable};
use invaders::Invaders;
use player::Player;
use render::render;

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;

use rusty_audio::Audio;

use std::env;
use std::{
    error::Error,
    io,
    sync::mpsc::channel,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

pub fn start_invaders() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "src/games/invaders/audio/explode.wav");
    audio.add("lose", "src/games/invaders/audio/lose.wav");
    audio.add("move", "src/games/invaders/audio/move.wav");
    audio.add("pew", "src/games/invaders/audio/pew.wav");
    audio.add("startup", "src/games/invaders/audio/startup.wav");
    audio.add("win", "src/games/invaders/audio/win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    let _ = stdout.execute(Hide);

    // Render loop in a separate thread
    let (render_tx, render_rx) = channel();
    let render_handle = spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw & Render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        sleep(Duration::from_millis(1));

        // Win or Lose?
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
