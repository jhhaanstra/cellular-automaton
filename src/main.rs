mod rules;
mod grid;

use crate::grid::{Grid, Vector};
use crate::rules::{HasThreeNeighbours, HasTwoOrThreeNeighbours, Rule};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

fn main() {
    println!("Hello, world!");
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
    use crate::grid::{Grid, Vector};
    use crate::Game;

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
