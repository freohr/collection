use std::io;

use crate::app::{App, CurrentScreen};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};

mod tui;

mod app;
mod collection;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    tui::restore()?;

    match res {
        Ok(do_print) => {
            if do_print {
                app.print_collection();
            }
        }
        Err(err) => println!("{err:?}"),
    };

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|frame| ui::ui(frame, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                continue;
            }

            match app.current_screen {
                app::CurrentScreen::Main => match key.code {
                    KeyCode::Char('a') => {
                        app.current_screen = CurrentScreen::Add;
                    }
                    KeyCode::Char('q') => {
                        app.current_screen = CurrentScreen::Exit;
                    }
                    _ => {}
                },
                app::CurrentScreen::Exit => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true);
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') | KeyCode::Esc => {
                        return Ok(false);
                    }
                    _ => {}
                },
                app::CurrentScreen::Add => match key.code {
                    KeyCode::Enter => {
                        app.save_directory();
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Backspace => {
                        app.dir_input.pop();
                    }
                    KeyCode::Esc => {
                        app.dir_input.clear();
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char(value) => {
                        app.dir_input.push(value);
                    }
                    _ => {}
                },
            }
        }
    }
}
