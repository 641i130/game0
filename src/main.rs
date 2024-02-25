use bevy::prelude::*;
use rand::prelude::*;

// World
const BLOCK_SIZE: f32 = 32.0;
const ARENA: f32 = 30.0;
const OFFSET: f32 = BLOCK_SIZE + 0.0;
const RESOLUTION: f32 = BLOCK_SIZE * ARENA + (BLOCK_SIZE * 2.0);
const ARENA_LEFT: isize = -14;
const ARENA_RIGHT: isize = 15;
// Player
const PADDLE_SPEED: f32 = 500.0;

#[derive(Component)]
struct Block;

#[derive(Resource, Default)]
struct Game {
    board: Vec<Vec<Block>>,
    player: Player,
    score: i32,
}

#[derive(Resource, Default, Component)]
struct Player {
    entity: Option<Entity>,
    x: isize,
    y: isize,
    move_cooldown: Timer,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Resource)]
struct BonusSpawnTimer(Timer);

fn main() {
    App::new()
        .add_systems(Startup, setup) // runs once
        .init_resource::<Game>()
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(
            Update,
            (
                move_player,
                //focus_camera,
                //rotate_bonus,
                //scoreboard_system,
                //spawn_bonus,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), teardown)
        //.add_systems(OnEnter(GameState::GameOver), display_score)
        .add_systems(
            Update,
            (
                gameover_keyboard.run_if(in_state(GameState::GameOver)),
                bevy::window::close_on_esc,
            ),
        )
        .add_systems(OnExit(GameState::GameOver), teardown)
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (RESOLUTION, RESOLUTION).into(),
                title: "game0".to_string(),
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .run();
}

// restart the game when pressing spacebar
fn gameover_keyboard(
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

// remove all entities that are not a camera or window
fn teardown(mut commands: Commands, entities: Query<Entity, (Without<Camera>, Without<Window>)>) {
    for entity in &entities {
        commands.entity(entity).despawn();
    }
}

fn setup(mut commands: Commands, mut game: ResMut<Game>) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    // Player
    // spawn the game character
    game.player.x = 0;
    game.player.y = 0;
    game.player.move_cooldown = Timer::from_seconds(0.3, TimerMode::Once);

    game.player.entity = Some(
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    scale: Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 1.0),
                    ..default()
                },
                ..default()
            })
            .id(),
    );

    // Spawn walls
    let mut rng = rand::thread_rng();
    // Should make a random map later (probalby a funny little maze)
    for row in ARENA_LEFT..ARENA_RIGHT {
        for col in ARENA_LEFT..ARENA_RIGHT {
            let w_pos = Vec2::new(col as f32 * OFFSET, row as f32 * OFFSET);
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, rng.gen(), rng.gen()),
                        ..default()
                    },
                    transform: Transform {
                        translation: w_pos.extend(0.0),
                        scale: Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Block,
            ));
        }
    }
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut transforms: Query<&mut Transform>,
) {
    if game.player.move_cooldown.tick(time.delta()).finished() {
        let mut moved = false;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            game.player.y += 32;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            game.player.y -= 32;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            game.player.x += 32;
            moved = true;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            game.player.x -= 32;
            moved = true;
        }

        if moved {
            game.player.move_cooldown.reset();
            *transforms.get_mut(game.player.entity.unwrap()).unwrap() = Transform {
                translation: Vec3::new(game.player.x as f32, game.player.y as f32, 1.0),
                scale: Vec3::new(BLOCK_SIZE, BLOCK_SIZE, 1.0),
                ..default()
            };
        }
    }
}
