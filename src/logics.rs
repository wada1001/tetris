use bevy::prelude::{IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin};

use crate::entities::config::AppState;

use self::system::{handle_input, initialize_game, tick_board};

pub mod system;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(initialize_game.in_schedule(OnEnter(AppState::InGame)))
            .add_system(handle_input.in_set(OnUpdate(AppState::InGame)))
            .add_system(tick_board.in_set(OnUpdate(AppState::InGame)));
    }
}
