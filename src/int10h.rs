pub struct Int10h {
    pub should_quit: bool,
    pub activate_tab: MenuItem,
}

#[derive(Clone, Copy, Debug)]
pub enum MenuItem {
    Home,
    MenuItem1,
}

impl MenuItem {
    pub fn as_u32(&self) -> u32 {
        match self {
            Self::Home => 0,
            Self::MenuItem1 => 1,
        }
    }
}

impl Int10h {
    pub const TITLES: [&'static str; 2] = ["Home", "MenuItem1"];

    pub fn on_ctrl_c(&mut self) {
        self.should_quit = true;
    }

    pub fn on_press(&self, _x: u16, _y: u16) {}

    pub fn on_release(&self, _x: u16, _y: u16) {}

    pub fn on_hold(&self, _x: u16, _y: u16) {}
}

impl Default for Int10h {
    fn default() -> Self {
        Self {
            should_quit: false,
            activate_tab: MenuItem::default(),
        }
    }
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> Self {
        match input {
            MenuItem::Home => 0,
            MenuItem::MenuItem1 => 1,
        }
    }
}

impl Default for MenuItem {
    fn default() -> Self {
        Self::Home
    }
}