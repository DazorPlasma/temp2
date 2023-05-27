mod choice;
mod converter;
mod screens;

fn main() {
    let mut current_screen = screens::Screen::Home;
    screens::change_screen(screens::Screen::Home, &mut current_screen);
}
