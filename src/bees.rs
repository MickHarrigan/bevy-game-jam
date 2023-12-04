use std::time::Duration;

use crate::{loading::TextureAssets, GameState};
use bevy::prelude::*;
use crate::world::Queen;

pub struct BeesPlugin;

impl Plugin for BeesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, spawn_bee.run_if(in_state(GameState::Playing)))
        .add_systems(Update, update_cursor.run_if(in_state(GameState::Playing)))
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
    query: Query<&Transform, With<Queen>>
){
    bee_spawner.timer.tick(time.delta());

    if bee_spawner.timer.just_finished() {
        let queen_transform = query.single();
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: textures.shmup.clone(),
                sprite: TextureAtlasSprite::new(12),
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
    mut cursor_evr: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    for ev in cursor_evr.read() {
        // info!("New cursor position: X: {}, Y: {}, in Window ID: {:?}", ev.position.x, ev.position.y, ev.window);
        mouse_position.position = ev.position;
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
        transform.translation += Vec3::new(direction.x, direction.y, 0.0) * speed * time.delta_seconds();
        if direction.x > 0.0 {
            sprite.flip_x = false;
        } else {
            sprite.flip_x = true;
        }
    }
}