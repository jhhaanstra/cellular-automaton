use std::collections::HashMap;
use vector2d::Vector2D;

pub struct Grid {
    cells: HashMap<(i32, i32), bool>
}

impl Grid {
    const DIRECTIONS: [Vector2D<i32>; 8] = [
        Vector2D{y: -1, x: -1},
        Vector2D{y: -1, x: 0},
        Vector2D{y: -1, x: 1},
        Vector2D{y: 0,  x: -1},
        Vector2D{y: 0,  x: 1},
        Vector2D{y: 1,  x: -1},
        Vector2D{y: 1,  x: 0},
        Vector2D{y: 1,  x: 1},
    ];

    pub fn new() -> Grid {
        Grid{cells: HashMap::new()}
    }

    pub fn add_cells(&mut self, cells: &Vec<&Vector2D<i32>>) {
        cells.iter().for_each(|cell| self.add_cell(cell));
    }

    pub fn add_cell(&mut self, cell: &Vector2D<i32>) {
        self.place(cell, true)
    }

    pub fn remove_cell(&mut self, cell: &Vector2D<i32>) {
        self.cells.remove(&Self::to_tuple(cell));
    }

    fn place(&mut self, cell: &Vector2D<i32>, value: bool) {
        self.cells.insert(Self::to_tuple(cell), value);
    }

    pub fn contains(&self, cell: &Vector2D<i32>) -> bool {
        self.cells.get(&Self::to_tuple(cell)).unwrap_or(&false).clone()
    }

    pub fn get_neighbours(&self, cell: &Vector2D<i32>) -> Vec<Vector2D<i32>> {
        Self::DIRECTIONS.iter()
            .map(|direction| cell + &direction)
            .filter(|point| *self.cells.get(&Self::to_tuple(&point))
                .unwrap_or(&false))
            .collect()
    }

    fn to_tuple(point: &Vector2D<i32>) -> (i32, i32) {
        (point.x, point.y)
    }
}

#[cfg(test)]
mod tests {
    use vector2d::Vector2D;
    use crate::grid::Grid;

    #[test]
    fn should_add_cell() {
        let mut grid = Grid::new();
        let point = &Vector2D { x: 1, y: 2 };
        grid.add_cell(point);

        assert!(grid.contains(point));
    }

    #[test]
    fn should_remove_cell() {
        let mut grid = Grid::new();
        let point = &Vector2D { x: 1, y: 2 };
        grid.add_cell(point);
        grid.remove_cell(point);

        assert!(!grid.contains(point));
    }

    #[test]
    fn should_find_no_neighbours() {
        let mut grid = Grid::new();
        let point = &Vector2D { x: 1, y: 1 };
        assert_eq!(grid.get_neighbours(point), vec![]);
    }

    #[test]
    fn should_find_neighbours() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            &Vector2D::new(0, 2),
            &Vector2D::new(1, 2),
            &Vector2D::new(2, 2),
            &Vector2D::new(0, 1),
            &Vector2D::new(2, 1),
            &Vector2D::new(0, 0),
            &Vector2D::new(1, 0),
            &Vector2D::new(2, 0),
        ]);
        let point = &Vector2D { x: 1, y: 1 };
        let neighbours = grid.get_neighbours(point);
        assert_eq!(neighbours.len(), 8);
        assert!(neighbours.contains(&Vector2D::new(0, 2)));
        assert!(neighbours.contains(&Vector2D::new(1, 2)));
        assert!(neighbours.contains(&Vector2D::new(2, 2)));
        assert!(neighbours.contains(&Vector2D::new(0, 1)));
        assert!(neighbours.contains(&Vector2D::new(2, 1)));
        assert!(neighbours.contains(&Vector2D::new(0, 0)));
        assert!(neighbours.contains(&Vector2D::new(1, 0)));
        assert!(neighbours.contains(&Vector2D::new(2, 0)));
    }
}