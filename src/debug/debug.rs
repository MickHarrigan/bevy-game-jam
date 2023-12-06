use bevy::{math, prelude::*};
use rand::Rng;

use crate::{
    bees::{Bee, BoidGroup, Collider, Velocity},
    boids::Boid,
    loading::TextureAssets,
    GameState,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Visualizer::default())
            .register_type::<Visualizer>()
            .register_type::<BoidGroup>()
            .add_systems(Update, bevy::window::close_on_esc)
            .add_systems(
                Update,
                (
                    visualize_quadtree,
                    visualize_boid_radius,
                    spawn_random_boids,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Visualizer {
    pub quadtree: bool,
    pub boid_vision: bool,
    pub boid_cone: bool,
}

pub fn visualize_quadtree(mut gizmos: Gizmos, vis: Res<Visualizer>, groups: Query<&BoidGroup>) {
    if !vis.quadtree {
        return;
    }
    for group in groups.iter() {
        let regions = group.graph.get_regions();
        regions.iter().for_each(|reg| {
            let (min_x, min_y, max_x, max_y) = reg.into_f32();

            let bottom_left = Vec3::new(min_x, min_y, 0.0);
            let bottom_right = Vec3::new(max_x, min_y, 0.0);
            let top_right = Vec3::new(max_x, max_y, 0.0);
            let top_left = Vec3::new(min_x, max_y, 0.0);

            gizmos.line(bottom_left, bottom_right, Color::WHITE);
            gizmos.line(bottom_right, top_right, Color::WHITE);
            gizmos.line(top_right, top_left, Color::WHITE);
            gizmos.line(top_left, bottom_left, Color::WHITE);
        });
    }
}

pub fn visualize_boid_radius(
    boids: Query<&Transform, With<Boid>>,
    groups: Query<&BoidGroup>,
    // mut comms: Commands,
    // mut mats: ResMut<Assets<ColorMaterial>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    mut gizmos: Gizmos,
    vis: Res<Visualizer>,
) {
    // make a circle on each boid that shows how far it can see
    if !vis.boid_vision {
        return;
    }
    let Ok(group) = groups.get_single() else {
        return;
    };
    for transform in &boids {
        gizmos.circle_2d(transform.translation.xy(), group.vision, Color::PURPLE);
    }
}

fn spawn_random_boids(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    textures: Res<TextureAssets>,
) {
    // spawn in 1000 boids randomly in the field on press of some key
    if input.just_pressed(KeyCode::Space) {
        (0..1000).for_each(|_| {
            let mut rng = rand::thread_rng();
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: textures.planes.clone(),
                    sprite: TextureAtlasSprite::new(11),
                    transform: Transform::from_xyz(
                        rng.gen_range(20.0..3820.0),
                        rng.gen_range(20.0..2140.0),
                        5.0,
                    ),
                    ..default()
                },
                Bee,
                Boid,
                Collider::new(5.0),
                Velocity::default(),
            ));
        });
    }
}

fn edit_boid_groups(groups: Query<&BoidGroup>) {
    // this should creat a window with a dropdown of all boidgroups and allow editing of all the values within
}
