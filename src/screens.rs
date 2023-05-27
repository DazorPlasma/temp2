use crate::choice;
use crate::converter;
use converter::Temperature;
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
    let input: u8 = input.trim().parse().ok()?;
    Some(input)
}

fn clear_screen() {
    //print!("{}[2J", 27 as char);
    clearscreen::clear().expect("Unable to clear the screen!");
}

pub fn change_screen(target_screen: Screen, current_screen: &mut Screen) {
    clear_screen();
    *current_screen = target_screen;
    match target_screen {
        Screen::CelsiusConvert => cel_to_fahren(current_screen),
        Screen::FahrenheitConvert => fahren_to_cel(current_screen),
        Screen::Home => home(current_screen),
    }
}

const HOME_MESSAGE: &str = "Temperature Converter - between Celsius and Fahrenheit.\nOptions:\n1: Convert from Celsius to Fahrenheit;\n2: Convert from Fahrenheit to Celsius;\n0: Exit.";

pub fn home(current_screen: &mut Screen) {
    println!("{HOME_MESSAGE}");
    loop {
        let choice = match get_choice() {
            Some(val) => choice::translate_choice(&val),
            None => continue,
        };

        if choice::process_choice(&choice, current_screen) {
            return;
        }
    }
}

pub fn cel_to_fahren(current_screen: &mut Screen) {
    loop {
        print!("Temp in celsius: ");
        stdout().flush().expect("Couldn't flush output!");
        let input = get_input();
        let result: f64 = match input {
            Some(val) => {
                if val.to_lowercase() == EXIT_KEYWORD {
                    change_screen(Screen::Home, current_screen);
                    return;
                }
                match val.parse::<f64>() {
                    Ok(val2) => converter::temperature_convert(&val2, Temperature::Celsius),
                    Err(_) => continue,
                }
            }
            None => continue,
        };
        println!("Temp in fahrenheit is {result}.\n");
    }
}

pub fn fahren_to_cel(current_screen: &mut Screen) {
    loop {
        print!("Temp in fahrenheit: ");
        stdout().flush().expect("Couldn't flush output!");
        let input = get_input();
        let result: f64 = match input {
            Some(val) => {
                if val.to_lowercase() == EXIT_KEYWORD {
                    change_screen(Screen::Home, current_screen);
                    return;
                }
                match val.parse::<f64>() {
                    Ok(val2) => converter::temperature_convert(&val2, Temperature::Fahrenheit),
                    Err(_) => continue,
                }
            }
            None => continue,
        };
        println!("Temp in celsius is {result}.\n");
    }
}
