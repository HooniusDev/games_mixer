//! Flappy bird game in Rust using bevy

use bevy::prelude::*;
use crate::flappy::bird::{SpawnBird};
use crate::{PausableSystems};
use crate::my_app::{Game};
use crate::my_app::AppState::Gameplay;

mod bird;
pub(crate) mod level;
mod pipes;

// add state for flappy game

// Declare events

// create restart event
#[derive(Event)]
pub struct Restart;
#[derive(Event)]
struct Start;

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub(crate) struct GameRoot(pub(crate) Option<Entity>);

pub struct FlappyGamePlugin;

impl Plugin for FlappyGamePlugin{
    fn build(&self, app: &mut App) {
        // add sub plugins
        app.add_plugins((
            bird::plugin,
            level::plugin,
            pipes::plugin,
        ));



        app.insert_resource(GameRoot{ 0: None });

        // add event handlers
      app.add_systems(OnEnter(Gameplay(Game::Flappy)), on_enter);

        // only get input when in gameplay state and the game is not paused
        app.add_systems(Update, (get_input)
            .in_set(PausableSystems)
            .run_if(in_state(Gameplay(Game::Flappy)))
        );
        app.add_observer(restart_game);
        app.add_observer(start_game);
    }
}

fn on_enter(
    mut commands: Commands,
) {
    println!("Entered Gameplay state");
        commands.trigger(Start);
}


/// Restart the game by despawning all entities.
fn restart_game(
    _event: On<Restart>,
    mut commands: Commands,
    bird: Query<Entity, With<bird::Bird>>,
    pipes: Query<Entity, With<pipes::Pipe>>,
) {
    println!("Restart");
    for bird_entity in bird.iter() {
        commands.entity(bird_entity).despawn() // Despawn the game
    }
    for pipe_entity in pipes.iter() {
        commands.entity(pipe_entity).despawn() // Despawn the game
    }
    commands.trigger(Start);
}

/// Start the game by spawning the bird and setting up the game state.
fn start_game(
    _event: On<Start>,
    mut commands: Commands,
    mut game_root: ResMut<GameRoot>,
) {

    let game_entity = commands.spawn((
        Name::new("Flappy Game"),
        Visibility::default(),
        DespawnOnExit(Gameplay(Game::Flappy)),
    )).id();

    game_root.0 = Some(game_entity);

    commands.queue(SpawnBird);

}

fn get_input(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if input.just_released(KeyCode::KeyR) {
        println!("Good bye!");
        commands.trigger(Restart);
    };

}