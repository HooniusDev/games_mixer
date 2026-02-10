//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions,
    input::common_conditions::input_just_pressed,
    prelude::*,
    color::palettes::css::*,
};

use crate::my_app::AppState;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<AppState>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    // app.add_systems(
    //     Update,
    //     draw_flappy_hitboxes.run_if(in_state(AppState::Gameplay(crate::my_app::Game::Flappy))),
    // );
}

fn _draw_flappy_hitboxes(
    mut gizmos: Gizmos,
    bird_query: Single<&Transform, With<crate::flappy::bird::Bird>>,
    pipe_query: Query<&Transform, With<crate::flappy::pipes::Pipe>>,
) {
    let bird = *bird_query;
    gizmos.circle_2d(
        Isometry2d::from_translation(bird.translation.truncate() + Vec2::new(5., 3.)),
        32.0,
        RED,
    );

    let pipe_size = Vec2::new(101., 603.0);

    for pipe_transform in pipe_query.iter() {
        gizmos.rect_2d(
            pipe_transform.translation.truncate(),
            pipe_size,
            PINK,
        );
    }
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
