use bevy::{
    prelude::*,
    render::mesh::{Mesh, VertexAttributeValues},
    input::mouse::{MouseMotion, MouseWheel},
    window::CursorGrabMode,
};

#[derive(Component)]
struct CameraController {
    pub enabled: bool,
    pub sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub distance: f32,
    pub angles: Vec2,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: 0.2,
            zoom_sensitivity: 0.1,
            distance: 5.0,
            angles: Vec2::new(0.0, 0.5),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_camera))
        .add_systems(Update, (rotate_brick, camera_controller))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Création d'une brique (1x2x3) avec Box (anciennement Cuboid)
    let mut mesh = Mesh::from(shape::Box {
        min_x: -1.5,  // Longueur totale = 3
        max_x: 1.5,
        min_y: -0.5,   // Épaisseur = 1
        max_y: 0.5,
        min_z: -1.0,   // Largeur = 2
        max_z: 1.0,
    });

    // Couleurs pour chaque sommet (6 faces × 4 sommets = 24 sommets)
    let colors = vec![
        // Face avant (rouge)
        Color::RED, Color::RED, Color::RED, Color::RED,
        // Face arrière (vert)
        Color::GREEN, Color::GREEN, Color::GREEN, Color::GREEN,
        // Face droite (bleu)
        Color::BLUE, Color::BLUE, Color::BLUE, Color::BLUE,
        // Face gauche (jaune)
        Color::YELLOW, Color::YELLOW, Color::YELLOW, Color::YELLOW,
        // Face haute (cyan)
        Color::CYAN, Color::CYAN, Color::CYAN, Color::CYAN,
        // Face basse (magenta)
        Color::PURPLE, Color::PURPLE, Color::PURPLE, Color::PURPLE,
    ];

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        VertexAttributeValues::Float32x4(
            colors.iter().map(|c| c.as_rgba_f32()).collect()
        ),
    );

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            unlit: true,
            ..default()
        }),
        ..default()
    });

    // Lumière
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}


fn rotate_brick(mut query: Query<&mut Transform, With<Handle<Mesh>>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
    ));
}

fn camera_controller(
    mut windows: Query<&mut Window>,
    mut mouse_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera3d>>,
    buttons: Res<Input<MouseButton>>,
) {
    let mut window = windows.single_mut();
    
    if buttons.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
    if buttons.just_pressed(MouseButton::Right) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
    
    let (mut transform, mut controller) = query.single_mut();
    
    for event in mouse_events.read() {
        if !controller.enabled || window.cursor.grab_mode != CursorGrabMode::Locked {
            continue;
        }
        
        controller.angles.x -= event.delta.x * controller.sensitivity * 0.01;
        controller.angles.y -= event.delta.y * controller.sensitivity * 0.01;
        controller.angles.y = controller.angles.y.clamp(0.1, 1.5);
    }
    
    for event in scroll_events.read() {
        controller.distance -= event.y * controller.zoom_sensitivity;
        controller.distance = controller.distance.clamp(1.0, 20.0);
    }
    
    let rotation = Quat::from_euler(
        EulerRot::YXZ,
        controller.angles.x,
        controller.angles.y,
        0.0,
    );
    
    transform.translation = rotation * Vec3::new(0.0, 0.0, controller.distance);
    transform.look_at(Vec3::ZERO, Vec3::Y);
}