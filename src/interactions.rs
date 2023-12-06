use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::GameState;

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MousePosition { position: Vec2::new(0.0, 0.0)})
            .add_systems(Update, update_mouse_position.run_if(in_state(GameState::Playing)))
            .add_systems(Update, show_mouse_location.run_if(in_state(GameState::Playing)));
    }
}

// Resources
#[derive(Resource)]
pub struct MousePosition {
    pub position: Vec2,
}

// Components
#[derive(Component)]
pub struct Highlightable;

#[derive(Component)]
pub struct Selectable;

// Systems


// Update mouse position
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
        mouse_position.position = world_pos;
    }
}
fn show_mouse_location(mut gizmos: Gizmos, mouse_position: Res<MousePosition>) {
    gizmos.ray_2d(mouse_position.position, Vec2::new(1., 0.), Color::GREEN);
    gizmos.ray_2d(mouse_position.position, Vec2::new(0., 1.), Color::RED);
}

fn draw_mouse_region() {
    todo!()
}