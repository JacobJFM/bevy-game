use bevy::{color::palettes::css::*, math::bounding::*, prelude::*};

const PLAYER_SPEED: f32 = 250.0;
const JUMP_POWER: f32 = 400.0;
const GRAVITY: f32 = -9.8 * 100.0;
const PLAYER_SIZE: (f32, f32) = (50.0, 50.0);

#[derive(Component)]
struct Player {
    speed: f32,
    jump_power: f32,
    velocity: Vec2,
    is_grounded: bool,
}

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Collider {
    size: Vec2,
}

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
        Player {
            speed: PLAYER_SPEED,
            jump_power: JUMP_POWER,
            velocity: Vec2::ZERO,
            is_grounded: false,
        },
        Collider {
            size: Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1),
        },
        SpriteBundle {
            sprite: Sprite {
                color: bevy::prelude::Color::Srgba(ALICE_BLUE),
                custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    // define platform size
    let platform_size = Vec2::new(200.0, 30.0);

    // spawn platforms
    commands.spawn((
        Platform,
        Collider {
            size: platform_size,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(platform_size),
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

fn player_jump(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Player>) {
    let mut player = query.single_mut();

    if input.just_pressed(KeyCode::Space) && player.is_grounded {
        player.velocity.y = player.jump_power;
        player.is_grounded = false;
    }
}

fn apply_gravity(
    mut player_query: Query<(&mut Player, &mut Transform, &Collider)>,
    platform_query: Query<(&Transform, &Collider), (With<Platform>, Without<Player>)>,
    time: Res<Time>,
) {
    let (mut player, mut player_transform, player_collider) = player_query.single_mut();
    let gravity = GRAVITY;

    player.velocity.y += gravity * time.delta_seconds();
    player_transform.translation.y += player.velocity.y * time.delta_seconds();

    let player_aabb = Aabb2d::new(
        player_transform.translation.truncate(),
        player_collider.size / 2.0
    );

    for (platform_transform, platform_collider) in platform_query.iter() {
        let platform_aabb = Aabb2d::new(
            platform_transform.translation.truncate(),
            platform_collider.size / 2.0
        );

        if player_aabb.intersects(&platform_aabb) && player.velocity.y <= 0.0 {
            // push player above platform, calculating new y position using centerpoints and size of both the player and platform
            player_transform.translation.y = platform_transform.translation.y + platform_collider.size.y / 2.0 + player_collider.size.y / 2.0;
            player.velocity.y = 0.0;
            player.is_grounded = true;
            break;
        }
    }
}