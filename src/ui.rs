use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, CurrentScreen, ActiveTab};

pub fn cli_ui(f: &mut Frame, app: &App) {
    // Create the main layout with horizontal split
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.area());

    // Left Sidebar with tabs
    let left_panel = chunks[0];
    draw_left_sidebar(f, app, left_panel);

    // Right Panel and Command Log
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(80), Constraint::Length(3)].as_ref())
        .split(chunks[1]);

    let main_panel = right_chunks[0];
    let command_log_block = right_chunks[1];

    draw_main_panel(f, app, main_panel);
    draw_command_log(f, app, command_log_block);
}

// Draw the left sidebar (tabs and hint block)
fn draw_left_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let tabs = vec![
        ListItem::new("1. Network Connect"),
        ListItem::new("2. Accounts"),
        ListItem::new("3. Transactions"),
        ListItem::new("4. Actions"),
    ];

    // Highlight the active tab
    let tabs_list = List::new(tabs)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">");

    f.render_widget(tabs_list, area);
}

// Draw the main panel for displaying detailed content based on active tab
fn draw_main_panel(f: &mut Frame, app: &App, area: Rect) {
    let content = match app.current_screen {
        CurrentScreen::Main => match app.active_tab {
            ActiveTab::Network => format!("Network Connect Details: {}", app.config_value),
            ActiveTab::Accounts => {
                // Display account information dynamically
                if app.accounts.is_empty() {
                    "No accounts found".to_string()
                } else {
                    let account_details: Vec<String> = app.accounts.iter()
                        .map(|(addr, balance)| format!("Address: {}, Balance: {} SOL", addr, balance))
                        .collect();
                    account_details.join("\n")
                }
            },
            ActiveTab::Transactions => "Transaction Details".to_string(),
            ActiveTab::Actions => "Action Commands (keygen, config, etc.)".to_string(),
        },
        CurrentScreen::Editing => format!("Editing Mode: {:?}", app.currently_editing),
        CurrentScreen::Exiting => "Are you sure you want to exit? (y/n)".to_string(),
        _ => "Solana CLI Interface".to_string(),
    };

    let main_block = Paragraph::new(content)
        .block(Block::default().title("Main Panel").borders(Borders::ALL));

    f.render_widget(main_block, area);
}

// Draw the command log at the bottom
fn draw_command_log(f: &mut Frame, app: &App, area: Rect) {
    let log_content = if app.command_input.is_empty() {
        "Command Log: No recent commands".to_string()
    } else {
        format!("Last Command: {}", app.command_input)
    };

    let command_log = Paragraph::new(log_content)
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().title("Command Log").borders(Borders::ALL));

    f.render_widget(command_log, area);
}
