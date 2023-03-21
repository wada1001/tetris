use bevy::prelude::*;

use crate::entities::{
    config::AppState,
    counter::FrameCounter,
    event::BoardUpdateEvent,
    tetris::{GameBoard, MoveDirection, StockTetrimono, TetrimonoQueue},
};

pub fn initialize_game(mut commands: Commands) {
    let mut queue = TetrimonoQueue::default();

    while !queue.is_enough_mono() {
        queue.add_set(true);
    }

    let mut game_board = GameBoard::default();
    game_board.init_tetrimono(queue.dequeue());

    commands.insert_resource(game_board);
    commands.insert_resource(queue);
    commands.insert_resource(StockTetrimono::default());
    commands.insert_resource(FrameCounter::default());
}

pub fn tick_board(
    mut next_state: ResMut<NextState<AppState>>,
    mut event_w: EventWriter<BoardUpdateEvent>,
    mut counter: ResMut<FrameCounter>,
    mut mono_queue: ResMut<TetrimonoQueue>,
    mut game_board: ResMut<GameBoard>,
) {
    counter.tick();

    if !counter.is_finished() {
        return;
    }
    event_w.send_default();

    if game_board.simulate_move(MoveDirection::Down) {
        // can drop
        game_board.apply_move(MoveDirection::Down);
    } else {
        // cannot drop
        game_board.write_tetrimono();
        game_board.init_tetrimono(mono_queue.dequeue());

        if game_board.is_game_over() {
            next_state.set(AppState::InGame);
        }

        let clear_count = game_board.clear_lines();

        while !mono_queue.is_enough_mono() {
            mono_queue.add_set(true);
        }
    }

    counter.reset();
}

// 更新したらイベントを出そう

pub fn handle_input(
    mut next_state: ResMut<NextState<AppState>>,
    mut event_w: EventWriter<BoardUpdateEvent>,
    kc: Res<Input<KeyCode>>,
    // input_counter: Local<(i64, i64)>,
    // mut stock_mono: ResMut<StockTetrimono>,
    // mut mono_queue: ResMut<TetrimonoQueue>,
    mut game_board: ResMut<GameBoard>,
    mut counter: ResMut<FrameCounter>,
) {
    if kc.just_pressed(KeyCode::Left) {
        if !game_board.simulate_move(MoveDirection::Left) {
            return;
        }
        game_board.apply_move(MoveDirection::Left);
    } else if kc.just_pressed(KeyCode::Right) {
        if !game_board.simulate_move(MoveDirection::Right) {
            return;
        }
        game_board.apply_move(MoveDirection::Right);
    } else if kc.just_pressed(KeyCode::Up) {
        if game_board.try_rotate(true) {
            return;
        }
    } else if kc.just_pressed(KeyCode::Space) {
        while game_board.simulate_move(MoveDirection::Down) {
            game_board.apply_move(MoveDirection::Down);
        }
        // 次の時間経過ですぐに処理させる
        counter.force_finish();
    } else if kc.just_pressed(KeyCode::Down) {
        if !game_board.simulate_move(MoveDirection::Down) {
            return;
        }
        game_board.apply_move(MoveDirection::Down);
    } else if kc.just_pressed(KeyCode::R) {
        next_state.set(AppState::InGame);
    }

    event_w.send_default();
}

#[test]
fn larger_can_hold_smaller() {}
