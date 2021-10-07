use crossterm::event::Event;
use super::{Frame, util::{self, Rect, TuiResult}};

mod text_scroll;
pub use text_scroll::TextScroll;

pub trait Widget {
    fn render(&self, frame: &mut Frame) -> TuiResult<()>;
    /// `zone` position and dimension are **inclusive**
    fn on_update_layout(&mut self, zone: Rect);
    /// Optionnel
    fn on_event(&mut self, _event: Event) {}
}
