use tui::widgets::Widget;

mod home;

pub trait App {
    type W: Widget;

    fn render() -> Self::W;
}

pub use home::*;