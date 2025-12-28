use std::{
    io,
    time::{Duration, Instant},
};
mod app;
mod engine;
mod ui;
use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    prelude::CrosstermBackend,
};

use crate::{
    app::App,
    engine::{Rng, SnowFlake},
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut rng = Rng::new(999);
    let mut snow = Vec::new();

    for _ in 0..220 {
        snow.push(SnowFlake {
            pos: glam::Vec3::new(
                rng.next_f32() * 40.0 - 20.0,
                10.0,
                rng.next_f32() * 20.0 - 10.0,
            ),
            drift: rng.next_f32() * 6.28,
            speed: 0.05 + rng.next_f32() * 0.12,
        });
    }

    let app = App::new(snow);
    let res = run_app(&mut terminal, app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        let elapsed = app.start_time.elapsed().as_secs_f32();

        if app.is_growing && app.anim_progress < 1.0  {
            app.anim_progress = (elapsed / 4.5).min(1.0);
        }
        if app.anim_progress >= 1.0 {
            for s in &mut app.snow {
                s.pos.y -= s.speed;
                s.pos.x += (s.drift + elapsed).sin() * 0.02;
                if s.pos.y < -10.0 {
                    s.pos.y = 10.0;
                }
            }
        }

        let dt = 0.05;
        if app.is_zooming {
            app.zoom_progress = (app.zoom_progress + dt).min(1.0);
        } else {
            app.zoom_progress = (app.zoom_progress - dt).max(0.0);
        }

        terminal.draw(|f| ui::render(f, &mut app, elapsed))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                let is_alt = key.modifiers.contains(KeyModifiers::ALT);
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => {
                        app.is_growing = true;
                        app.anim_progress = 0.0;
                        app.start_time = Instant::now();
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') if is_alt => {
                        if app.anim_progress >= 0.9 {
                            app.is_zooming = !app.is_zooming;
                        }
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Char(c) if !is_alt => {
                        app.input.push(c);
                    }
                    KeyCode::Left => {
                        app.angle_y -= 0.15;
                    }
                    KeyCode::Right => {
                        app.angle_y += 0.15;
                    }
                    KeyCode::Up => {
                        app.angle_x -= 0.15;
                    }
                    KeyCode::Down => {
                        app.angle_x += 0.15;
                    }
                    _ => {}
                }
            }
        }
    }
}
