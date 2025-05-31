use bevy::{
    color::palettes::css::{PURPLE, TEAL},
    prelude::*,
};
use rand::Rng;

pub struct TDGame;

impl Plugin for TDGame {
    fn build(&self, app: &mut App) {
        // Add game
        app.init_resource::<Game>();
        app.add_systems(Startup, setup_scene);
    }
}

#[derive(Resource, Default)]
pub struct Game {
    board: Board,
}

pub struct Board {
    width: i32,
    height: i32,
    grid: Vec<Vec<Cell>>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            width: 20,
            height: 20,
            grid: Default::default(),
        }
    }
}

pub enum CellType {
    Road,
    Ground,
}

pub struct Cell {
    entity: Entity,
    width: f32,
    cell_type: CellType,
}

impl Cell {
    pub fn new(width: f32, entity: Entity, cell_type: CellType) -> Self {
        Self {
            width,
            entity,
            cell_type,
        }
    }
}

fn setup_scene(
    mut game: ResMut<Game>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            game.board.width as f32 / 2.,
            game.board.height as f32 / 2.,
            0.,
        ),
    ));

    let mut road_x = 0;
    let mut road_y = 0;
    let mut road = Vec::new();
    while road_x < game.board.width && road_y < game.board.height {
        if rand::rng().random_range(0.0..1.0) < 0.5 {
            road_x += 1;
        } else {
            road_y += 1;
        }
        road.push((road_x, road_y));
    }

    let mut grid = Vec::new();
    for i in 0..game.board.width {
        let mut row = Vec::new();
        for j in 0..game.board.height {
            let is_road = road.contains(&(i, j));
            let entity = commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(10., 10.))),
                MeshMaterial2d(materials.add(if is_road {
                    Color::from(PURPLE)
                } else {
                    Color::from(TEAL)
                })),
                Transform::from_xyz(
                    ((i - game.board.width / 2) * 20) as f32,
                    ((j - game.board.height / 2) * 20) as f32,
                    0.,
                ),
            ));
            row.push(Cell::new(
                10.,
                entity.id(),
                if is_road {
                    CellType::Road
                } else {
                    CellType::Ground
                },
            ));
        }
        grid.push(row);
    }

    game.board.grid = grid;
}
