use bevy::prelude::*;
use visualizer::{update_revolute_joints, Link, RevoluteJoint};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 4_000.,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (rotating_system, update_revolute_joints))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cylinder::new(1.0, 5.0)),
                material: materials.add(Color::rgb_u8(124, 144, 255)),
                transform: Transform::from_xyz(0.0, 0.5, 0.0),
                ..default()
            },
            Link,
            RevoluteJoint {
                translation: Vec3::new(0., 3., 0.),
                axis: Vec3::Z,
                angle: 0.,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Cylinder::new(1.0, 5.0)),
                    material: materials.add(Color::rgb_u8(124, 144, 255)),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..default()
                },
                Link,
                RevoluteJoint {
                    translation: Vec3::new(0., 3., 0.),
                    axis: Vec3::Z,
                    angle: 0.,
                },
            ));
        });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotating_system(time: Res<Time>, mut query: Query<&mut RevoluteJoint>) {
    for mut joint in &mut query {
        let rotation_angle = time.elapsed_seconds() * 0.5;
        joint.angle = rotation_angle;
    }
}
