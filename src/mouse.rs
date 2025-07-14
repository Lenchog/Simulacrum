use bevy::prelude::*;
use bevy_simple_subsecond_system::hot;

#[derive(Resource)]
pub struct MouseCoordinates(pub Vec2);

#[hot]
pub fn update_mouse_coords(
    mut coords: ResMut<MouseCoordinates>,
    q_window: Single<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };
    // idk how this works, copy-pasted from
    // https://bevy-cheatbook.github.io/cookbook/cursor2world.html
    // and then fixed with clippy
    coords.0 = q_window
        .into_inner()
        .cursor_position()
        .map(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.unwrap().origin.truncate())
        .unwrap_or_default();
}
