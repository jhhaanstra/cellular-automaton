use core::grid::{Grid, Vector};
use core::game::{Game};
use clap::{command, value_parser, Arg, ArgAction};
use regex::Regex;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

fn cli() -> Game{
    let matches = command!()
        .about("Simulation of Conway's Game of Life.")
        .version("0.1")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("List of cells that should be alive when the simulation starts. Concatenated by a semicolon. E.g. 1,2;3,4 maps to the cells x1,y2 and x3,y4 to be alive when the simulation starts.")
                .required(true)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("width")
                .long("width")
                .value_parser(value_parser!(i32))
                .default_value("120")
                .help("Width of the viewport in the terminal")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("height")
                .long("height")
                .value_parser(value_parser!(i32))
                .default_value("60")
                .help("Height of the viewport in the terminal")
                .action(ArgAction::Append)
        )
        .get_matches();

    let width = *matches
        .get_one::<i32>("width")
        .expect("default ensures there is always a value");

    let height = *matches
        .get_one::<i32>("height")
        .expect("default ensures there is always a value");

    let grid = matches
        .get_one::<String>("input")
        .map(parse_input)
        .map(|cells| Grid::from_cells(HashSet::from(cells)))
        .expect("default ensures there is always a value");

    Game { width, height, grid }
}

fn main() {
    let mut game = cli();
    let running: bool = true;

    while running {
        std::process::Command::new("clear").status().unwrap();
        print!("{}", game);
        thread::sleep(Duration::from_millis(100));
        game.update();
    }
}

fn parse_input(input: &String) -> HashSet<Vector> {
    let vector_regex = Regex::new("(?P<x>\\d+),(?P<y>\\d+)").unwrap();
    input.split(";")
        .into_iter()
        .map(|vec| vector_regex.captures(vec).unwrap())
        .map(|cap| Vector::new(
            cap["x"].parse().expect("Invalid input provided, expected format is x,y;x,y..."),
            cap["y"].parse().expect("Invalid input provided, expected format is x,y;x,y..."))
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use core::grid::Vector;
    use crate::parse_input;
    use std::collections::HashSet;

    #[test]
    fn should_parse_input() {
        let actual = parse_input(&String::from("1,2;3,4;7,45"));
        let expected: HashSet<Vector> = vec!(Vector::new(1, 2), Vector::new(3, 4), Vector::new(7, 45)).into_iter().collect();
        assert_eq!(actual, expected);
    }
}
