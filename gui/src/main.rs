use core::grid::Grid;
use core::grid::Vector;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

const GRID_DIMENSION: f32 = 25.0;

#[derive(Component)]
struct View {
    pub grid: Grid,
    pub width: i32,
    pub height: i32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let mut grid = Grid::new();
    grid.add_cells(&vec![
        Vector::new(0, 0),
        Vector::new(1, 0),
        Vector::new(2, 0),
    ]);

    for x in -10..10 {
        for y in -10..10 {
            let shape = meshes.add(Rectangle::new(GRID_DIMENSION, GRID_DIMENSION));

            let color = match grid.contains(&Vector::new(x, y)) {
                true => Color::srgb(0.0, 0.0, 0.0),
                false => Color::srgb(1.0, 1.0, 1.0)
            };

            commands.spawn((
                Mesh2d(shape),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(
                    GRID_DIMENSION * x as f32,
                    GRID_DIMENSION * y as f32,
                    0.0,
                ),
            ));
        }
    }
}

fn add_grid(mut commands: Commands) {
    let mut grid = Grid::new();
    grid.add_cells(&vec![
        Vector::new(0, 0),
        Vector::new(1, 0),
        Vector::new(2, 0),
    ]);

    commands.spawn(View{grid, width: 100, height: 100});
}