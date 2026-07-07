use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

pub struct App {
    text: String,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(self.text.clone().bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let text = Text::from("Storage size is diplayed here!".yellow());

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut app = App {
        text: String::from(" Storage Usage Viewer "),
        exit: false,
    };

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = DefaultTerminal::new(backend)?;

    terminal.draw(|frame| {
        let title = Line::from(app.text.clone().bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let paragraph = Paragraph::new(app.text).block(block);
        frame.render_widget(paragraph, frame.size());
    })?;

    Ok(())
}
