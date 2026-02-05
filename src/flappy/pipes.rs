use bevy::prelude::*;
use crate::asset_tracking::LoadResource;
use crate::PausableSystems;
use crate::screens::{Game, Screen};

const MOVE_SPEED: f32 = 400.0;
const RESPAWN_X: f32 = 700.0;

pub(super) fn plugin(app: &mut App) {

    app.load_resource::<PipeAssets>();
    app.init_resource::<PipeAssets>();

    app.add_systems(Startup, setup_pipes);
    app.add_systems(Update, (movement, respawn).in_set(PausableSystems));
}

fn respawn(
    mut query: Query<&mut Transform, With<Pipe>>,
) {
    for mut transform in query.iter_mut() {
        if transform.translation.x < -RESPAWN_X {
            transform.translation.x = RESPAWN_X ;
        }
    }
}

fn movement(
    mut query: Query<&mut Transform, With<Pipe>>,
    time: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= MOVE_SPEED * time.delta_secs();
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Pipe;

fn setup_pipes(
    mut commands: Commands,
    pipe_assets: Res<PipeAssets>,
) {

    // spawn upper pipe
    commands.spawn((
        Name::new("Pipe"),
        Sprite {
            image: pipe_assets.sprite.clone(),
            flip_y: true,
            ..default()
        },
        Transform::from_xyz(500.0, 400.0, 0.0),
        DespawnOnExit(Screen::Gameplay(Game::Flappy)),
        Pipe,
    ));

    //spawn lower pipe
    commands.spawn((
        Name::new("Pipe"),
        Sprite {
            image: pipe_assets.sprite.clone(),
            ..default()
        },
        Transform::from_xyz(500.0, -400.0, 0.0),
        DespawnOnExit(Screen::Gameplay(Game::Flappy)),
        Pipe,
    ));
}

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