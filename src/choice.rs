use crate::screens::{change_screen, Screen};

pub enum Choice {
    CelsiusConverter,
    FahrenheitConverter,
    Exit,
}

pub enum ChoiceParseError {
    NoOption,
}

impl TryFrom<u8> for Choice {
    type Error = ChoiceParseError;
    fn try_from(choice: u8) -> Result<Self, Self::Error> {
        match choice {
            1 => Ok(Choice::CelsiusConverter),
            2 => Ok(Choice::FahrenheitConverter),
            0 => Ok(Choice::Exit),
            _ => Err(Self::Error::NoOption),
        }
    }
}

pub fn process_choice(choice: &Choice, current_screen: &mut Screen) {
    match choice {
        Choice::Exit => {
            if let Screen::Home = current_screen {
                std::process::exit(0);
            } else {
                change_screen(Screen::Home, current_screen);
            }
        }
        Choice::CelsiusConverter => change_screen(Screen::CelsiusConvert, current_screen),
        Choice::FahrenheitConverter => change_screen(Screen::FahrenheitConvert, current_screen),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choise_check1() {
        let c = Choice::try_from(124);
        match c {
            Ok(_) => panic!("Shouldn't exist!"),
            Err(_) => ()
        }
    }

    #[test]
    fn choise_check2() {
        let c = Choice::try_from(1);
        match c {
            Ok(val) => if let Choice::CelsiusConverter = val {
                ()
            } else {
                panic!("Wrong option!")
            },
            Err(_) => panic!()
        }
    }

    #[test]
    fn choise_check3() {
        let c = Choice::try_from(0);
        match c {
            Ok(val) => if let Choice::Exit = val {
                ()
            } else {
                panic!("Wrong option!")
            },
            Err(_) => panic!()
        }
    }
}