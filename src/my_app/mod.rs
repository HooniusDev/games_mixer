//! The game's main screen states and transitions between them.

mod gameplay;
mod loading;
mod splash;
mod title;

use crate::demo::DemoGamePlugin;
use crate::flappy::FlappyGamePlugin;
use bevy::prelude::*;

#[derive(Event)]
pub struct StartGameEvent;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Game {
    Demo,
    Flappy,
}

#[derive(Resource, Debug, Deref, DerefMut)]
pub struct CurrentGame(pub Option<Game>);

impl Default for CurrentGame {
    fn default() -> Self {
        CurrentGame(None)
    }
}

impl AppState {
    pub fn is_gameplay(&self) -> bool {
        matches!(self, AppState::Gameplay(_))
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_state::<AppState>();
    app.init_resource::<CurrentGame>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));

    app.add_plugins((FlappyGamePlugin, DemoGamePlugin));
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum AppState {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay(Game),
}
