mod rules;
mod grid;

use crate::grid::{Grid, Vector};
use crate::rules::{HasThreeNeighbours, HasTwoOrThreeNeighbours, Rule};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::thread;
use std::time::Duration;
use clap::{Arg, ArgAction, Command};
use clap::builder::TypedValueParser;
use regex::Regex;

// For later
fn cli() -> Command {
    Command::new("git")
        .about("A fictional versioning CLI")
        .version("0.1")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("List of cells that should be alive when the simulation starts. Concatenated by a semicolon. E.g. 1,2;3,4 maps to the cells x1,y2 and x3,y4 to be alive when the simulation starts.")
                // .required(true)
                .default_value("1,1;1,2;1,3")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("dimensions")
                .short('d') // Short flag, e.g., -n
                .long("dimensions") // Long flag, e.g., --name
                .value_name("DIMENSIONS")
                .help("Dimensions of the grid in the display (e.g. 10x10 when 10 is provided)")
                .action(ArgAction::Set)
        )
}

fn main() {
    let grid = Grid {
        cells: parse_input(&String::from("20,19;19,20;20,20;20,21;21,21"))
    };

    let mut game = Game::new(40, grid);
    let running: bool = true;

    while running {
        std::process::Command::new("clear").status().unwrap();
        print!("{}", game);
        thread::sleep(Duration::from_millis(50));
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
    fn new(dimensions: i32, grid: Grid) -> Game {
        Game {
            width: dimensions,
            height: dimensions,
            grid,
        }
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
    use std::collections::HashSet;
    use crate::grid::{Grid, Vector};
    use crate::{parse_input, Game};

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

        let game = Game::new(3, grid);
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

        let mut game = Game::new(3, grid.clone());
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

        let mut game = Game::new(3, state1.clone());

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
