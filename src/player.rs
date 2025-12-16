use bevy::prelude::*;

// Atlas constants
const TILE_SIZE: u32 = 64; // 64x64 tiles
const WALK_FRAMES: usize = 9; // 9 columns per walking row
const MOVE_SPEED: f32 = 140.0; // pixels per second
const ANIM_DT: f32 = 0.1; // seconds per frame (~10 fps)

#[derive(Component)]
struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationState {
    facing: Facing,
    moving: bool,
    was_moving: bool,
}

fn spawm_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the spritesheet and build a grid layout: 64x64 tiles, 9 columns, 12 rows
    let texture = asset_server.load("hero_spritesheet.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32, // columns used for walking frames
        12,                 // at least 12 rows available
        None,
        None,
    ));

    let facing = Facing::Down;
    let start_index = atlas_index_for(facing, 0);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        Transform::from_translation(Vec3::ZERO),
        Player,
        AnimationState {
            facing,
            moving: false,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut AnimationState), With<Player>>,
) {
    let Ok((mut transform, mut anim)) = player.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }

    if direction != Vec2::ZERO {
        let speed = 300.0;
        let delta = direction.normalize() * speed * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;

        // Update facing based on dominant direction
        if direction.x.abs() > direction.y.abs() {
            anim.facing = if direction.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            };
        } else {
            anim.facing = if direction.y > 0.0 {
                Facing::Up
            } else {
                Facing::Down
            };
        }
    } else {
        anim.moving = false;
    }
}
