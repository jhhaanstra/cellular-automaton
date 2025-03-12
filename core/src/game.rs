use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use crate::grid::{Grid, Vector};

pub trait Rule {
    fn complies(point: &Vector, grid: &Grid) -> bool;
}

pub struct HasTwoOrThreeNeighbours;
pub struct HasThreeNeighbours;
impl Rule for HasTwoOrThreeNeighbours {
    fn complies(point: &Vector, grid: &Grid) -> bool {
        let nr_neighbours = grid.get_occupied_neighbours(&point).len();
        nr_neighbours == 2 || nr_neighbours == 3
    }
}

impl Rule for HasThreeNeighbours {
    fn complies(point: &Vector, grid: &Grid) -> bool {
        grid.get_occupied_neighbours(&point).len() == 3
    }
}

pub struct Game {
    pub width: i32,
    pub height: i32,
    pub grid: Grid,
}

impl Game {

    pub fn update(&mut self) {
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
mod GameTests {
    use crate::game::Game;
    use crate::grid::{Grid, Vector};

    #[test]
    fn should_handle_persist_cells() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            Vector::new(0, 0),
            Vector::new(0, 1),
            Vector::new(1, 0),
            Vector::new(1, 1),
        ]);

        let mut game = Game{ width: 3, height: 3, grid: grid.clone() };
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

        let mut game = Game{ width: 3, height: 3, grid: state1.clone() };

        game.update();
        assert_eq!(game.grid, state2);

        game.update();
        assert_eq!(game.grid, state1);

        game.update();
        assert_eq!(game.grid, state2);

        game.update();
        assert_eq!(game.grid, state1);
    }

    #[test]
    fn display_game_state() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![Vector::new(1, 2), Vector::new(2, 0)]);

        let game = Game{ width: 3, height: 3, grid };
        assert_eq!(String::from(format!("{}", game)), "..X\n...\n.X.\n");
    }
}

#[cfg(test)]
mod has_two_or_three_neighbours_tests {
    use crate::grid::{Grid, Vector};
    use crate::game::{HasTwoOrThreeNeighbours, Rule};

    #[test]
    fn when_two_neighbours_then_complies() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            Vector::new(2, 2),
            Vector::new(0, 0),
        ]);

        assert!(HasTwoOrThreeNeighbours::complies(&Vector::new(1, 1), &grid));
    }

    #[test]
    fn when_three_neighbours_then_complies() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            Vector::new(2, 2),
            Vector::new(0, 0),
            Vector::new(2, 1),
        ]);

        assert!(HasTwoOrThreeNeighbours::complies(&Vector::new(1, 1), &grid));
    }

    #[test]
    fn when_less_than_two_neighbours_then_does_not_comply() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            Vector::new(2, 2),
        ]);

        assert!(!HasTwoOrThreeNeighbours::complies(&Vector::new(1, 1), &grid));
    }

    #[test]
    fn when_more_than_three_neighbours_then_does_not_comply() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            Vector::new(2, 2),
            Vector::new(0, 0),
            Vector::new(2, 1),
            Vector::new(1, 2),
        ]);

        assert!(!HasTwoOrThreeNeighbours::complies(&Vector::new(1, 1), &grid));
    }
}

#[cfg(test)]
mod has_three_neighbours_tests {
    use crate::grid::{Grid, Vector};
    use crate::game::{HasThreeNeighbours, Rule};

    #[test]
    fn when_three_neighbours_then_complies() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            Vector::new(2, 2),
            Vector::new(0, 0),
            Vector::new(2, 1),
        ]);

        assert!(HasThreeNeighbours::complies(&Vector::new(1, 1), &grid));
    }

    #[test]
    fn when_less_than_three_neighbours_then_does_not_comply() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            Vector::new(2, 1),
            Vector::new(1, 2),
        ]);

        assert!(!HasThreeNeighbours::complies(&Vector::new(1, 1), &grid));
    }

    #[test]
    fn when_more_than_three_neighbours_then_does_not_comply() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            Vector::new(2, 2),
            Vector::new(0, 0),
            Vector::new(2, 1),
            Vector::new(1, 2),
        ]);

        assert!(!HasThreeNeighbours::complies(&Vector::new(1, 1), &grid));
    }
}