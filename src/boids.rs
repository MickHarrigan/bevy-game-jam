use crate::{
    bees::{Body, Team},
    world::LdtkLevel,
};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::bees::{BoidGroup, Collider, Velocity};

#[derive(Component)]
pub struct Boid;

pub fn create_boid_group(
    mut comms: Commands,
    level: Res<Assets<LdtkProject>>,
    handle: Res<LdtkLevel>,
    mut loaded: Local<bool>,
) {
    if *loaded {
        return;
    }
    if let Some(data) = level.get(&handle.0) {
        let height = data.iter_root_levels().next().unwrap().px_hei;
        let width = data.iter_root_levels().next().unwrap().px_wid;
        comms.spawn(BoidGroup::new(
            Vec2::new(0., 0.),
            Vec2::new(width as f32, height as f32),
            Team(0),
        ));
        *loaded = true;
    }
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
        .for_each(|(transform, mut collider, mut velocity)| {
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
            let margin: i32 = 20;
            // if (x < win.min.x + margin && velocity.value.x < 0.0)
            //     || (x > win.max.x - margin && velocity.value.x > 0.0)
            // {
            //     new_velocity.x *= -1.0;
            // }
            // if (y < win.min.y + margin && velocity.value.y < 0.0)
            //     || (y > win.max.y - margin && velocity.value.y > 0.0)
            // {
            //     new_velocity.y *= -1.0;
            // }

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
        let rotation =
            Quat::from_rotation_z(-direction.x.atan2(direction.y) + std::f32::consts::PI / 2.0);
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
