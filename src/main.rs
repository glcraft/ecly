use crossterm::event::{Event, poll};
use crossterm::style::Print;
use crossterm::{cursor, execute};
use std::io;

mod editor;
mod tui;

#[cfg(test)]
mod tests {
    #[test]
    fn bla() {
        
    }
}

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout();

    let code_rs = std::fs::read_to_string("./src/main.rs")?;
    
    let mut frame = tui::Frame::new();
    let text = "Hello world";
    let width = tui::terminal_size().0;
    let (left, right) = (-8, width as i16-2);
    let (mut pos, mut sens) = ((left,5), true);
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    'edit: loop {
        use crossterm::event::{read, KeyCode, KeyEvent};
        if poll(std::time::Duration::from_millis(200))? {
            match read()? {
                Event::Key(KeyEvent{ code:KeyCode::Esc, .. }) => break 'edit,
                // Event::Key(KeyEvent{code: KeyCode::Char(c), ..}) => execute!(stdout, Print(c))?,
                // Event::Key(KeyEvent{code: KeyCode::Backspace, ..}) => execute!(stdout, cursor::MoveLeft(1), Print(' '), cursor::MoveLeft(1))?,
                // Event::Mouse(event) => println!("{:?}", event),
                // Event::Resize(width, height) => println!("New size {}x{}", width, height),
                _ => ()
            }
        }
        if sens {
            pos.0+=1;
        } else {
            pos.0-=1;
        }
        if sens && pos.0 == right {
            sens=false;
        } else if  !sens && pos.0 == left {
            sens=true;
        }
        frame.clear()?;
        frame.write_str(pos, text)?;
        frame.flush()?;

    }
    execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;
}
