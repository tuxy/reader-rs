use std::io;
use ratatui::{
    crossterm::event::{
        self,
        KeyCode,
    }, layout::Margin, text::ToText, widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap
    }, DefaultTerminal
};

pub fn area(mut terminal: DefaultTerminal, content: String) -> io::Result<()> {
    // Simply represents current scroll location
    let mut vertical_scroll = 0; // from app state
    loop {
        let _ = terminal.draw(|frame| {

            let text = tui_markdown::from_str(&content);
            // Add text and content here
            let paragraph = Paragraph::new(text.to_text())
                .scroll((vertical_scroll as u16, 0))
                .wrap(Wrap { trim: true })
                .block(Block::new().borders(Borders::RIGHT)); // to show a background for the scrollbar
            
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    
            let mut scrollbar_state = ScrollbarState::new(text.height()).position(vertical_scroll);
    
            let area = frame.area();
            // Render content first
            frame.render_widget(paragraph, area);
            // Stateful scrollbar render
            frame.render_stateful_widget(
                scrollbar,
                area.inner(Margin {
                    // Taken from example, adds margin to scrollbar
                    vertical: 1,
                    horizontal: 0,
                }),
                &mut scrollbar_state,
            );
        });

        //
        // HANDLING Key input (Quit, up, down, left, right)
        //

        // Breaks loop

        if let event::Event::Key(key) = event::read()? {
            // Handle 'q' or Esc key for leaving program
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                KeyCode::Down => vertical_scroll += 1,
                KeyCode::Up => {
                    match vertical_scroll {
                        0 => vertical_scroll = 0,
                        _ => vertical_scroll -= 1,
                    }
                }
                _ => ()
            }
        }
    }
}