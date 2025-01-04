use std::fmt::{Display, Formatter};
use vector2d::Vector2D;

fn main() {
    println!("Hello, world!");
}

struct Game {
    width: i32,
    height: i32,
    active_cells: Vec<Vector2D<i32>>,
}

impl Game {
    fn new(dimensions: i32) -> Game {
        Game {
            width: dimensions,
            height: dimensions,
            active_cells: Vec::new(),
        }
    }

    fn add_cell(&mut self, cell: Vector2D<i32>) {
        self.active_cells.push(cell);
    }

    fn add_cells(&mut self, cells: Vec<Vector2D<i32>>) {
        self.active_cells.extend(cells);
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut value: String = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                value.push(if self.active_cells.contains(&Vector2D::new(x, y)) {
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
    use crate::Game;
    use vector2d::Vector2D;

    #[test]
    fn display_game_state() {
        let mut game = Game::new(3);
        game.add_cells(vec![Vector2D { x: 1, y: 2 }, Vector2D { x: 2, y: 0 }]);

        assert_eq!(String::from(format!("{}", game)), "..X\n...\n.X.\n");
    }
}
