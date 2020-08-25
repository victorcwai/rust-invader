use rusty_audio::Audio;
use std::error::Error;
use std::{io, thread};
use crossterm::{terminal, ExecutableCommand, event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use std::time::{Duration, Instant};
use crossterm::event::{Event, KeyCode};
use std::sync::mpsc;
use invaders::{render, frame};
use invaders::frame::{new_frame, Drawable};
use invaders::player::Player;
use invaders::invader::Invaders;

/*
Raw mode
- Input will not be forwarded to screen
- Input will not be processed on enter press
- Input will not be line buffered (input sent byte-by-byte to input buffer)
- Special keys like backspace and CTL+C will not be processed by terminal driver
- New line character will not be processed therefore println! can't be used, use write! instead
 */

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?; // ? = crash program if fail
    stdout.execute(EnterAlternateScreen)?; // enter alternate screen in terminal
    stdout.execute(Hide)?; // hide cursor

    // Render loop (separate thread)
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            // receive curr_frame from main thread
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });


    // Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {
        // per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // input
        while event::poll(Duration::default())? { // keep polling until no input
            if let Event::Key(key) = event::read()? {
                match key.code {
                    // Moving player ship
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    // Exit the game (losing) by Esc or 'q'
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
        if invaders.upqdate(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw & render
        // Generics: create a vector of struct that impl Drawables
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for d in drawables {
            d.draw(&mut curr_frame);
        }
        // do not unwrap if render_tx error (because it runs before Render thread starts)
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1)); // to let Render thread finish rendering

        // Win or lose
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }
    // Cleanup threads/channels
    drop(render_tx);
    render_handle.join().unwrap();
    // Block until all audio stops playing
    audio.wait();
    // Cleanup terminal
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())


}


