use bevy::window::PrimaryWindow;
use bevy::prelude::*;
use crate::GameState;

use std::collections::HashSet;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MousePosition(Vec2::new(0.0, 0.0)))
            .add_systems(Update, update_mouse_position.run_if(in_state(GameState::Playing)))
            .add_systems(Update, show_mouse_location.run_if(in_state(GameState::Playing)))
            .insert_resource(MouseState(MouseStates::Default))
            .insert_resource(HighlightedEntities(HashSet::new()))
            .add_systems(Update, mouse_state_manager.run_if(in_state(GameState::Playing)))
            .add_systems(Update, draw_mouse_region.run_if(in_state(GameState::Playing)))
            .add_systems(Update, bloom_highlighted_entities.run_if(in_state(GameState::Playing)))
            .add_systems(Update, remove_bloom.run_if(in_state(GameState::Playing)))
        ;
    }
}

// Resources
#[derive(Resource, Debug)]
pub struct MousePosition(pub Vec2);

#[derive(Resource)]
struct LastClickedPosition(Vec2);

// Components
#[derive(Debug)]
enum MouseStates {
    LeftDragging(Vec2), // LeftDragging holds the state of last left mouse click
    RightDragging(Vec2),
    MiddleDragging(Vec2),
    Default
}
#[derive(Resource, Debug)]
struct MouseState(MouseStates);

#[derive(Component)]
pub struct Highlightable;

#[derive(Component)]
pub struct Highlighted;
#[derive(Resource, Debug)]
struct HighlightedEntities(HashSet<Entity>);

#[derive(Component)]
pub struct Clickable;

// Systems
fn update_mouse_position(
    q_cam: Query<(&Camera, &GlobalTransform)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut mouse_position: ResMut<MousePosition>,
) {
    let (camera, camera_transform) = q_cam.single();
    let window = q_window.single();

    if let Some(world_pos) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    // .map(|ray| ray.origin.truncate())
    {
        mouse_position.0 = world_pos;
    }
}
fn show_mouse_location(mut gizmos: Gizmos, mouse_position: Res<MousePosition>) {
    gizmos.ray_2d(mouse_position.0, Vec2::new(1., 0.), Color::GREEN);
    gizmos.ray_2d(mouse_position.0, Vec2::new(0., 1.), Color::RED);
}

fn mouse_state_manager(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    mut mouse_state: ResMut<MouseState>,
    mouse_position: Res<MousePosition>,
    mut highlighted_entities: ResMut<HighlightedEntities>,
    mut q_entities: Query<(Entity, &Transform, Option<&Highlighted>), With<Highlightable>>,
) {
    for button in buttons.get_just_pressed() {
        info!("{:?} is currently held down", button);
        // mouse_state.0 = MouseStates::MouseDown;
        match button {
            MouseButton::Left => {
                mouse_state.0 = { MouseStates::LeftDragging(mouse_position.0) };
                // TODO: Do something with the highlighted entities with an event at this point
                highlighted_entities.0.clear();
            },
            MouseButton::Right => mouse_state.0 = MouseStates::RightDragging(mouse_position.0),
            MouseButton::Middle => mouse_state.0 = MouseStates::MiddleDragging(mouse_position.0),
            _ => {}
        }
    }
    for button in buttons.get_just_released() {
        info!("{:?} has been released", button);
        match button {
            // On mouse left just released
            MouseButton::Left => {
                // If it was previously in the dragging state
                if let MouseStates::LeftDragging(start_pos) = mouse_state.0 {
                    // Grab the mouse dragged square region
                    let min_x = start_pos.x.min(mouse_position.0.x);
                    let max_x = start_pos.x.max(mouse_position.0.x);
                    let min_y = start_pos.y.min(mouse_position.0.y);
                    let max_y = start_pos.y.max(mouse_position.0.y);
                    // For all entities with
                    for (entity, transform, highlighted) in q_entities.iter_mut() {
                        let entity_pos = transform.translation;
                        // Check if their transform is in the square region
                        if entity_pos.x >= min_x && entity_pos.x <= max_x &&
                            entity_pos.y >= min_y && entity_pos.y <= max_y {
                            info!("Entity {:?} is inside the mouse square region.", entity);
                            // Add them to the highlighted entities hashmap
                            highlighted_entities.0.insert(entity);

                            if let None = highlighted {
                                commands.entity(entity).insert(Highlighted);
                            }
                        } else {
                            if let Some(_) = highlighted {
                                commands.entity(entity).remove::<Highlighted>();
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        mouse_state.0 = MouseStates::Default;
    }
}

fn draw_mouse_region
(
    mouse_state: Res<MouseState>,
    mouse_position: Res<MousePosition>,
    mut gizmos: Gizmos
) {
    match mouse_state.0 {
        MouseStates::LeftDragging(pos) => {
            // Diagonal debug line
            // gizmos.line_2d(pos, mouse_position.0, Color::RED);
            // Draw horizontal line
            gizmos.line_2d(pos, Vec2::new(mouse_position.0.x, pos.y), Color::BLUE);
            // Draw vertical line
            gizmos.line_2d(pos, Vec2::new(pos.x, mouse_position.0.y), Color::BLUE);
            // Draw connecting horizontal line
            gizmos.line_2d(mouse_position.0, Vec2::new(pos.x, mouse_position.0.y), Color::BLUE);
            // Draw connecting vertical line
            gizmos.line_2d(mouse_position.0, Vec2::new(mouse_position.0.x, pos.y), Color::BLUE);
        }
        _ => {}
    }
}

fn bloom_highlighted_entities
(
    mut q_highlighted: Query<&mut TextureAtlasSprite, Added<Highlighted>>,
) {
    for mut texture in q_highlighted.iter_mut() {
        let col = texture.color.as_hsla();
        texture.color = Color::hsla(col.h(), col.s(), col.l() * 1.5, col.a());
    }
}

fn remove_bloom
(
    mut removals: RemovedComponents<Highlighted>,
    mut q_highlighted: Query<(Entity, &mut TextureAtlasSprite)>
) {
    for removed_entity in removals.read() {
        if let Ok((_, mut texture)) = q_highlighted.get_mut(removed_entity) {
            let col = texture.color.as_hsla();
            texture.color = Color::hsla(col.h(), col.s(), col.l() / 1.5, col.a());
        }
    }
}