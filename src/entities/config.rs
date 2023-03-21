use bevy::prelude::States;

pub const TETRIMONO_MATRIX_SIZE: usize = 4;
pub const MAX_HEIGHT: usize = 22;
pub const MAX_WIDTH: usize = 10;
pub const MIN_QUEUE_MONO_COUNT: usize = 7;
pub const INITIAL_DROP_FRAMES: i64 = 48;
pub const INITIAL_MONO_POSITION: (usize, usize) = (20, 5);
pub const HEIGHT_WITH_PADDING: usize = MAX_HEIGHT + HEIGHT_PADDING;
pub const WIDTH_WITH_PADDING: usize = MAX_WIDTH + WIDTH_PADDING;
pub const WIDTH_PADDING: usize = 4;
pub const HEIGHT_PADDING: usize = 4;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    InGame,
}
