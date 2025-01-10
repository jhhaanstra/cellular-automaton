mod rules;
mod grid;

use crate::grid::{Grid, Vector};
use std::fmt::{Display, Formatter};

fn main() {
    println!("Hello, world!");
}

struct Game {
    width: i32,
    height: i32,
    grid: Grid
}

impl Game {
    fn new(dimensions: i32, grid: Grid) -> Game {
        Game {
            width: dimensions,
            height: dimensions,
            grid: grid,
        }
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
}
