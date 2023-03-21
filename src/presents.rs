use bevy::prelude::{IntoSystemAppConfig, IntoSystemConfig, OnEnter, OnUpdate, Plugin};

use crate::entities::config::AppState;

use self::system::{create_board, update_board};

pub mod components;
pub mod system;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(create_board.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_board.in_set(OnUpdate(AppState::InGame)));
    }
}
