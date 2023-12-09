use crate::{
    bees::{Body, Team},
    // world::LdtkLevel,
};
use bevy::prelude::*;
// use bevy_ecs_ldtk::prelude::*;

use crate::bees::{BoidGroup, Collider, Velocity};
use crate::tilemap::LevelData;

#[derive(Component)]
pub struct Boid;

pub fn create_boid_group(
    mut comms: Commands,
    // level: Res<Assets<LdtkProject>>,
    // handle: Res<LdtkLevel>,
    level_data: Res<LevelData>,
    mut loaded: Local<bool>,
) {
    if *loaded {
        return;
    }
    let height = level_data.level_height;
    let width = level_data.level_width;
    comms.spawn(BoidGroup::new(
        Vec2::new(0., 0.),
        Vec2::new(width, height),
        Team(0),
    ));
    *loaded = true;
}

pub fn update_boids(
    mut query: Query<(&Transform, &mut Collider, &mut Velocity)>,
    universe: Query<&BoidGroup>,
) {
    // TODO: dont let this crash, add this only once
    let universe = match universe.get_single() {
        Ok(a) => a,
        Err(_) => return,
    };
    query
        .iter_mut()
        .for_each(|(transform, collider, mut velocity)| {
            let x = transform.translation.x as i32;
            let y = transform.translation.y as i32;
            // let win = universe.graph.size();

            // -------------------- collision query --------------------
            let query_region = collider
                .into_region(transform.translation)
                .with_margin((universe.vision) as i32);
            let exclude = match &collider.id {
                Some(id) => vec![id.clone()],
                None => vec![],
            };

            let collisions = universe.graph.query(&query_region, &exclude);

            let (mass_center, aligment, separation) = collisions.iter().fold(
                (Vec3::ZERO, Vec3::ZERO, Vec3::ZERO),
                |(mcen, alg, sep), body| {
                    (
                        mcen + body.position.normalize(),
                        alg + body.velocity.normalize(),
                        sep + (transform.translation - body.position).normalize(),
                    )
                },
            );

            let mut direction = velocity.0.normalize();

            // -------------------- Cohesion --------------------
            if mass_center.length() > 0.0 {
                direction += (mass_center.normalize() - transform.translation.normalize())
                    .normalize()
                    * universe.cohesion;
            }

            // -------------------- Alignment --------------------
            if aligment.length() > 0.0 {
                direction += aligment.normalize() * universe.alignment;
            }

            // -------------------- Separation --------------------
            if separation.length() > 0.0 {
                direction += separation.normalize() * universe.separation;
            }

            let mut new_velocity = direction.normalize() * velocity.0.length();

            // -------------------- World Border --------------------
            // this barely works, but it does work
            let margin: i32 = 20;
            if (x < 0 + margin && velocity.0.x < 0.0) || (x > 3840 - margin && velocity.0.x > 0.0) {
                new_velocity.x *= -1.0;
            }
            if (y < 0 + margin && velocity.0.y < 0.0) || (y > 2160 - margin && velocity.0.y > 0.0) {
                new_velocity.y *= -1.0;
            }

            // finally set the new velocity
            velocity.0 = new_velocity;
        });
}

pub fn move_system(
    mut query: Query<(&mut Transform, &Velocity)>,
    universe: Query<&BoidGroup>,
    time: Res<Time>,
) {
    let universe = match universe.get_single() {
        Ok(a) => a,
        Err(_) => return,
    };
    query.par_iter_mut().for_each(|(mut transform, velocity)| {
        let direction = velocity.0.normalize();
        let rotation = Quat::from_rotation_z(-direction.x.atan2(direction.y));
        transform.rotation = rotation;

        transform.translation += velocity.0 * time.delta_seconds() * universe.speed;
        transform.translation.z = 5.;
    });
}

pub fn build_or_update_quadtree(
    mut query: Query<(Entity, &Transform, &mut Collider, &Velocity), With<Boid>>,
    mut universe: Query<&mut BoidGroup>,
) {
    let mut universe = match universe.get_single_mut() {
        Ok(a) => a,
        Err(_) => return,
    };
    universe.graph.clear();
    query
        .iter_mut()
        .for_each(|(entity, transform, mut collider, velocity)| {
            collider.id = Some(universe.graph.insert(
                collider.into_region(transform.translation),
                Body {
                    entity,
                    position: transform.translation,
                    velocity: velocity.0,
                },
            ));
        });
}
