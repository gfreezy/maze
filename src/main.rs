mod maze;

use bevy::prelude::*;
use bevy::utils::tracing::Event;
use maze::{Maze, SIZE};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
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

struct RegenerateEvent;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "maze".to_string(),
            width: 400,
            height: 400,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(255., 255., 255.)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(make_maze)
        .add_system(position_translation)
        .add_system(keyboard_input_system)
        .add_event::<RegenerateEvent>()
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut regenerate_events: ResMut<Events<RegenerateEvent>>,
) {
    commands.spawn(Camera2dBundle::default());
    let texture_handle = asset_server.load("cell.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(4.0, 4.0), 1, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for y in 0..SIZE {
        for x in 0..SIZE {
            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_scale(Vec3::splat(25.)),
                    ..Default::default()
                })
                .with(Position {
                    x: x as i32,
                    y: y as i32,
                });
        }
    }

    commands.insert_resource(Atlases {
        cell: texture_atlas_handle.clone(),
    });

    regenerate_events.send(RegenerateEvent);
}

fn make_maze(
    mut query: Query<(&Position, &mut TextureAtlasSprite)>,
    mut regenerate_reader: Local<EventReader<RegenerateEvent>>,
    regenerate_events: Res<Events<RegenerateEvent>>,
) {
    if regenerate_reader.iter(&regenerate_events).next().is_some() {
        let maze = Maze::new();
        for (pos, mut sprite) in query.iter_mut() {
            if let Some(cell) = maze.get_cell(pos.x as usize, pos.y as usize) {
                sprite.index = cell as u32;
                println!("cell: {:?}, index: {:b}", pos, cell);
            }
        }
    }
}

fn convert((x, y): (f32, f32), bound_window: f32, bound_game: f32) -> (f32, f32) {
    let tile_size = bound_window / bound_game;
    ((x - 1.5) * tile_size, (1.5 - y) * tile_size)
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&mut Position, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    let min_height_width = window.height().min(window.width());
    for (pos, mut transform) in q.iter_mut() {
        let (x, y) = convert(
            (pos.x as f32, pos.y as f32),
            min_height_width as f32,
            SIZE as f32,
        );
        transform.translation = Vec3::new(x, y, 0.0);
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut regenerate_events: ResMut<Events<RegenerateEvent>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        regenerate_events.send(RegenerateEvent);
        println!("pressed");
    }
}

#[cfg(test)]
mod tests {
    use crate::convert;
    use crate::maze::SIZE;

    fn check((x, y): (isize, isize), (x2, y2): (isize, isize)) {
        assert_eq!(
            convert((x as f32, y as f32), 400., SIZE as f32),
            (x2 as f32, y2 as f32)
        );
    }

    #[test]
    fn test_convert_pos() {
        check((0, 0), (-150, 150));
        check((0, 1), (-150, 50));
        check((1, 0), (-50, 150));
    }
}
