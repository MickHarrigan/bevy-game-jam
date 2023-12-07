use bevy::window::PrimaryWindow;
use bevy::prelude::*;
use crate::bees::Bee;
use crate::GameState;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MousePosition(Vec2::new(0.0, 0.0)))
            .add_systems(Update, update_mouse_position.run_if(in_state(GameState::Playing)))
            .add_systems(Update, show_mouse_location.run_if(in_state(GameState::Playing)))
            .insert_resource(MouseState(MouseStates::Default))
            .add_systems(Update, mouse_state_manager.run_if(in_state(GameState::Playing)))
            .add_systems(Update, draw_mouse_region.run_if(in_state(GameState::Playing)))
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
struct Highlighted {
    list: Vec<Entity>
}

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
    buttons: Res<Input<MouseButton>>,
    mut mouse_state: ResMut<MouseState>,
    mouse_position: Res<MousePosition>,
    mut q_bees: Query<(Entity, &Transform), With<Bee>>,
) {
    for button in buttons.get_just_pressed() {
        info!("{:?} is currently held down", button);
        // mouse_state.0 = MouseStates::MouseDown;
        match button {
            MouseButton::Left => mouse_state.0 = MouseStates::LeftDragging(mouse_position.0),
            MouseButton::Right => mouse_state.0 = MouseStates::RightDragging(mouse_position.0),
            MouseButton::Middle => mouse_state.0 = MouseStates::MiddleDragging(mouse_position.0),
            _ => {}
        }
    }
    for button in buttons.get_just_released() {
        info!("{:?} has been released", button);
        match button {
            // Checking for all bee entities inside square mouse region on mouse up event
            MouseButton::Left => {
                if let MouseStates::LeftDragging(start_pos) = mouse_state.0 {
                    let min_x = start_pos.x.min(mouse_position.0.x);
                    let max_x = start_pos.x.max(mouse_position.0.x);
                    let min_y = start_pos.y.min(mouse_position.0.y);
                    let max_y = start_pos.y.max(mouse_position.0.y);

                    for (entity, transform) in q_bees.iter_mut() {
                        let bee_pos = transform.translation;
                        if bee_pos.x >= min_x && bee_pos.x <= max_x &&
                            bee_pos.y >= min_y && bee_pos.y <= max_y {
                            info!("Bee entity {:?} is inside the mouse square region.", entity);
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
            gizmos.line_2d(pos, mouse_position.0, Color::RED);
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