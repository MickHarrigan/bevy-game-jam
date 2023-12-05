use std::time::Duration;
use rand::Rng;

// use crate::world::Queen;
use crate::{loading::TextureAssets, GameState};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_pancam::PanCam;

pub struct BeesPlugin;

impl Plugin for BeesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, update_cursor.run_if(in_state(GameState::Playing)))
            .add_systems(Update, place_bee.run_if(in_state(GameState::Playing)))
            .add_systems(Update,show_mouse_location.run_if(in_state(GameState::Playing)))
            .add_systems(Update, move_bee.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct Collider {
    radius: f32,
}
impl Collider {
    pub fn new(radius: f32) -> Self {
        Collider { radius }
    }
}

#[derive(Component)]
struct Velocity(Vec3);
impl Velocity {
    pub fn default() -> Self {
        let mut rng = rand::thread_rng();
        Velocity(Vec3::new(rng.gen_range(-20.0..20.0), rng.gen_range(-20.0..20.0), 0.0))
    }
}

#[derive(Component)]
struct Bee;

#[derive(Component)]
struct Boid {
    seperation: f32,
    alignment: f32,
    cohesion: f32,
}

#[derive(Resource)]
struct MousePosition {
    position: Vec2,
}

fn setup(mut commands: Commands) {
    // commands.insert_resource(BeeSpawner {
    //     timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    // });
    commands.insert_resource(MousePosition {
        position: Vec2::new(0.0, 0.0),
    });
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

fn place_bee(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    textures: Res<TextureAssets>,
    mouse_input: Res<Input<MouseButton>>,
    ) 
    {
        if mouse_input.just_pressed(MouseButton::Left) {
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: textures.planes.clone(),
                    sprite: TextureAtlasSprite::new(11),
                    transform: Transform::from_xyz(
                        mouse_position.position.x,
                        mouse_position.position.y,
                        5.0,
                    ),
                    ..default()
                },
                Bee,
                Collider::new(5.0),
                Velocity::default(),

            ));
            info!("Bee spawned");
    }
}

// Move bees according to velocity
fn move_bee(
    mut query: Query<(&mut Transform, &mut Velocity), With<Bee>>,
    time: Res<Time>,
) {
    // let speed = 25.0;
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();

        transform.rotation = Quat::from_rotation_arc(Vec3::Y, velocity.0.normalize());
    }
}

fn show_mouse_location(mut gizmos: Gizmos, mouse_position: Res<MousePosition>) {
    gizmos.ray_2d(mouse_position.position, Vec2::new(1., 0.), Color::GREEN);
    gizmos.ray_2d(mouse_position.position, Vec2::new(0., 1.), Color::RED);
}
