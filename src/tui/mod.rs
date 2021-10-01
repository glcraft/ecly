use std::io::{Stdout, Write};

use crossterm::{cursor::MoveTo, queue, style::Print, terminal::{Clear, ClearType}};

pub mod widget;
pub mod event_listener;

pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}
pub struct Frame {
    screen: Rect,
    term: Stdout
}

pub fn terminal_size() -> (u16,u16) {
    use terminal_size::{Height, Width};
    terminal_size::terminal_size().and_then( |(Width(w),Height(h))| Some((w,h))).or_else(|| Some((50,25))).unwrap()
}
#[inline]
fn min<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1<v2 {v1} else {v2}
}
#[inline]
fn max<T: PartialOrd>(v1: T, v2: T) -> T {
    if v1>v2 {v1} else {v2}
}
#[inline]
fn clamp<T: PartialOrd>(v_min: T, v_max: T, x: T) -> T {
    max(v_min, min(v_max, x))
}


impl Frame {
    pub fn new() -> Self {
        let (w,h) = terminal_size();
        Self {
            screen: Rect { x: 0, y: 0, w, h },
            term: std::io::stdout(),
        }
    }
    pub fn is_in(&self, pos: (i16, i16)) -> bool {
        let (x,y) = pos;
        x>=self.screen.x as i16 && x<(self.screen.x+self.screen.w) as i16 && 
        y>=self.screen.y as i16 && y<(self.screen.y+self.screen.h) as i16
    }
    pub fn clear(&mut self)  -> Result<(), std::io::Error> {
        queue!(self.term, Clear(ClearType::All))
    }
    pub fn flush(&mut self)  -> Result<(), std::io::Error> {
        self.term.flush()
    }
    pub fn write_str(&mut self, pos: (i16, i16), text: &str) -> Result<(), std::io::Error>{
        let (x,y) = pos;
        if y >= self.screen.y as i16 && y < (self.screen.y+self.screen.h) as i16 {
            let offset = (
                self.screen.x as i16 - x,
                (self.screen.x + self.screen.w) as i16 - 1 - x,
            );
            let text_offset = (clamp(0, text.len() as i16, offset.0) as usize, clamp(0, text.len() as i16, offset.1) as usize);
            let text1= &text[text_offset.0..text_offset.1];
            if text1.len() > 0 {
                let x1 = clamp(self.screen.x  as i16, (self.screen.x+self.screen.w) as i16, x);
                return queue!(self.term, MoveTo(x1 as u16, y as u16), Print(text1))
            }
        } 
        Ok(())
    }
    pub fn write_char(&mut self, pos: (i16, i16), ch: char) -> Result<(), std::io::Error> {
        if self.is_in(pos) {
            queue!(self.term, MoveTo(pos.0 as u16, pos.1 as u16), Print(ch))
        } else {
            Ok(())
        }
    }
}