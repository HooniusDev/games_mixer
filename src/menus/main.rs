//! The main menu (seen on the title screen).

use std::time::Duration;
use bevy::camera::NormalizedRenderTarget;
use bevy::input_focus::InputFocus;
use bevy::picking::backend::HitData;
use bevy::picking::pointer::{Location, PointerId};
use bevy::prelude::*;

use crate::my_app::Game;
use crate::{asset_tracking::ResourceHandles, menus::Menu, my_app::AppState, theme::widget};
use crate::theme::palette::BUTTON_BACKGROUND;

pub(super) fn plugin(app: &mut App) {

    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
    app.add_systems(Update, select_buttons_keyboard.run_if(in_state(Menu::Main)));
}

#[derive(Component, Debug, Reflect)]
struct MainMenu;

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        MainMenu,
        DespawnOnExit(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button("Play Demo", enter_loading_or_gameplay_demo),
            widget::button("Play Flappy", enter_loading_or_gameplay_flappy),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
            widget::button("Exit", exit_app),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button("Play Demo", enter_loading_or_gameplay_demo),
            widget::button("Play Flappy", enter_loading_or_gameplay_flappy),
            widget::button("Settings", open_settings_menu),
            widget::button("Credits", open_credits_menu),
        ],
    ));
}

// ordering goes bad when mouse touches buttons..
fn select_buttons_keyboard(
    mut input_focus: ResMut<InputFocus>,
    input: Res<ButtonInput<KeyCode>>,
    mut selected_idx: Local<usize>, // track the currently selected button
    mut buttons: Query<(Entity, &mut BackgroundColor), (With<Button>, With<ChildOf>)>,
    mut commands: Commands,
) {

    let num_buttons = buttons.iter().len();

    if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::ArrowUp) {
        *selected_idx = (*selected_idx + num_buttons + 1) % num_buttons;
    }
    if input.just_pressed(KeyCode::KeyS) || input.just_pressed(KeyCode::ArrowDown) {
        *selected_idx = (*selected_idx + num_buttons - 1) % num_buttons;
    }
    if input.just_pressed(KeyCode::Enter) || input.just_pressed(KeyCode::Space) {
        if let Some((entity, _)) = buttons.iter().nth(*selected_idx) {
            commands.trigger(Pointer::<Click> {
                entity,
                pointer_id: PointerId::Mouse,
                pointer_location: Location {
                    target: NormalizedRenderTarget::None {
                        width: 0,
                        height: 0,
                    },
                    position: Vec2::ZERO,
                },
                event: Click {
                    button: PointerButton::Primary,
                    hit: HitData {
                        camera: Entity::PLACEHOLDER,
                        depth: 0.0,
                        position: None,
                        normal: None,
                    },
                    duration: Duration::from_secs_f32(0.1),
                },
            });
        }
    }

    // loop the buttons and set colors to default, except for the selected one
    for (i, (entity, mut bg_color)) in buttons.iter_mut().enumerate() {
        if i == *selected_idx {
            *bg_color = BUTTON_BACKGROUND.into();
        } else {
            *bg_color = Color::srgb(0.225, 0.350, 0.65).into();
            input_focus.set(entity);
        }
    }
}

fn enter_loading_or_gameplay_demo(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<AppState>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(AppState::Gameplay(Game::Demo));
    } else {
        next_screen.set(AppState::Loading);
    }
}

fn enter_loading_or_gameplay_flappy(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<AppState>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(AppState::Gameplay(Game::Flappy));
    } else {
        next_screen.set(AppState::Loading);
    }
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
