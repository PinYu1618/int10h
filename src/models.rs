use tui::widgets::Widget;

pub trait App {
    type W: Widget;

    fn render() -> Self::W;
}