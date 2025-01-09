use crate::grid::Grid;
use vector2d::Vector2D;

pub trait Rule {
    fn complies(point: Vector2D<i32>, grid: Grid) -> bool;
}

struct HasTwoOrThreeNeighbours;
struct HasThreeNeighbours;
impl Rule for HasTwoOrThreeNeighbours {
    fn complies(point: Vector2D<i32>, grid: Grid) -> bool {
        let nr_neighbours = grid.get_neighbours(&point).len();
        nr_neighbours == 2 || nr_neighbours == 3
    }
}

impl Rule for HasThreeNeighbours {
    fn complies(point: Vector2D<i32>, grid: Grid) -> bool {
        grid.get_neighbours(&point).len() == 3
    }
}

#[cfg(test)]
mod has_two_or_three_neighbours_tests {
    use crate::grid::Grid;
    use crate::rules::{HasTwoOrThreeNeighbours, Rule};
    use vector2d::Vector2D;

    #[test]
    fn when_two_neighbours_then_complies() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            &Vector2D::new(2, 2),
            &Vector2D::new(0, 0),
        ]);

        assert!(HasTwoOrThreeNeighbours::complies(Vector2D::new(1, 1), grid));
    }

    #[test]
    fn when_three_neighbours_then_complies() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            &Vector2D::new(2, 2),
            &Vector2D::new(0, 0),
            &Vector2D::new(2, 1),
        ]);

        assert!(HasTwoOrThreeNeighbours::complies(Vector2D::new(1, 1), grid));
    }

    #[test]
    fn when_less_than_two_neighbours_then_does_not_comply() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            &Vector2D::new(2, 2),
        ]);

        assert!(!HasTwoOrThreeNeighbours::complies(Vector2D::new(1, 1), grid));
    }

    #[test]
    fn when_more_than_three_neighbours_then_does_not_comply() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            &Vector2D::new(2, 2),
            &Vector2D::new(0, 0),
            &Vector2D::new(2, 1),
            &Vector2D::new(1, 2),
        ]);

        assert!(!HasTwoOrThreeNeighbours::complies(Vector2D::new(1, 1), grid));
    }
}

#[cfg(test)]
mod has_three_neighbours_tests {
    use crate::grid::Grid;
    use crate::rules::{HasThreeNeighbours, Rule};
    use vector2d::Vector2D;

    #[test]
    fn when_three_neighbours_then_complies() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            &Vector2D::new(2, 2),
            &Vector2D::new(0, 0),
            &Vector2D::new(2, 1),
        ]);

        assert!(HasThreeNeighbours::complies(Vector2D::new(1, 1), grid));
    }

    #[test]
    fn when_less_than_three_neighbours_then_does_not_comply() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            &Vector2D::new(2, 1),
            &Vector2D::new(1, 2),
        ]);

        assert!(!HasThreeNeighbours::complies(Vector2D::new(1, 1), grid));
    }

    #[test]
    fn when_more_than_three_neighbours_then_does_not_comply() {
        let mut grid = Grid::new();
        grid.add_cells(&vec![
            &Vector2D::new(2, 2),
            &Vector2D::new(0, 0),
            &Vector2D::new(2, 1),
            &Vector2D::new(1, 2),
        ]);

        assert!(!HasThreeNeighbours::complies(Vector2D::new(1, 1), grid));
    }
}