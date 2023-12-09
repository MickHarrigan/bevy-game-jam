use bevy::time::common_conditions::on_timer;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;
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

use crate::interactions::{MousePosition, Highlightable};
use bevy::prelude::*;
// use bevy::window::PrimaryWindow;

pub struct BeesPlugin;

impl Plugin for BeesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), setup)
            .add_systems(Update, create_boid_group.run_if(in_state(GameState::Playing)))
            .add_systems(Update, place_bee.run_if(in_state(GameState::Playing)))
            .add_systems(Update, animate_wings.run_if(in_state(GameState::Playing)))
            .add_systems(
                Update,
                (build_or_update_quadtree, update_boids, move_system).run_if(
                    in_state(GameState::Playing)
                        .and_then(on_timer(Duration::from_secs_f32(1. / 90.))),
                ),
            )
        ;
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
pub struct Bee;

#[derive(Component)]
pub enum BeeBehavior {
    Traveling(Vec2), // Destination coordinates
    Wondering(Vec2), // Point of origin coordinates
    Exploring(Vec2), // Point of origin coordinates
    Interacting(Vec2), // Coordinates of interactable object
}

// Separate out these types of data???
#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)]
#[reflect(from_reflect = false)]
pub struct BoidGroup {
    #[reflect(ignore)]
    pub graph: quadtree::prelude::tree::QuadTree<Body>,
    pub id: u32,
    pub count: u32,
    #[inspector(min = 0.0, max = 1.0)]
    pub separation: f32,
    #[inspector(min = 0.0, max = 1.0)]
    pub alignment: f32,
    #[inspector(min = 0.0, max = 1.0)]
    pub cohesion: f32,
    #[inspector(min = 0.0, max = 200.0)]
    pub speed: f32,
    #[inspector(min = 0.0, max = 1000.0)]
    pub vision: f32,
}

impl Default for BoidGroup {
    fn default() -> Self {
        Self {
            graph: QuadTree::new(region::Region::new(
                coord::Coord { x: 0, y: 0 },
                coord::Coord { x: 0, y: 0 },
            )),
            ..default()
        }
    }
}

impl BoidGroup {
    pub fn new(min: Vec2, max: Vec2, team: Team) -> Self {
        let min = coord::Coord::from_f32(min.x, min.y);
        let max = coord::Coord::from_f32(max.x, max.y);
        BoidGroup {
            graph: QuadTree::new(region::Region::new(min, max)),
            id: team.0,
            count: 0,
            separation: 0.3,
            alignment: 0.4,
            cohesion: 0.8,
            speed: 40.0,
            vision: 50.0,
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



fn setup(mut _commands: Commands) {
    // commands.insert_resource(BeeSpawner {
    //     timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    // });
    // commands.insert_resource(MousePosition {
    //     position: Vec2::new(0.0, 0.0),
    // });
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_wings(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn place_bee(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    textures: Res<TextureAssets>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        let bee_entity = commands.spawn((
            SpriteBundle {
                texture: textures.beebody1.clone(),
                transform: Transform::from_xyz(
                    mouse_position.0.x,
                    mouse_position.0.y,
                    5.0,
                ),
                ..default()
            },
            Bee,
            BeeBehavior::Wondering(Vec2::new(mouse_position.0.x, mouse_position.0.y)),
            Boid,
            Highlightable,
            Collider::new(5.0),
            Velocity::default(),
        )).id();

        // Create the wings
        let texture_atlas_handle = textures.bee1wingmap.clone();

        // Spawn the wings as a child of the bee body
        commands.entity(bee_entity).with_children(|parent| {
            parent.spawn(SpriteSheetBundle {
                texture_atlas: textures.bee1wingmap.clone(),
                sprite: TextureAtlasSprite::new(0), // Set the initial sprite index
                transform: Transform::from_xyz(0.0, 0.0, 1.0), // Adjust the position of the wings relative to the bee body
                ..Default::default()
            })
                .insert(AnimationIndices { first: 0, last: 3 - 1 })
                .insert(AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)));
        });

        info!("Bee spawned");
    }
}
