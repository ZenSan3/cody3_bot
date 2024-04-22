use ratatui::{prelude::*, widgets::*};

pub fn ui(frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Cody is Connected")
            .block(Block::default()
                .title("Cody3")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))),
        frame.size(),
    );
}