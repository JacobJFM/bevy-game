use bevy::prelude::*;

const PLAYER_SPEED: f32 = 200.0;
const JUMP_POWER: f32 = 300.0;
const GRAVITY: f32 = -9.8 * 100.0;

#[derive(Component)]
struct Player {
    speed: f32,
    jump_power: f32,
    velocity: Vec2,
    is_grounded: bool,
}

#[derive(Component)]
struct Platform;

fn main() {
    // Your program's entry point
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (player_movement, player_jump, apply_gravity))
        .run();
}

fn setup(mut commands: Commands) {
    // spawn camera
    commands.spawn(Camera2dBundle::default());

    // spawn player
    commands.spawn((
        Player { speed: PLAYER_SPEED, jump_power: JUMP_POWER, velocity: Vec2::ZERO, is_grounded: false },
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 0.0, 0.0, 1.0), // RED color using RGB values
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    // spawn platforms
    commands.spawn((
        Platform,
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(200.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..default()
        },
    ));
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();
    let mut direction = Vec3::ZERO;

    if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
    }

    transform.translation += direction * player.speed * time.delta_seconds();
}

fn player_jump(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Player>,
) {
    let mut player = query.single_mut();

    if input.just_pressed(KeyCode::Space) && player.is_grounded {
        player.velocity.y = player.jump_power;
        player.is_grounded = false;
    }
}

fn apply_gravity(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    platform_query: Query<&Transform, (With<Platform>, Without<Player>)>,
    time: Res<Time>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();
    let gravity = GRAVITY;

    player.velocity.y += gravity * time.delta_seconds();
    player_transform.translation.y += player.velocity.y * time.delta_seconds();
    
    // simple collision detection with platform
    for platform_transform in platform_query.iter() {
        let platform_top = platform_transform.translation.y + 15.0; // half of hard-coded platform height
        let player_bottom = player_transform.translation.y - 15.0; // half of player height

        if player_bottom <= platform_top && player.velocity.y <= 0.0 {
            player_transform.translation.y = platform_top + 15.0;
            player.velocity.y = 0.0;
            player.is_grounded = true;
            break;
        }
    }
}