use crate::screens::{change_screen, Screen};

pub enum Choice {
    CelsiusConverter,
    FahrenheitConverter,
    Exit,
    Nothing,
}

pub fn translate_choice(choice: &u8) -> Choice {
    match choice {
        1 => Choice::CelsiusConverter,
        2 => Choice::FahrenheitConverter,
        0 => Choice::Exit,
        _ => Choice::Nothing,
    }
}

pub fn process_choice(choice: &Choice, current_screen: &mut Screen) -> bool {
    match choice {
        Choice::Exit => {
            if let Screen::Home = current_screen {
                std::process::exit(0);
            } else {
                change_screen(Screen::Home, current_screen);
                return true;
            }
        }
        Choice::CelsiusConverter => change_screen(Screen::CelsiusConvert, current_screen),
        Choice::FahrenheitConverter => change_screen(Screen::FahrenheitConvert, current_screen),
        Choice::Nothing => return false,
    }

    true
}
