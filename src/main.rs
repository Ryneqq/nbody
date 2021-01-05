mod body;
mod camera_system;

use self::body::Body;
use self::camera_system::*;
use bevy::prelude::*;
use itertools::Itertools;
use rand::{Rng, thread_rng};

pub type Float = f32;
pub type Vector = Vec3;
pub type Point = Vec3;

use bevy::{
    prelude::*,
    render::{
        camera::Camera,
        mesh::shape,
    },
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    window::CursorMoved,
};

fn main() {
    App::build()
        .init_resource::<State>()
        .init_resource::<GarvityTimer>()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(camera_drag.system())
        .add_system(camera_look_around.system())
        .add_system(camera_depth.system())
        .add_system(gravity_system.system())
        .run();
}

struct GarvityTimer {
    timer: Timer,
}

impl Default for GarvityTimer {
    fn default() -> Self {
        GarvityTimer {
            timer: Timer::from_seconds(1.0, true),
        }
    }
}

fn gravity_system(
    mut commands: Commands,
    time: Res<Time>,
    mut state: ResMut<GarvityTimer>,
    mut body_query: Query<(Entity, &mut Body, &mut Transform)>
) {
    state.timer.tick(time.delta_seconds);

    if state.timer.finished {
        let body_copies = body_query.iter_mut()
            .map(|(entity, body, _)| (entity, (*body).clone()))
            .map(|(e, body)| {
                if body.id() == 9999999 {
                    dbg!(&body);
                }

                (e, body)
            })
            .collect_vec();

        for (entity, mut body, mut transform) in body_query.iter_mut() {
            for (copy_entity, copy) in body_copies.iter() {
                if copy.id() != body.id() {
                    body.apply_gravity(copy);

                    if body.colliding(copy) {
                        println!("Merging {} with {}", body.id(), copy.id());
                        *body = body.clone().merge(copy.clone());
                        commands.despawn(*copy_entity);
                    } else {
                        transform.translation = body.position();
                    }
                }
            }

            body.step();
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let bodies = (0..30)
        .map(|id| random_body(id))
        .chain(Some(black_hole()))
        .for_each(|body| {
            commands
                .spawn(PbrComponents {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        subdivisions: 4,
                        radius: body.radius(),
                    })),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                    transform: Transform::from_translation(body.position()),
                    ..Default::default()
                })
                .with(body);
        });

    commands
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            ..Default::default()
        });
}

const V: f64 = 1f64;

fn random_body(id: usize) -> Body {
    let mut rng = thread_rng();
    let mass = rng.gen_range(10f32.powi(5), 10f32.powi(7));
    let x = rng.gen_range(-1.0, 1.0);
    let y = rng.gen_range(-1.0, 1.0);
    let z = 0f32;
    let position = Point::new(x, y, z);
    let v_x = rng.gen_range(-V, V) as f32;
    let v_y = rng.gen_range(-V, V) as f32;
    let v_z = 0f32;
    let velocity = Vector::new(v_x, v_y, v_z);

    Body::new(id, mass, position, velocity)
}

fn black_hole() -> Body {
    let mass = 10f32.powi(10);
    let position = Point::new(0., 0., 0.);
    let velocity = Vector::new(0., 0., 0.);

    Body::new(9999999, mass, position, velocity)
}
