use std::collections::HashSet;
use std::ops::Add;

pub struct Grid {
    cells: HashSet<Vector>
}

impl Grid {
    const DIRECTIONS: [Vector; 8] = [
        Vector{y: -1, x: -1},
        Vector{y: -1, x: 0},
        Vector{y: -1, x: 1},
        Vector{y: 0,  x: -1},
        Vector{y: 0,  x: 1},
        Vector{y: 1,  x: -1},
        Vector{y: 1,  x: 0},
        Vector{y: 1,  x: 1},
    ];

    pub fn new() -> Grid {
        Grid{cells: HashSet::new()}
    }

    pub fn add_cells(&mut self, cells: &Vec<Vector>) {
        cells.iter().for_each(|cell| self.add_cell(cell));
    }

    pub fn add_cell(&mut self, cell: &Vector) {
        self.cells.insert(cell.clone());
    }

    pub fn remove_cell(&mut self, cell: &Vector) {
        self.cells.remove(cell);
    }

    pub fn contains(&self, cell: &Vector) -> bool {
        self.cells.contains(cell)
    }

    pub fn get_neighbours(&self, cell: &Vector) -> Vec<Vector> {
        Self::DIRECTIONS.iter()
            .map(|direction| cell + direction)
            .filter(|point| self.cells.contains(point))
            .collect()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vector {
    x: i32,
    y: i32
}

impl Vector {

    pub fn new(x: i32, y: i32) -> Vector {
        Vector{x, y}
    }

}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}

impl Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[cfg(test)]
mod vector_tests {
    use crate::grid::Vector;

    #[test]
    fn should_add() {
        let a = Vector::new(1, 2);
        let b = Vector::new(3, 4);
        assert_eq!(Vector::new(4, 6), a + b);
    }

    #[test]
    fn should_add_with_reference() {
        let a = Vector::new(1, 2);
        let b = &Vector::new(3, 4);
        assert_eq!(Vector::new(4, 6), a + b);
    }

    #[test]
    fn should_add_with_both_references() {
        let a = &Vector::new(1, 2);
        let b = &Vector::new(3, 4);
        assert_eq!(Vector::new(4, 6), a + b);
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::grid::{Grid, Vector};

    #[test]
    fn should_add_cell() {
        let mut grid = Grid::new();
        let point = &Vector { x: 1, y: 2 };
        grid.add_cell(point);

        assert!(grid.contains(point));
    }

    #[test]
    fn should_remove_cell() {
        let mut grid = Grid::new();
        let point = &Vector { x: 1, y: 2 };
        grid.add_cell(point);
        grid.remove_cell(point);

        assert!(!grid.contains(point));
    }

    #[test]
    fn should_find_no_neighbours() {
        let grid = Grid::new();
        let point = &Vector { x: 1, y: 1 };
        assert_eq!(grid.get_neighbours(point), vec![]);
    }

    #[test]
    fn should_find_neighbours() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            Vector::new(0, 2),
            Vector::new(1, 2),
            Vector::new(2, 2),
            Vector::new(0, 1),
            Vector::new(2, 1),
            Vector::new(0, 0),
            Vector::new(1, 0),
            Vector::new(2, 0),
        ]);
        let point = Vector { x: 1, y: 1 };
        let neighbours = grid.get_neighbours(&point);
        assert_eq!(neighbours.len(), 8);
        assert!(neighbours.contains(&Vector::new(0, 2)));
        assert!(neighbours.contains(&Vector::new(1, 2)));
        assert!(neighbours.contains(&Vector::new(2, 2)));
        assert!(neighbours.contains(&Vector::new(0, 1)));
        assert!(neighbours.contains(&Vector::new(2, 1)));
        assert!(neighbours.contains(&Vector::new(0, 0)));
        assert!(neighbours.contains(&Vector::new(1, 0)));
        assert!(neighbours.contains(&Vector::new(2, 0)));
    }
}