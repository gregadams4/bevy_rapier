extern crate rapier3d as rapier; // For the debug UI.

use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy_rapier3d::physics::RapierPhysicsPlugin;
use bevy_rapier3d::render::RapierRenderPlugin;
use rapier3d::dynamics::RigidBodyBuilder;
use rapier3d::geometry::ColliderBuilder;
use rapier3d::pipeline::PhysicsPipeline;
use ui::DebugUiPlugin;

#[path = "../../src_debug_ui/mod.rs"]
mod ui;

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .insert_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_winit::WinitPlugin::default())
        .add_plugin(bevy_wgpu::WgpuPlugin::default())
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(RapierRenderPlugin)
        .add_plugin(DebugUiPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_startup_system(enable_physics_profiling.system())
        .run();
}

fn enable_physics_profiling(mut pipeline: ResMut<PhysicsPipeline>) {
    pipeline.counters.enable()
}

fn setup_graphics(mut commands: Commands) {
    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
            ..Default::default()
        })
        .spawn(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-30.0, 30.0, 100.0),
                Vec3::new(0.0, 10.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        });
}

pub fn setup_physics(mut commands: Commands) {
    /*
     * Ground
     */
    let ground_size = 50.0;
    let ground_height = 0.1;

    let rigid_body = RigidBodyBuilder::new_static().translation(0.0, -ground_height, 0.0);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height, ground_size);
    commands.spawn((rigid_body, collider));

    /*
     * Create the cubes
     */
    let num = 4;
    let rad = 0.2;

    let shift = rad * 4.0 + rad;
    let centerx = shift * (num / 2) as f32;
    let centery = shift / 2.0;
    let centerz = shift * (num / 2) as f32;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;

    for j in 0usize..20 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shift * 5.0 - centerx + offset;
                let y = j as f32 * (shift * 5.0) + centery + 3.0;
                let z = k as f32 * shift * 2.0 - centerz + offset;

                // Build the rigid body.
                let rigid_body = RigidBodyBuilder::new_dynamic().translation(x, y, z);

                // Attach multiple colliders to this rigid-body using Bevy hierarchy.
                let collider1 = ColliderBuilder::cuboid(rad * 10.0, rad, rad);
                let collider2 = ColliderBuilder::cuboid(rad, rad * 10.0, rad).translation(
                    rad * 10.0,
                    rad * 10.0,
                    0.0,
                );
                let collider3 = ColliderBuilder::cuboid(rad, rad * 10.0, rad).translation(
                    -rad * 10.0,
                    rad * 10.0,
                    0.0,
                );

                // NOTE: we need the Transform and GlobalTransform
                // so that the transform of the entity with a rigid-body
                // is properly propagated to its children with collider meshes.
                commands
                    .spawn((
                        rigid_body,
                        Transform::identity(),
                        GlobalTransform::identity(),
                    ))
                    .with_children(|parent| {
                        parent.spawn((collider1,));
                        parent.spawn((collider2,));
                        parent.spawn((collider3,));
                    });
            }
        }

        offset -= 0.05 * rad * (num as f32 - 1.0);
    }
}
