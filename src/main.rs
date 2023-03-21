use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{ExitCondition, WindowResolution},
};
use entities::{config::AppState, event::BoardUpdateEvent, tetris::GameBoard};
use logics::LogicPlugin;
use presents::RenderPlugin;

mod entities;
mod logics;
mod presents;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_event::<BoardUpdateEvent>()
        .insert_resource(GameBoard::default())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(FixedTime::new_from_secs(1. / 60.))
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Tetris"),
                resizable: true,
                resolution: WindowResolution::new(500., 676.),
                ..Default::default()
            }),
            exit_condition: ExitCondition::OnAllClosed,
            close_when_requested: true,
        }))
        .add_plugin(LogicPlugin)
        .add_plugin(RenderPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands, mut state: ResMut<NextState<AppState>>) {
    commands.spawn(Camera2dBundle::default());
    state.set(AppState::InGame);
}
