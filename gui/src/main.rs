use core::grid::Grid;
use core::grid::Vector;
use core::game::Game;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(DrawTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_game, add_camera))
        .add_systems(Update, update_grid)
        .run();
}

const GRID_DIMENSION: f32 = 25.0;

#[derive(Component)]
struct View {
    pub game: Game,
    pub width: i32,
    pub height: i32,
}

#[derive(Resource)]
struct DrawTimer(Timer);


fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn update_grid(time: Res<Time>,
               mut tick: ResMut<DrawTimer>,
               commands: Commands,
               meshes: ResMut<Assets<Mesh>>,
               materials: ResMut<Assets<ColorMaterial>>,
               mut query: Query<&mut View>) {
    if tick.0.tick(time.delta()).just_finished() {
        let mut view = query.single_mut();
        let game = &mut view.game;
        game.update();

        draw_grid(commands, meshes, materials, &game.grid);
    }
}

fn draw_grid(mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<ColorMaterial>>,
             grid: &Grid) {
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

fn add_game(mut commands: Commands) {
    let mut grid = Grid::new();
    grid.add_cells(&vec![
        Vector::new(0, 0),
        Vector::new(1, 0),
        Vector::new(2, 0),
    ]);

    let game = Game {
        width: 100,
        height: 100,
        grid,
    };

    commands.spawn(View{game, width: 100, height: 100});
}