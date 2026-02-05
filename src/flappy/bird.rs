use bevy::prelude::*;
use crate::{PausableSystems};
use crate::asset_tracking::LoadResource;
use crate::my_app::{Game};
use crate::my_app::AppState::Gameplay;

pub struct SpawnBird;

impl Command for SpawnBird {
    fn apply(self, world: &mut World) -> () {

        let assets = world.get_resource::<BirdAssets>();

        if let Some(assets) = assets {
            world.spawn((
                Transform::from_xyz(-520.0, 0.0, 0.0),
                Name::new("Bird"),
                Sprite {
                    image : assets.bird_sprite.clone(),
                    ..default()
                },
                Bird,
                Velocity::default(),
                DespawnOnExit(Gameplay(Game::Flappy)),
            ));
        }
    }
}




pub(super) fn plugin(app: &mut App) {
    app.load_resource::<BirdAssets>();

    app.add_systems(Update, (get_input, movement, apply_gravity, rotate)
        .in_set(PausableSystems)
        .run_if(in_state(Gameplay(Game::Flappy)))
    );
}


fn rotate(
    mut query: Query<(&Velocity, &mut Transform), With<Bird>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        let angle = (velocity.0 / 300.0).clamp(-1.0, 1.0) * std::f32::consts::FRAC_PI_4;
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

fn movement(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.y += velocity.0 * time.delta_secs();
    }
}

fn apply_gravity(mut query: Query<&mut Velocity>, time: Res<Time>) {
    for mut velocity in query.iter_mut() {
        velocity.0 -= 500. * time.delta_secs() ;
    }
}

#[derive(Component, Debug, Clone, Copy, Default, Reflect, Deref, DerefMut)]
struct Velocity(f32);

fn get_input(
    input: Res<ButtonInput<KeyCode>>,
    mut velocity: Single<&mut Velocity, With<Bird>>,
) {
        if input.just_pressed(KeyCode::Space) {
            velocity.0 = 300.0;
        };

}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Bird;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct BirdAssets {
    #[dependency]
    pub bird_sprite: Handle<Image>,
}

impl FromWorld for BirdAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bird_sprite: assets.load("flappy/flappy00.png"),
        }
    }

}