//! Flappy bird game in Rust using bevy

use bevy::prelude::*;
use crate::PausableSystems;

mod bird;
pub(crate) mod level;
mod pipes;

// create restart event
#[derive(Event)]
pub struct Restart;

pub(super) fn plugin(app: &mut App) {

    app.add_plugins((
        bird::plugin,
        level::plugin,
        pipes::plugin,
    ));

    app.add_systems(Update, get_input.in_set(PausableSystems));
}

fn get_input(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if input.just_released(KeyCode::KeyR) {
        commands.trigger(Restart);
    };

}