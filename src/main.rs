mod maze;

use bevy::prelude::*;
use maze::{Maze, SIZE};

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

const ARENA_WIDTH: usize = SIZE;
const ARENA_HEIGHT: usize = SIZE;

struct Atlases {
    cell: Handle<TextureAtlas>,
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "maze".to_string(),
            width: 400,
            height: 400,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(255., 255., 255.)))
        .add_resource({
            let mut m = Maze::new();
            m.regenerate();
            m
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(spawn_maze)
        .add_system(position_translation)
        .add_system(keyboard_input_system)
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    let texture_handle = asset_server.load("cell.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(4.0, 4.0), 1, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(Atlases {
        cell: texture_atlas_handle,
    });
}

fn spawn_maze(commands: &mut Commands, maze: ResMut<Maze>, atlases: Res<Atlases>) {
    for (y, row) in maze.iter_row().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: atlases.cell.clone_weak(),
                    sprite: TextureAtlasSprite::new((dbg!(*cell)) as u32),
                    transform: Transform {
                        translation: Default::default(),
                        rotation: Default::default(),
                        scale: Vec3::splat(25.),
                    },
                    ..Default::default()
                })
                .with(Position {
                    x: x as i32,
                    y: (SIZE - y - 1) as i32,
                })
                .with(Size::square(1.));
            println!("{}, {}, {}", x, y, cell);
        }
    }
}

fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos * tile_size - (bound_window / 2.) + (tile_size / 2.)
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&mut Position, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    let min_height_width = window.height().min(window.width());
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, min_height_width as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, min_height_width as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn keyboard_input_system(
    commands: &mut Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut maze: ResMut<Maze>,
    query: Query<(Entity, &TextureAtlasSprite)>,
) {
    if keyboard_input.pressed(KeyCode::Return) {
        for (entity, _) in query.iter() {
            commands.despawn(entity);
        }
        maze.regenerate();
        println!("return pressed");
    }
}
