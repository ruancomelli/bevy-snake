use bevy::{
    prelude::*,
    time::common_conditions::on_timer,
    window::{PresentMode, WindowResolution},
};
use bevy_snake::plugins::{camera::CameraPlugin, quit::QuitPlugin};
use rand::random;
use std::time::Duration;

const BACKGROUND_COLOR: Color = Color::srgb(0.04, 0.04, 0.04);
const SNAKE_HEAD_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const FOOD_COLOR: Color = Color::srgb(1.0, 0.0, 1.0);

const SNAKE_HEAD_SIZE: f32 = 0.8;
const SNAKE_SEGMENT_SIZE: f32 = 0.64;
const FOOD_SIZE: f32 = 0.8;

const FOOD_SPAWN_PERIOD: Duration = Duration::from_secs(1);
const SNAKE_MOVEMENT_STEP_PERIOD: Duration = Duration::from_millis(150);

const ARENA_SIZE: u32 = 10;

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_systems(Startup, spawn_snake)
        .add_systems(Update, spawn_food.run_if(on_timer(FOOD_SPAWN_PERIOD)))
        .add_systems(Update, snake_movement_input.before(snake_movement))
        .add_systems(
            Update,
            snake_movement.run_if(on_timer(SNAKE_MOVEMENT_STEP_PERIOD)),
        )
        .add_systems(Update, snake_eating.after(snake_movement))
        .add_systems(Update, snake_growth.after(snake_eating))
        .add_systems(Update, game_over.after(snake_movement))
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Snake!".into(),
                resolution: WindowResolution::new(500., 500.),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CameraPlugin)
        .add_plugins(QuitPlugin::default().add_key_binding(KeyCode::KeyQ))
        .run();
}

fn snake_movement_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut head: Single<&mut SnakeHead>,
) {
    let dir: Direction = if keyboard_input.pressed(KeyCode::ArrowLeft) {
        Direction::Left
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        Direction::Down
    } else if keyboard_input.pressed(KeyCode::ArrowUp) {
        Direction::Up
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        Direction::Right
    } else {
        head.direction
    };
    if dir != head.direction.opposite() {
        head.direction = dir;
    }
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    head_query: Single<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    let (head_entity, head) = *head_query;

    let segment_positions = segments
        .0
        .iter()
        .map(|e| *positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();

    *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));

    let mut head_pos = positions.get_mut(head_entity).unwrap();

    *head_pos = match &head.direction {
        Direction::Left => head_pos.left(),
        Direction::Right => head_pos.right(),
        Direction::Up => head_pos.up(),
        Direction::Down => head_pos.down(),
    };

    if segment_positions.contains(&head_pos) {
        game_over_writer.send(GameOverEvent);
    }

    segment_positions
        .iter()
        .zip(segments.0.iter().skip(1))
        .for_each(|(pos, segment)| {
            *positions.get_mut(*segment).unwrap() = *pos;
        });
}

fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    let snake_head_position = Position { x: 3, y: 3 };
    *segments = SnakeSegments(vec![
        commands
            .spawn((
                Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                Transform::from_scale(Vec3::new(10.0, 10.0, 10.0)),
            ))
            .insert(SnakeHead {
                direction: Direction::Right,
            })
            .insert(SnakeSegment)
            .insert(snake_head_position)
            .insert(Size(SNAKE_HEAD_SIZE))
            .id(),
        spawn_segment(commands, snake_head_position.left()),
    ]);
}

fn size_scaling(window: Single<&Window>, mut q: Query<(&Size, &mut Transform)>) {
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.0 / ARENA_SIZE as f32 * window.width(),
            sprite_size.0 / ARENA_SIZE as f32 * window.height(),
            1.0,
        );
    }
}

fn position_translation(window: Single<&Window>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width(), ARENA_SIZE as f32),
            convert(pos.y as f32, window.height(), ARENA_SIZE as f32),
            0.0,
        );
    }
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn left(&self) -> Position {
        Position {
            x: warp_position(self.x - 1),
            y: self.y,
        }
    }
    fn right(&self) -> Position {
        Position {
            x: warp_position(self.x + 1),
            y: self.y,
        }
    }
    fn up(&self) -> Position {
        Position {
            x: self.x,
            y: warp_position(self.y + 1),
        }
    }
    fn down(&self) -> Position {
        Position {
            x: self.x,
            y: warp_position(self.y - 1),
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
struct Size(f32);

fn spawn_food(mut commands: Commands) {
    commands
        .spawn(Sprite {
            color: FOOD_COLOR,
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_SIZE as f32) as i32,
            y: (random::<f32>() * ARENA_SIZE as f32) as i32,
        })
        .insert(Size(FOOD_SIZE));
}

#[derive(Component)]
struct Food;

#[derive(PartialEq, Copy, Clone, Default)]
enum Direction {
    Left,
    Up,
    #[default]
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component)]
struct SnakeSegment;

#[derive(Default, Resource)]
struct SnakeSegments(Vec<Entity>);

fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn(Sprite {
            color: SNAKE_SEGMENT_COLOR,
            ..default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size(SNAKE_SEGMENT_SIZE))
        .id()
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_pos: Single<&Position, With<SnakeHead>>,
) {
    for (ent, food_pos) in food_positions.iter() {
        if food_pos == *head_pos {
            commands.entity(ent).despawn();
            growth_writer.send(GrowthEvent);
        }
    }
}

#[derive(Event)]
struct GrowthEvent;

#[derive(Default, Resource)]
struct LastTailPosition(Option<Position>);

fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.read().next().is_some() {
        if let Some(last_tail_position) = last_tail_position.0 {
            segments.0.push(spawn_segment(commands, last_tail_position));
        }
    }
}

#[derive(Event)]
struct GameOverEvent;

fn game_over(
    mut commands: Commands,
    reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if !reader.is_empty() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
    }
}

fn warp_position(pos: i32) -> i32 {
    if pos < 0 {
        pos + ARENA_SIZE as i32
    } else if pos >= ARENA_SIZE as i32 {
        pos - ARENA_SIZE as i32
    } else {
        pos
    }
}
