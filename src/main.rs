use std::fmt;
use std::io::{stdout, Write};
use std::num::ParseIntError;
use std::{io::stdin, str::FromStr};

#[derive(PartialEq, Eq)]
enum Unit {
    CELSIUS,
    FAHRENHEIT,
    KELVIN,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Unit::CELSIUS => write!(f, "°C"),
            Unit::FAHRENHEIT => write!(f, "°F"),
            Unit::KELVIN => write!(f, "°K"),
        }
    }
}

impl FromStr for Unit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c" => Ok(Unit::CELSIUS),
            "celsius" => Ok(Unit::CELSIUS),
            "f" => Ok(Unit::FAHRENHEIT),
            "fahrenheit" => Ok(Unit::FAHRENHEIT),
            "k" => Ok(Unit::KELVIN),
            "kelvin" => Ok(Unit::KELVIN),
            _ => Err(()),
        }
    }
}

fn main() {
    println!("Temperature converter");
    print!("\nInsert the unit you want to convert from (C, F, K): ");
    stdout().flush().unwrap();

    #[allow(unused_mut)]
    let mut unit_from: Unit;

    loop {
        let result = input_parse_unit();
        match result {
            Ok(uf) => {
                unit_from = uf;
                println!("Converting from: {}", unit_from);
                break;
            }
            Err(_) => {
                print!("Invalid input, choose from: C, F, K: ");
                stdout().flush().unwrap();
            }
        }
    }

    print!("\nNow, the unit you want to convert to: ");
    stdout().flush().unwrap();

    #[allow(unused_mut)]
    let mut unit_to: Unit;

    loop {
        let result = input_parse_unit();
        match result {
            Ok(ut) => {
                unit_to = ut;
                if unit_to == unit_from {
                    print!("You cannot convert to the same unit, choose a different one: ");
                    stdout().flush().unwrap();
                    continue;
                }
                println!("Converting degrees {} to {}", unit_from, unit_to);
                break;
            }
            Err(_) => {
                print!("Invalid input, choose to: C, F, K: ");
                stdout().flush().unwrap();
            }
        }
    }

    print!("Now, enter the value you want to convert: ");
    stdout().flush().unwrap();

    #[allow(unused_mut)]
    let mut value: i32;

    loop {
        let result = input_parse_i32();
        match result {
            Ok(temp) => {
                value = temp;
                break;
            }
            Err(_) => {
                print!("That's not a valid temperature, try again: ");
                stdout().flush().unwrap();
                continue;
            }
        }
    }

    let converted = convert_temp(&value, &unit_from, &unit_to);
    println!(
        "\n{}{} is equal to {}{}",
        value, unit_from, converted, unit_to
    );
}

fn convert_temp(temp: &i32, from: &Unit, to: &Unit) -> i32 {
    #[allow(unused_mut)]
    let mut converted: i32;

    match from {
        Unit::CELSIUS => match to {
            Unit::FAHRENHEIT => converted = (temp * 9) / 5 + 32,
            Unit::KELVIN => converted = temp + 273,
            _ => converted = 0,
        },
        Unit::FAHRENHEIT => match to {
            Unit::CELSIUS => converted = (temp - 32) * 5 / 9,
            Unit::KELVIN => converted = (temp - 32) * 5 / 9 + 273,
            _ => converted = 0,
        },
        Unit::KELVIN => match to {
            Unit::CELSIUS => converted = temp - 273,
            Unit::FAHRENHEIT => converted = (temp - 273) * 9 / 5 + 32,
            _ => converted = 0,
        },
    };

    converted
}

fn get_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    buffer.trim().to_owned()
}

fn input_parse_unit() -> Result<Unit, ()> {
    let input = get_input();

    input.parse::<Unit>()
}

fn input_parse_i32() -> Result<i32, ParseIntError> {
    let input = get_input();

    input.parse::<i32>()
}