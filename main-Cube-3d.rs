use bevy::prelude::*;
use bevy::render::mesh::{Mesh, VertexAttributeValues};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Création d'un cube avec des couleurs différentes par face
    let mut mesh = Mesh::from(shape::Cube { size: 1.0 });

    // Couleurs pour chaque sommet (6 faces × 4 sommets = 24 sommets)
    let colors = vec![
        // Face avant (rouge)
        Color::RED,
        Color::RED,
        Color::RED,
        Color::RED,
        // Face arrière (vert)
        Color::GREEN,
        Color::GREEN,
        Color::GREEN,
        Color::GREEN,
        // Face droite (bleu)
        Color::BLUE,
        Color::BLUE,
        Color::BLUE,
        Color::BLUE,
        // Face gauche (jaune)
        Color::YELLOW,
        Color::YELLOW,
        Color::YELLOW,
        Color::YELLOW,
        // Face haute (cyan)
        Color::CYAN,
        Color::CYAN,
        Color::CYAN,
        Color::CYAN,
        // Face basse (magenta)
        Color::PURPLE,
        Color::PURPLE,
        Color::PURPLE,
        Color::PURPLE,
    ];

    // On applique les couleurs au mesh
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        VertexAttributeValues::Float32x4(
            colors.iter().map(|c| c.as_rgba_f32()).collect()
        ),
    );

    // On crée une entité avec le cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            // Désactive l'éclairage métallique pour mieux voir les couleurs
            unlit: true,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // Ajout d'une lumière
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Ajout d'une caméra
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotate_cube(mut query: Query<&mut Transform, With<Handle<Mesh>>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 0.5); // Rotation lente
    }
}
