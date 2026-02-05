use bevy::prelude::*;
use crate::flappy::{Restart};
use crate::flappy::bird::{Bird, SpawnBird};
use crate::screens::{Game, Screen};

pub fn plugin(app: &mut App) {
    println!("Flappy plugin loaded.");
    app.add_observer(restart_game);
}

pub fn restart_game(
    _restart: On<Restart>,
    mut commands: Commands,
    query: Query<Entity, With<Bird>>,

) {

    for bird_entity in query.iter() {
        commands.entity(bird_entity).despawn()
    }

    commands.queue(SpawnBird);
}

pub fn start_game(
    mut commands: Commands,
    //bird_assets: Res<BirdAssets>,
) {
    println!("Starting flappy game");
    commands.queue(SpawnBird);

    commands.spawn((
        Name::new("Flappy Game"),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay(Game::Flappy)),
    ));
    println!("Welcome to Flappy Game!");
}