use tui::widgets::{StatefulWidget, Widget};

pub trait Model {
    type ModelWidget: Widget;
}

pub trait StatefulModel {
    type StatefulModelWidget: StatefulWidget;
}
