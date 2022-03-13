use bevy::prelude::*;

use crate::{assets::Images, Camera};

#[derive(Component)]
pub struct EndSceneOne(pub Timer);

pub fn setup(mut commands: Commands, images: Res<Images>, camera: Query<Entity, With<Camera>>) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            texture: images.end_1.clone(),
            ..Default::default()
        })
        .insert(Name::new("End Scene 1"))
        .insert(EndSceneOne(Timer::from_seconds(5., false)));

    let camera = camera.single();
    commands.entity(camera).despawn();

    let new_camera_bundle = OrthographicCameraBundle::new_2d();
    commands
        .spawn_bundle(new_camera_bundle)
        .insert(Name::new("Camera"))
        .insert(Camera);
}

pub fn switch_end_scene(
    mut commands: Commands,
    images: Res<Images>,
    mut end_one: Query<(Entity, &mut EndSceneOne)>,
    time: Res<Time>,
) {
    if let Some((entity, mut end_one)) = end_one.iter_mut().next() {
        end_one.0.tick(time.delta());
        if end_one.0.finished() {
            commands.entity(entity).despawn();

            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                    texture: images.end_2.clone(),
                    ..Default::default()
                })
                .insert(Name::new("End Scene 2"));
        }
    }
}
