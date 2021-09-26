use std::io;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use tui::text::{Spans, Span};
use tui::layout::{Alignment, Constraint, Direction, Layout};

#[cfg(test)]
mod tests{
    #[test]
    fn bla() {
        let a = 647;
        println!("a: {}", a);
        println!("a^a: {}", a^a);
        println!("a^a^a: {}", (a^a)^a);
    }
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let code_rs = std::fs::read_to_string("./src/main.rs")?;


    terminal.draw(|f| {
        let text = vec![
            Spans::from(vec![
                Span::raw("First"),
                Span::styled("line",Style::default().add_modifier(Modifier::ITALIC)),
                Span::raw("."),
            ]),
            Spans::from(Span::styled("Second line", Style::default().fg(Color::Red))),
        ];
        let p = Paragraph::new(text)
            .block(Block::default().title("Paragraph").borders(Borders::ALL))
            .style(Style::default().fg(Color::White).bg(Color::Black))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [Constraint::Ratio(1, 3), Constraint::Ratio(2, 3)].as_ref()
            )
            .split(f.size());
        let block = Block::default()
             .title("Block")
             .borders(Borders::ALL);
        f.render_widget(p, chunks[0]);
        let block = Block::default()
             .title("Block 2")
             .borders(Borders::ALL);
        f.render_widget(block, chunks[1]);
    })?;
    Ok(())
}