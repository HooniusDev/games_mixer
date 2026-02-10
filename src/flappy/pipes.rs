use crate::PausableSystems;
use crate::asset_tracking::LoadResource;
use crate::my_app::AppState::Gameplay;
use crate::my_app::Game;
use bevy::prelude::*;
use rand::Rng;
use crate::flappy::bird::{is_alive};

const MOVE_SPEED: f32 = 400.0;
const RESPAWN_X: f32 = 700.0;
const SPAWN_DELAY: f32 = 1.5;


pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PipeAssets>();
    app.init_resource::<PipeAssets>();

    app.add_systems(
        Update,
        (movement, despawn, spawn)
            .run_if(is_alive)
            .in_set(PausableSystems),
    );
}



fn spawn(
    mut commands: Commands,
    pipe_assets: Res<PipeAssets>,
    time: Res<Time>,
    mut timer: Local<Timer>,
) {
    if timer.duration().is_zero() {
        *timer = Timer::from_seconds(SPAWN_DELAY, TimerMode::Repeating);
        return;
    }

    if timer.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let gap_y = rng.random_range(-200.0..200.0);

        commands.spawn((
            Name::new("Pipe"),
            Sprite {
                image: pipe_assets.sprite.clone(),
                flip_y: true,
                ..default()
            },
            Transform::from_xyz(RESPAWN_X, 400.0 - gap_y, 0.0),
            DespawnOnExit(Gameplay(Game::Flappy)),
            Pipe,
        ));

        //spawn lower pipe
        commands.spawn((
            Name::new("Pipe"),
            Sprite {
                image: pipe_assets.sprite.clone(),
                ..default()
            },
            Transform::from_xyz(RESPAWN_X, -400.0 - gap_y, 0.0),
            DespawnOnExit(Gameplay(Game::Flappy)),
            Pipe,
        ));
    }
    //timer.tick(time.delta());
}

fn despawn(mut commands: Commands, query: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, transform) in query {
        if transform.translation.x < -RESPAWN_X {
            commands.entity(entity).despawn();
        }
    }
}

fn movement(mut query: Query<&mut Transform, With<Pipe>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= MOVE_SPEED * time.delta_secs();
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Pipe;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PipeAssets {
    #[dependency]
    pub sprite: Handle<Image>,
}

impl FromWorld for PipeAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            sprite: assets.load("flappy/pipe.png"),
        }
    }
}
