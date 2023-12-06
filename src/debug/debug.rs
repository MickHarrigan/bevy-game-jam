use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{bees::BoidGroup, boids::Boid, GameState};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Visualizer::default())
            .register_type::<Visualizer>()
            .add_systems(
                Update,
                (visualize_quadtree, visualize_boid_sight).run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct Visualizer {
    pub quadtree: bool,
    pub boid_vision: bool,
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

pub fn visualize_boid_sight(
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
        // comms.spawn(MaterialMesh2dBundle {
        //     mesh: meshes.add(shape::Circle::new(group.vision).into()).into(),
        //     material: mats.add(ColorMaterial::from(Color::PURPLE).into()).into(),
        //     transform: *transform,
        //     ..default()
        // });
    }
}
