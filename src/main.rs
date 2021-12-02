use bevy::prelude::*;
use rand::Rng;

mod hitbox;

use hitbox::{intersection, Hitbox};

const GRAVITY: f32 = 9.8 * 50.0;
struct Velocity {
    translation: Vec3,
    rotation: f32,
}

fn random_sign_cringe() -> f32 {
    let mut rnd = rand::thread_rng();
    match rnd.gen_bool(0.5) {
        true => 1.,
        _ => -1.,
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut rnd = rand::thread_rng();
    let pos = (
        rnd.gen_range(200.0..500.0) * random_sign_cringe(),
        rnd.gen_range(0.0..400.0),
    );
    let dir = -pos.0 / f32::abs(pos.0);
    let velocity = Vec3::new(dir * 300., 700., 0.0);

    let transform = Transform::from_xyz(pos.0, -360., 0.0);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let hitbox_area = vec![(Vec2::new(0., 0.), Vec2::new(30., 30.))];
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
            sprite: Sprite::new(Vec2::new(30., 30.)),
            transform,
            ..Default::default()
        })
        .insert(Velocity {
            translation: velocity,
            rotation: -dir * 5.0,
        })
        .insert(Hitbox::new(hitbox_area));
}

fn click(q: Query<&Interaction>) {
    for interaction in q.iter() {
        match interaction {
            Interaction::None => (),
            _ => println!("{:?}", interaction),
        }
    }
}

fn gravity(time: Res<Time>, mut q: Query<&mut Velocity>) {
    let delta = time.delta_seconds();
    for mut v in q.iter_mut() {
        v.translation -= Vec3::new(0., delta * GRAVITY, 0.);
    }
}

fn movement(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    let delta = time.delta_seconds();
    for (v, mut t) in q.iter_mut() {
        t.translation += delta * v.translation;
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Bevy Playground".into(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        //        .add_system(movement.system())
        .add_system(click.system())
        //        .add_system(gravity.system())
        .run();
}
