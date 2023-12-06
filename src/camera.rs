use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use crate::GameState;

pub struct CameraPlugin;

// Responsible for paining the camera in bounds with WASD and arrow keys. Plus zoom in with mouse wheel
// Only active during GameState::Playing
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), setup_camera_controls)
            .add_systems(Update, panning_controls.run_if(in_state(GameState::Playing)))
            .add_systems(Update, zooming_controls.run_if(in_state(GameState::Playing)));
    }
}

fn setup_camera_controls(
    mut q_camera: Query<&mut OrthographicProjection, With<Camera2d>>,
) {
    info!("Setting up camera plugin");
    let mut projection = q_camera.single_mut();
    projection.scale = 0.6;
}

fn panning_controls
(
    keys: Res<Input<KeyCode>>,
    mut q_cam: Query<&mut Transform, With<Camera2d>>
)
{
    let pan_speed = 2.0;
    let (mut cam_transform) = q_cam.single_mut();
    if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
        cam_transform.translation.y += pan_speed;
    }
    if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
        cam_transform.translation.x -= pan_speed;
    }
    if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
        cam_transform.translation.y -= pan_speed;
    }
    if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
        cam_transform.translation.x += pan_speed;
    }
}

// TODO: Zoom into camera position and not center
fn zooming_controls
(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q_camera: Query<&mut OrthographicProjection, With<Camera2d>>,

) {
    let zoom_speed = 0.2;
    let max_scale = 5.0;
    let min_scale = 0.2;
    for ev in scroll_evr.iter() {
        use bevy::input::mouse::MouseScrollUnit;
        let mut projection = q_camera.single_mut();
        // info!("Current projection value: {:?}", projection.scale);
        // match ev.unit {
        //     MouseScrollUnit::Line => {
        //         println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
        //     }
        //     MouseScrollUnit::Pixel => {
        //         println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
        //     }
        // }

        let zoom_amount = match ev.unit {
            MouseScrollUnit::Line => ev.y * zoom_speed,
            MouseScrollUnit::Pixel => ev.y * zoom_speed,
        };

        projection.scale -= zoom_amount;
        if projection.scale < min_scale {projection.scale = min_scale}
        if projection.scale > max_scale {projection.scale = max_scale}

    }
}