use crate::choice;
use crate::converter;
use std::io::{stdin, stdout, Write};

const EXIT_KEYWORD: &str = "exit";

#[derive(Clone, Copy)]
pub enum Screen {
    CelsiusConvert,
    FahrenheitConvert,
    Home,
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    stdin().read_line(&mut input).ok()?;
    Some(input.trim().to_string())
}

fn get_choice() -> Option<u8> {
    let input = get_input()?;
    if input.to_lowercase() == "exit" {
        return Some(0);
    }
    let input: u8 = input.parse().ok()?;
    Some(input)
}

pub fn change_screen(target_screen: Screen, current_screen: &mut Screen) {
    clearscreen::clear().expect("Unable to clear the screen!");
    *current_screen = target_screen;
    match target_screen {
        Screen::CelsiusConvert => cel_to_fahren(current_screen),
        Screen::FahrenheitConvert => fahren_to_cel(current_screen),
        Screen::Home => home(current_screen),
    }
}

const HOME_MESSAGE: &str = "Temperature Converter - between Celsius and Fahrenheit.
Options:
1: Convert from Celsius to Fahrenheit;
2: Convert from Fahrenheit to Celsius;
0: Exit.";

pub fn home(current_screen: &mut Screen) {
    println!("{HOME_MESSAGE}");
    loop {
        let choice = match get_choice() {
            Some(val) => choice::Choice::try_from(val),
            None => continue,
        };

        match choice {
            Ok(val) => choice::process_choice(&val, current_screen),
            Err(_) => continue
        }
    }
}

pub fn cel_to_fahren(current_screen: &mut Screen) {
    loop {
        print!("Temp in celsius: ");
        stdout().flush().expect("Couldn't flush output!");
        let input: f64 = match get_input() {
            Some(val) => {
                if val.to_lowercase() == EXIT_KEYWORD {
                    change_screen(Screen::Home, current_screen);
                    return;
                }
                match val.parse::<f64>() {
                    Ok(val2) => val2,
                    Err(_) => continue,
                }
            }
            None => continue,
        };
        let val = converter::Celsius(input);
        println!("Temp in fahrenheit is {}.\n", val.to_fahrenheit());
    }
}

pub fn fahren_to_cel(current_screen: &mut Screen) {
    loop {
        print!("Temp in fahrenheit: ");
        stdout().flush().expect("Couldn't flush output!");
        let input: f64 = match get_input() {
            Some(val) => {
                if val.to_lowercase() == EXIT_KEYWORD {
                    change_screen(Screen::Home, current_screen);
                    return;
                }
                match val.parse::<f64>() {
                    Ok(val2) => val2,
                    Err(_) => continue,
                }
            }
            None => continue,
        };
        let val = converter::Fahrenheit(input);
        println!("Temp in celsius is {}.\n", val.to_celsius());
    }
}