use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use vector2d::Vector2D;

fn main() {
    println!("Hello, world!");
}

struct Game {
    grid: HashMap<(i32, i32), bool>
}

impl Game {
    fn new(dimensions: i32) -> Game {
        let mut grid = HashMap::new();
        Game {
            grid,
        }
    }

    fn add_cells(&mut self, cells: &Vec<&Vector2D<i32>>) {
        cells.iter().for_each(|cell| self.add_cell(cell));
    }

    fn add_cell(&mut self, cell: &Vector2D<i32>) {
        self.place(cell, true)
    }

    fn remove_cell(&mut self, cell: &Vector2D<i32>) {
        self.place(cell, false)
    }

    fn place(&mut self, cell: &Vector2D<i32>, value: bool) {
        self.grid.insert(Self::to_tuple(cell), value);
    }

    fn is_alive(&self, cell: &Vector2D<i32>) -> bool {
        self.grid.get(&Self::to_tuple(cell)).unwrap_or(&false).clone()
    }

    fn to_tuple(point: &Vector2D<i32>) -> (i32, i32) {
        (point.x, point.y)
    }
}



impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut value: String = String::new();

        let min_x = self.grid.keys()
            .map(|key| key.0)
            .min()
            .unwrap_or_else(|| 0);
        let min_y = self.grid.keys()
            .map(|key| key.1)
            .min()
            .unwrap_or_else(|| 0);
        let max_x = self.grid.keys()
            .map(|key| key.1)
            .max()
            .unwrap_or_else(|| 0);
        let max_y = self.grid.keys()
            .map(|key| key.1)
            .max()
            .unwrap_or_else(|| 0);

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                value.push(if self.is_alive(&Vector2D::new(x, y)) {
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
        game.add_cells(&vec![&Vector2D { x: 1, y: 2 }, &Vector2D { x: 2, y: 0 }]);

        assert_eq!(String::from(format!("{}", game)), ".X\n..\nX.\n");
    }

    #[test]
    fn should_add_cell() {
        let mut game = Game::new(3);
        let point = &Vector2D { x: 1, y: 2 };
        game.add_cell(point);

        assert!(game.is_alive(point));
    }

    #[test]
    fn should_remove_cell() {
        let mut game = Game::new(3);
        let point = &Vector2D { x: 1, y: 2 };
        game.add_cell(point);
        game.remove_cell(point);

        assert!(!game.is_alive(point));
    }
}
