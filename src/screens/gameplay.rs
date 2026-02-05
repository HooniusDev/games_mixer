//! The screen state for the main gameplay.

use bevy::{prelude::*};

use crate::{Pause, demo::level::spawn_level, menus::Menu, screens::{Screen, Game}, flappy::level::start_game};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay(Game::Demo)), spawn_level);
    app.add_systems(OnEnter(Screen::Gameplay(Game::Flappy)), start_game);

    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(should_pause),
            close_menu.run_if(should_close_menu),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay(Game::Demo)), (close_menu, unpause));
    app.add_systems(OnExit(Screen::Gameplay(Game::Flappy)), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(is_in_gameplay),
    );
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(false));
}

fn pause(mut next_pause: ResMut<NextState<Pause>>) {
    next_pause.set(Pause(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DespawnOnExit(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

/// Predicate: true when the current screen is any Gameplay variant.
fn is_in_gameplay(screen: Res<State<Screen>>) -> bool {
    matches!(screen.get(), Screen::Gameplay(_))
}

/// Predicate used for the pause toggle (checks screen, menu and keys).
fn should_pause(
    screen: Res<State<Screen>>,
    menu: Res<State<Menu>>,
    keys: Res<ButtonInput<KeyCode>>,
) -> bool {
    matches!(screen.get(), Screen::Gameplay(_))
        && *menu.get() == Menu::None
        && (keys.just_pressed(KeyCode::KeyP) || keys.just_pressed(KeyCode::Escape))
}

/// Predicate used to close menus with key press while in gameplay.
fn should_close_menu(screen: Res<State<Screen>>, menu: Res<State<Menu>>, keys: Res<ButtonInput<KeyCode>>,) -> bool {
    matches!(screen.get(), Screen::Gameplay(_))
        && *menu.get() != Menu::None
        && keys.just_pressed(KeyCode::KeyP)
}