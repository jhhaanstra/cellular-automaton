mod rules;
mod grid;

use crate::grid::{Grid, Vector};
use crate::rules::{HasThreeNeighbours, HasTwoOrThreeNeighbours, Rule};
use clap::{command, value_parser, Arg, ArgAction};
use regex::Regex;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
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

struct Game {
    width: i32,
    height: i32,
    grid: Grid,
}

impl Game {
    fn new(width: i32, height: i32, grid: Grid) -> Game {
        Game { width, height, grid }
    }

    fn update(&mut self) {
        let mut new_state = HashSet::new();
        let occupied = self.grid.get_occupied_cells();
        let mut neighbours: HashSet<Vector> = HashSet::new();

        for cell in occupied {
            if HasTwoOrThreeNeighbours::complies(&cell, &self.grid) {
                new_state.insert(cell);
            }

            self.grid.get_neighbouring_cells(&cell)
                .iter()
                .for_each(|n| { neighbours.insert(*n); });
        }

        neighbours.into_iter()
            .filter(|n| HasThreeNeighbours::complies(&n, &self.grid))
            .for_each(|n| { new_state.insert(n); });

        self.grid = Grid { cells: new_state };
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut value: String = String::new();

        for y in 0..self.height {
            for x in 0..self.width {

                value.push(if self.grid.contains(&Vector::new(x, y)) {
                    'X'
                } else {
                    '.'
                });
            }
            value.push('\n');
        }

        write!(f, "{}", value)
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::{Grid, Vector};
    use crate::{parse_input, Game};
    use std::collections::HashSet;

    #[test]
    fn should_parse_input() {
        let actual = parse_input(&String::from("1,2;3,4;7,45"));
        let expected: HashSet<Vector> = vec!(Vector::new(1, 2), Vector::new(3, 4), Vector::new(7, 45)).into_iter().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn display_game_state() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![Vector::new(1, 2), Vector::new(2, 0)]);

        let game = Game::new(3, 3, grid);
        assert_eq!(String::from(format!("{}", game)), "..X\n...\n.X.\n");
    }

    #[test]
    fn should_handle_persist_cells() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            Vector::new(0, 0),
            Vector::new(0, 1),
            Vector::new(1, 0),
            Vector::new(1, 1),
        ]);

        let mut game = Game::new(3, 3, grid.clone());
        game.update();

        assert_eq!(game.grid, grid)
    }

    #[test]
    fn should_handle_oscillators() {
        let mut state1 = Grid::new();
        state1.add_cells(&vec![
            Vector::new(1, 0),
            Vector::new(1, 1),
            Vector::new(1, 2),
        ]);

        let mut state2 = Grid::new();
        state2.add_cells(&vec![
            Vector::new(0, 1),
            Vector::new(1, 1),
            Vector::new(2, 1),
        ]);

        let mut game = Game::new(3, 3, state1.clone());

        game.update();
        assert_eq!(game.grid, state2);

        game.update();
        assert_eq!(game.grid, state1);

        game.update();
        assert_eq!(game.grid, state2);

        game.update();
        assert_eq!(game.grid, state1);
    }
}
