use bevy::{
    prelude::*,
    render::{
        camera::Camera,
        mesh::shape,
    },
    input::mouse::{MouseMotion, MouseWheel},
    window::CursorMoved,
};

#[derive(Default)]
pub struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

pub fn camera_drag(
    mut state: ResMut<State>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut camera_query: Query<(&Camera, &mut Transform)>
) {
    if mouse_button_input.pressed(MouseButton::Right)  {
        for (_, ref mut transform) in camera_query.iter_mut() {
            for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
                let drag_vec = transform.rotation * Vec3::new(-event.delta.x(), event.delta.y(), 0f32);
                let magnitude = 0.1;

                transform.translation += drag_vec * magnitude;
            }
        }
    }
}

pub fn camera_look_around(
    mut state: ResMut<State>,
    mouse_button_input: Res<Input<MouseButton>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut camera_query: Query<(&Camera, &mut Transform)>
) {
    if mouse_button_input.pressed(MouseButton::Left)  {
        for (_, mut transform) in camera_query.iter_mut() {
            for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
                let magnitude = 0.004;
                let drag_vec = event.delta * magnitude;
                let camera_drag = Quat::from_rotation_y(drag_vec.x()) * Quat::from_rotation_x(drag_vec.y());

                transform.rotation *= camera_drag;
            }
        }
    }
}

pub fn camera_depth(
    mut state: ResMut<State>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut camera_query: Query<(&Camera, &mut Transform)>
) {
    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        for (_, mut transform) in camera_query.iter_mut() {
            let direction = quat_to_direction(transform.rotation);
            let scroll = event.y;
            transform.translation += direction * scroll;
        }
    }
}

fn quat_to_direction(quat: Quat) -> Vec3 {
    let [x, y, z, w] = quat.as_ref();

     Vec3::new(
        2f32 * (x*z + w*y),
        2f32 * (y*z - w*x),
        1f32 - 2f32 * (x*x + y*y),
     )
}

fn quat_to_up_vec(quat: Quat) -> Vec3 {
    let [x, y, z, w] = quat.as_ref();

    Vec3::new(
        2f32 * (x*y - w*z),
        1f32 - 2f32 * (x*x + z*z),
        2f32 * (y*z + w*x),
    )
}

fn quat_to_down_vec(quat: Quat) -> Vec3 {
    quat_to_up_vec(quat) * -1f32
}

fn quat_to_left_vec(quat: Quat) -> Vec3 {
    let [x, y, z, w] = quat.as_ref();

    Vec3::new(
        1f32 - 2f32 * (y*y + z*z),
        2f32 * (x*y + w*z),
        2f32 * (x*z - w*y),
    )
}

fn quat_to_right_vec(quat: Quat) -> Vec3 {
    quat_to_left_vec(quat) * -1f32
}
