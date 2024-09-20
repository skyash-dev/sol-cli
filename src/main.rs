use std::{error::Error, io};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};


mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, ActiveTab, CurrentlyEditing, ActionType},
    ui::cli_ui,
};

fn main() -> Result<(), Box<dyn Error>> {

    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        // Draw the UI
        terminal.draw(|f| cli_ui(f, app))?;

        // Handle keyboard events
        if let Event::Key(key) = event::read()? {
            // Only handle KeyEventKind::Press
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        // Switch to editing screen
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::PublicKey);
                    }
                    KeyCode::Char('q') => {
                        // Switch to exiting screen
                        app.current_screen = CurrentScreen::Exiting;
                    }
                    KeyCode::Char('c') => {
                        // Handle command input or toggle command input mode if needed
                        app.current_screen = CurrentScreen::CommandLog;
                    }
                    _ => {}
                },
                CurrentScreen::Exiting => match key.code {
                    KeyCode::Char('y') => {
                        return Ok(true); // Confirm exit
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        // Return to the main screen and clear editing state
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    _ => {}
                },
                CurrentScreen::Editing => match key.code {
                    KeyCode::Enter => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::PublicKey => {
                                    app.currently_editing = Some(CurrentlyEditing::KeypairFile);
                                }
                                CurrentlyEditing::KeypairFile => {
                                    app.currently_editing = Some(CurrentlyEditing::Config);
                                }
                                CurrentlyEditing::Config => {
                                    app.currently_editing = Some(CurrentlyEditing::SolAmount);
                                }
                                CurrentlyEditing::SolAmount => {
                                    // Perform an action or save and return to the main screen
                                    // app.save_key_value(); // or any other method to save the state
                                    app.current_screen = CurrentScreen::Main;
                                }
                                CurrentlyEditing::None => {
                                    println!("none!");
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::PublicKey => {
                                    app.command_input.pop(); // Adjust this if you have other input fields
                                }
                                CurrentlyEditing::KeypairFile => {
                                    app.keypair_file.pop();
                                }
                                CurrentlyEditing::Config => {
                                    app.config_value.pop();
                                }
                                CurrentlyEditing::SolAmount => {
                                    // Convert sol_amount to String for simplicity
                                    let amount_str = app.sol_amount.to_string();
                                    let mut chars: Vec<char> = amount_str.chars().collect();
                                    chars.pop();
                                    app.sol_amount = chars.iter().collect::<String>().parse().unwrap_or(0.0);
                                }
                                CurrentlyEditing::None => {
                                    println!("none!");
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        // Return to the main screen and clear editing state
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Tab => {
                        // Cycle through editing fields
                        app.toggle_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::PublicKey => {
                                    app.command_input.push(value); // Adjust this based on your input fields
                                }
                                CurrentlyEditing::KeypairFile => {
                                    app.keypair_file.push(value);
                                }
                                CurrentlyEditing::Config => {
                                    app.config_value.push(value);
                                }
                                CurrentlyEditing::SolAmount => {
                                    let digit = value.to_digit(10);
                                    if let Some(digit) = digit {
                                        app.sol_amount = app.sol_amount * 10.0 + digit as f64;
                                    }
                                }
                                CurrentlyEditing::None => {
                                println!("none!");}
                            }
                        }
                    }
                    _ => {}
                },
                CurrentScreen::CommandLog => match key.code {
                    KeyCode::Esc => {
                        // Return to main screen from the command log screen
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
