use bevy::prelude::*;

const SCALE_FACTOR: f32 = 4.;
const SPRITE_SIZE: f32 = 16.;

struct Sherif;

// STARTUP SETUP
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let sherif_sprite = asset_server.load("sprites/sherif.png");
    let window = windows.get_primary().expect("Windows should be spawned");
    let bottom = window.height() / 2.;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(sherif_sprite.into()),
            transform: Transform {
                translation: Vec3::new(0., (SPRITE_SIZE * SCALE_FACTOR), 10.),
                scale: Vec3::new(SCALE_FACTOR, SCALE_FACTOR, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Sherif);
    println!("bottom: {}", bottom);
}

fn movement(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Sherif>>) {
    if let Ok(mut transform) = query.single_mut() {
        let dir_y = if keyboard_input.pressed(KeyCode::W) {
            1.
        } else if keyboard_input.pressed(KeyCode::S) {
            -1.
        } else {
            0.
        };
        let dir_x = if keyboard_input.pressed(KeyCode::A) {
            -1.
        } else if keyboard_input.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };
        transform.translation.y += dir_y;
        transform.translation.x += dir_x;
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(movement.system())
        .run();
}
