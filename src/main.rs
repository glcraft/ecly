
mod log;

use crossterm::{event::Event, execute};
use clui::widget::Paragraph;
use std::io::Write;
use clui::widget::Widget;

// mod editor;


#[cfg(test)]
mod tests {
    #[test]
    fn bla() {
        
    }
}

fn main() -> Result<(), std::io::Error> {
    use clui::widget::{Alignments, AlignmentX, AlignmentY};
    let mut stdout = std::io::stdout();
    let mut frame = clui::Frame::new();
    let mut txt_scroll = Paragraph::new();
    txt_scroll.set_text("c'est un paragraphe\nPas un gros paragraphe\nMais quand même!\nToute facon, on s'en fout, c'est pour le test");
    txt_scroll.set_alignment(Alignments(AlignmentX::Right, AlignmentY::Bottom));
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    'edit: loop {
        use crossterm::event::{read, KeyCode, KeyEvent};
        // if poll(std::time::Duration::from_millis(200))? 
        {
            match read()? {
                Event::Key(KeyEvent{ code:KeyCode::Esc, .. }) => break 'edit,
                // Event::Key(KeyEvent{code: KeyCode::Char(c), ..}) => execute!(stdout, Print(c))?,
                // Event::Key(KeyEvent{code: KeyCode::Backspace, ..}) => execute!(stdout, cursor::MoveLeft(1), Print(' '), cursor::MoveLeft(1))?,
                // Event::Mouse(event) => println!("{:?}", event),
                // Event::Resize(width, height) => println!("New size {}x{}", width, height),
                _ => ()
            }
        }
        // println!()
        txt_scroll.on_update_layout(frame.get_screen());
        frame.clear()?;
        txt_scroll.render(&mut frame)?;
        frame.flush()?;
        // log_writeln!("fslkjdfq")?;
    }
    execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;
    log::flush()
}
