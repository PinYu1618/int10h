use tui::widgets::Widget;

mod home;
mod pets;

pub trait App {
    type W: Widget;

    fn render() -> Self::W;
}

pub use home::*;
pub use pets::*;