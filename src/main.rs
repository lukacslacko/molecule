use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (camera_movement, camera_rotation))
        .run();
}

#[derive(Component)]
struct CameraController {
    pub sensitivity: f32,
    pub speed: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create several spheres at different positions
    let sphere_mesh = meshes.add(Sphere::new(0.5).mesh().ico(5).unwrap());
    
    // Sphere 1 - Red
    commands.spawn((
        Mesh3d(sphere_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    // Sphere 2 - Green
    commands.spawn((
        Mesh3d(sphere_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_xyz(2.0, 0.0, 0.0),
    ));
    
    // Sphere 3 - Blue
    commands.spawn((
        Mesh3d(sphere_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.0, 1.0),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_xyz(-2.0, 0.0, 0.0),
    ));
    
    // Sphere 4 - Yellow
    commands.spawn((
        Mesh3d(sphere_mesh.clone()),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 0.0),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            ..default()
        })),
        Transform::from_xyz(0.0, 2.0, 0.0),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });

    // Light 1 - Above and to the left
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-4.0, 8.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Light 2 - Above and to the right
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController {
            sensitivity: 0.002,
            speed: 5.0,
        },
    ));
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &CameraController)>,
) {
    for (mut transform, controller) in query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let forward = *transform.forward();
        let right = *transform.right();

        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            velocity -= forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity -= right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity += right;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            velocity += Vec3::Y;
        }
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            velocity -= Vec3::Y;
        }

        velocity = velocity.normalize_or_zero();
        transform.translation += velocity * controller.speed * time.delta_secs();
    }
}

fn camera_rotation(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &CameraController)>,
) {
    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    for event in mouse_motion_events.read() {
        for (mut transform, controller) in query.iter_mut() {
            let delta_x = event.delta.x * controller.sensitivity;
            let delta_y = event.delta.y * controller.sensitivity;

            // Rotate around the Y axis (yaw)
            let yaw = Quat::from_rotation_y(-delta_x);
            
            // Rotate around the local X axis (pitch)
            let right = transform.right();
            let pitch = Quat::from_axis_angle(right.into(), -delta_y);

            transform.rotation = yaw * transform.rotation;
            transform.rotation = pitch * transform.rotation;
        }
    }
}
