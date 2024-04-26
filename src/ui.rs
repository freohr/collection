use crate::app::{App, CurrentBlock, CurrentScreen};

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Clear, List, Paragraph, Wrap},
    Frame,
};

macro_rules! basic_block {
    () => {
        Block::bordered()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default())
    };
}

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.size());

    // Title
    let title = Paragraph::new(Text::styled("Collection", Style::default().fg(Color::Blue)))
        .block(basic_block!());

    frame.render_widget(title, chunks[0]);

    // Main View
    let list_display = Layout::horizontal([
        Constraint::Percentage(30),
        Constraint::Percentage(50),
        Constraint::Fill(1),
    ])
    .split(chunks[1]);

    let mut dir_list = Vec::<String>::new();
    for dir in app.dir_list() {
        let path = dir.to_str();

        if let Some(path) = path {
            dir_list.push(String::from(path));
        }
    }

    let dir_block = if let CurrentBlock::Directories = app.current_block {
        basic_block!().border_style(Style::new().green())
    } else {
        basic_block!()
    };

    let dir_list = List::new(dir_list).highlight_symbol(">").block(dir_block);

    frame.render_stateful_widget(dir_list, list_display[0], &mut app.selected_dir.clone());

    let mut file_list = Vec::<String>::new();
    for file in app.file_list() {
        let path = file.path();
        let string = path.to_str();

        if let Some(string) = string {
            file_list.push(String::from(string));
        }
    }

    let file_block = if let CurrentBlock::Files = app.current_block {
        basic_block!().border_style(Style::new().green())
    } else {
        basic_block!()
    };
    let file_list = List::new(file_list).highlight_symbol(">").block(file_block);

    frame.render_stateful_widget(file_list, list_display[1], &mut app.selected_file.clone());
    // Bottom Nav Row
    let current_nav_text = vec![match app.current_screen {
        CurrentScreen::Main => Span::styled("Files", Style::default().fg(Color::Green)),
        CurrentScreen::Add => Span::styled("Add Directory", Style::default().fg(Color::Yellow)),
        CurrentScreen::Exit => Span::styled("Exit", Style::default().fg(Color::LightRed)),
    }
    .to_owned()];

    let file_details = Paragraph::new("File Details").block(basic_block!());
    frame.render_widget(file_details, list_display[2]);

    // Footer

    let mode_footer = Paragraph::new(Line::from(current_nav_text)).block(basic_block!());

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (a) to add directory / ← h,↓ j,↑ k,→ l",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Add => Span::styled(
                "(Esc) to cancel / (Enter) to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Exit => Span::styled(
                "(q) to quit / (a) to add directory to collection",
                Style::default().fg(Color::Red),
            ),
        }
    };
    let key_footer = Paragraph::new(Line::from(current_keys_hint)).block(basic_block!());

    let footer_chunks =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_footer, footer_chunks[1]);

    // Rendering editing pop-up
    match app.current_screen {
        CurrentScreen::Add => {
            let popup_block = Block::default()
                .title("Enter a directory path")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let area = centered_rect(frame.size(), 60, 25);
            frame.render_widget(popup_block, area);

            let popup_chunks = Layout::horizontal([Constraint::Percentage(100)])
                .margin(1)
                .split(area);

            let dir_block = basic_block!();
            let dir_text = Paragraph::new(app.dir_input.clone()).block(dir_block);
            frame.render_widget(dir_text, popup_chunks[0]);
        }
        CurrentScreen::Exit => {
            frame.render_widget(Clear, frame.size());

            let popup_block = Block::default()
                .title("Y/N")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Would you like to output the collection as text? (y/n)",
                Style::default().fg(Color::Red),
            );

            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = centered_rect(frame.size(), 60, 25);
            frame.render_widget(exit_paragraph, area);
        }
        _ => {}
    }
}

/// # Usage
///
/// ```rust
/// let rect = centered_rect(f.size(), 50, 50);
/// ```
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
