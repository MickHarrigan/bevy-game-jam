use bevy::time::common_conditions::on_timer;
use quadtree::prelude::*;
use quadtree::quadtree::tree::QuadTree;
use rand::Rng;
use std::time::Duration;

use crate::boids::{create_boid_group, move_system};
// use crate::world::Queen;
use crate::{
    boids::{build_or_update_quadtree, update_boids, Boid},
    loading::TextureAssets,
    GameState,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_pancam::PanCam;

pub struct BeesPlugin;

impl Plugin for BeesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(
                Update,
                create_boid_group.run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, update_cursor.run_if(in_state(GameState::Playing)))
            .add_systems(Update, place_bee.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_boids.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                show_mouse_location.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                build_or_update_quadtree.run_if(on_timer(Duration::from_secs_f32(1. / 90.))),
            )
            .add_systems(Update, move_system.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub id: Option<quadtree::prelude::slot_map::SlotId>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Collider { radius, id: None }
    }

    pub fn into_region(&self, origin: Vec3) -> quadtree::prelude::region::Region {
        let min = quadtree::prelude::coord::Coord::from_f32(origin.x, origin.y)
            - quadtree::prelude::coord::Coord::from_f32(self.radius, self.radius) / 2;
        let max = quadtree::prelude::coord::Coord::from_f32(origin.x, origin.y)
            + quadtree::prelude::coord::Coord::from_f32(self.radius, self.radius) / 2;

        quadtree::prelude::region::Region::new(min, max)
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn default() -> Self {
        let mut rng = rand::thread_rng();
        Velocity(Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            0.0,
        ))
    }
}

#[derive(Component)]
struct Bee;

// Seperate out these types of data???
#[derive(Component)]
pub struct BoidGroup {
    pub graph: quadtree::prelude::tree::QuadTree<Body>,
    pub id: u32,
    pub count: u32,
    pub separation: f32,
    pub alignment: f32,
    pub cohesion: f32,
    pub speed: f32,
    pub vision: f32,
}

impl BoidGroup {
    pub fn new(min: Vec2, max: Vec2, team: Team) -> Self {
        let min = coord::Coord::from_f32(min.x, min.y);
        let max = coord::Coord::from_f32(max.x, max.y);
        BoidGroup {
            graph: QuadTree::new(region::Region::new(min, max)),
            id: team.0,
            count: 0,
            separation: 0.1,
            alignment: 0.4,
            cohesion: 0.5,
            speed: 20.0,
            vision: 10.0,
        }
    }
}

#[derive(Component)]
pub struct Team(pub u32);

#[derive(Component, Debug)]
pub struct Body {
    pub entity: Entity,
    pub position: Vec3,
    pub velocity: Vec3,
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
) {
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
            Boid,
            Collider::new(5.0),
            Velocity::default(),
        ));
        info!("Bee spawned");
    }
}

// Move bees according to velocity
fn move_bee(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Collider), With<Bee>>,
    time: Res<Time>,
) {
    // let speed = 25.0;
    for (mut transform, velocity, collider) in query.iter_mut() {
        transform.translation.x += velocity.0.x * time.delta_seconds();
        transform.translation.y += velocity.0.y * time.delta_seconds();

        transform.rotation = Quat::from_rotation_arc(Vec3::Y, velocity.0.normalize());

        // Print all bee collisions
    }
}

fn show_mouse_location(mut gizmos: Gizmos, mouse_position: Res<MousePosition>) {
    gizmos.ray_2d(mouse_position.position, Vec2::new(1., 0.), Color::GREEN);
    gizmos.ray_2d(mouse_position.position, Vec2::new(0., 1.), Color::RED);
}
