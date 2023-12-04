use std::time::Duration;

use crate::world::Queen;
use crate::{loading::TextureAssets, GameState};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_pancam::PanCam;

pub struct BeesPlugin;

impl Plugin for BeesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, spawn_bee.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_cursor.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                show_mouse_location.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, move_bee.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Bee;

#[derive(Resource)]
struct BeeSpawner {
    // How often to spawn a bee (repeating timer)
    timer: Timer,
}

#[derive(Resource)]
struct MousePosition {
    position: Vec2,
}

fn setup(mut commands: Commands) {
    commands.insert_resource(BeeSpawner {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    });
    commands.insert_resource(MousePosition {
        position: Vec2::new(0.0, 0.0),
    });
}

fn spawn_bee(
    mut commands: Commands,
    time: Res<Time>,
    mut bee_spawner: ResMut<BeeSpawner>,
    textures: Res<TextureAssets>,
    query: Query<&Transform, With<Queen>>,
) {
    bee_spawner.timer.tick(time.delta());

    if bee_spawner.timer.just_finished() {
        let queen_transform = query.single();
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: textures.planes.clone(),
                sprite: TextureAtlasSprite::new(11),
                transform: Transform::from_xyz(
                    queen_transform.translation.x,
                    queen_transform.translation.y,
                    5.0,
                ),
                ..default()
            },
            Bee,
        ));
        info!("Bee spawned");
    }
}

// Update mouse position
fn update_cursor(
    q_cam: Query<(&Camera, &GlobalTransform), With<PanCam>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let (cam, cam_trans) = q_cam.single();
    let window = q_window.single();
    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| cam.viewport_to_world(cam_trans, cursor))
        .map(|ray| ray.origin.truncate())
    {
        mouse_position.position = world_pos;
    }
}

// Move bees toards mouse cursor
fn move_bee(
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite), With<Bee>>,
    mouse_position: Res<MousePosition>,
    time: Res<Time>,
) {
    let speed = 25.0;
    for (mut transform, mut sprite) in query.iter_mut() {
        let mut direction = mouse_position.position - transform.translation.truncate();
        direction = direction.normalize();
        // info!("Direction: {:?}", direction);
        transform.translation +=
            Vec3::new(direction.x, direction.y, 0.0) * speed * time.delta_seconds();
        if direction.x > 0.0 {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}

fn show_mouse_location(mut gizmos: Gizmos, mouse_position: Res<MousePosition>) {
    gizmos.ray_2d(mouse_position.position, Vec2::new(1., 0.), Color::GREEN);
    gizmos.ray_2d(mouse_position.position, Vec2::new(0., 1.), Color::RED);
}
