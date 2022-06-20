//use cursive::theme::BaseColor;
//use cursive::theme::PaletteColor;
//use cursive::theme::Color;

mod bios;

use bios::Bios;

fn main() {
    Bios::default().run();

/*
    let mut siv = cursive::default();

    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    siv.set_theme(theme);

    siv.run();
*/
}

// SeaBIOS (version rel-1.12.1-0-ga5cab58)
// Booting from Hard Disk...
// Starting MS-DOS...