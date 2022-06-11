use crate::utils::frame::{new_frame, Drawable};
use crate::utils::render::render;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use rusty_audio::Audio;
use std::{
    error::Error,
    io,
    sync::mpsc::channel,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

pub mod direction;
pub mod food;
pub mod snake;

pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 20;

pub fn play_snake() -> Result<(), Box<dyn Error>> {
    // Audio
    // ToDo add audio
    let mut _audio = Audio::new();

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    let _ = stdout.execute(Hide);

    // Render loop in a separate thread
    let (render_tx, render_rx) = channel();
    let render_handle = spawn(move || {
        let mut last_frame = new_frame(NUM_COLS, NUM_ROWS);
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
    let mut snake = snake::Snake::new();
    let mut food = food::Food::new();
    let mut instant = Instant::now();

    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame(NUM_COLS, NUM_ROWS);

        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => snake.move_left(),
                    KeyCode::Right => snake.move_right(),
                    KeyCode::Up => snake.move_up(),
                    KeyCode::Down => snake.move_down(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Updates
        snake.update(delta);
        if snake.detect_food(&food) {
            food.eaten(&snake.body);
        }

        // Draw & Render
        let drawables: Vec<&dyn Drawable> = vec![&snake, &food];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        sleep(Duration::from_millis(1));

        // Win or Lose?
        if snake.hit_something() {
            // audio.play("lose");
            break 'gameloop;
        }
        if snake.is_max_length() {
            // audio.play("win");
            break 'gameloop;
        }
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    // audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
