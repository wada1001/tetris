use bevy::prelude::Resource;
use rand::seq::SliceRandom;
use rand::thread_rng;

use super::config::{
    HEIGHT_PADDING, HEIGHT_WITH_PADDING, INITIAL_MONO_POSITION, MAX_HEIGHT, MAX_WIDTH,
    MIN_QUEUE_MONO_COUNT, WIDTH_WITH_PADDING,
};
use super::tetrimono::{Tetromino, TetrominoType};

pub enum MoveDirection {
    Down,
    Left,
    Right,
}

#[derive(Resource)]
pub struct GameBoard {
    pub board: [[u8; WIDTH_WITH_PADDING]; HEIGHT_WITH_PADDING],
    pub cursor: (usize, usize),
    pub tetrimono: Option<Tetromino>,
}

#[derive(Resource, Default)]
pub struct TetrimonoQueue {
    pub queue: Vec<TetrominoType>,
}

#[derive(Resource, Default, Clone, Copy)]
pub struct CurrentTetrimono {
    pub cursor: (usize, usize),
    pub mono: Option<Tetromino>,
}

#[derive(Resource, Default)]
pub struct StockTetrimono {
    pub mono: Option<Tetromino>,
}

impl Default for GameBoard {
    fn default() -> Self {
        let mut board = [[0; WIDTH_WITH_PADDING]; HEIGHT_WITH_PADDING];
        for h in 0..(HEIGHT_WITH_PADDING) {
            board[h][0] = 9;
            board[h][1] = 9;
            board[h][WIDTH_WITH_PADDING - 1] = 9;
            board[h][WIDTH_WITH_PADDING - 2] = 9;
        }

        for w in 0..(WIDTH_WITH_PADDING) {
            board[0][w] = 9;
            board[1][w] = 9;
        }
        return GameBoard {
            board: board,
            cursor: (0, 0),
            tetrimono: None,
        };
    }
}

impl GameBoard {
    pub fn get_written_clone(&self) -> [[u8; WIDTH_WITH_PADDING]; HEIGHT_WITH_PADDING] {
        if self.tetrimono.is_none() {
            return self.board.clone();
        }

        let mut board_clone = self.board.clone();
        let tetrimono = self.tetrimono.unwrap();
        for y in 0..4 {
            for x in 0..4 {
                if tetrimono.figure[y][x] == 0 {
                    continue;
                }
                board_clone[self.cursor.0 + y][self.cursor.1 + x] = tetrimono.figure[y][x];
            }
        }
        return board_clone;
    }

    pub fn write_tetrimono(&mut self) {
        if self.tetrimono.is_none() {
            return;
        }

        let tetrimono = self.tetrimono.unwrap();
        for y in 0..4 {
            for x in 0..4 {
                if tetrimono.figure[y][x] == 0 {
                    continue;
                }
                self.board[self.cursor.0 + y][self.cursor.1 + x] = tetrimono.figure[y][x];
            }
        }
        self.tetrimono = None;
    }

    pub fn init_tetrimono(&mut self, tetrimono: Tetromino) {
        self.cursor = INITIAL_MONO_POSITION;
        self.tetrimono = Some(tetrimono);
    }
    pub fn simulate_move(&self, dir: MoveDirection) -> bool {
        if self.tetrimono.is_none() {
            return false;
        }

        let tetrimono = self.tetrimono.unwrap();
        let mut cursor = self.cursor.clone();

        cursor = match dir {
            MoveDirection::Down => (cursor.0 - 1, cursor.1),
            MoveDirection::Left => (cursor.0, cursor.1 - 1),
            MoveDirection::Right => (cursor.0, cursor.1 + 1),
        };

        return !GameBoard::check_duplicate(self.board, tetrimono, cursor);
    }

    pub fn apply_move(&mut self, dir: MoveDirection) {
        self.cursor = match dir {
            MoveDirection::Down => (self.cursor.0 - 1, self.cursor.1),
            MoveDirection::Left => (self.cursor.0, self.cursor.1 - 1),
            MoveDirection::Right => (self.cursor.0, self.cursor.1 + 1),
        };
    }

    pub fn try_rotate(&mut self, is_right: bool) -> bool {
        if self.tetrimono.is_none() {
            return false;
        }

        let mut tetrimono = self.tetrimono.unwrap();
        let mut cursor = self.cursor;

        if is_right {
            tetrimono.rotate_right();
        } else {
            tetrimono.rotate_left();
        }

        if !GameBoard::check_duplicate(self.board, tetrimono, cursor) {
            self.tetrimono = Some(tetrimono);
            self.cursor = cursor;
            return true;
        }

        if tetrimono.matrix_size == 4 {
            // TODO Imino
            return false;
        }

        let angle = tetrimono.rotation % 180;

        if angle != 0 && angle > 0 {
            cursor = (cursor.0, cursor.1 - 1);
        } else if angle != 0 && angle < 0 {
            cursor = (cursor.0, cursor.1 + 1);
        } else if angle == 0 && is_right {
            cursor = (cursor.0, cursor.1 - 1);
        } else if angle == 0 && !is_right {
            cursor = (cursor.0, cursor.1 + 1);
        } else {
            panic!("srs 1. failed");
        }

        if !GameBoard::check_duplicate(self.board, tetrimono, cursor) {
            self.tetrimono = Some(tetrimono);
            self.cursor = cursor;
            return true;
        }

        if angle != 0 {
            cursor = (cursor.0 + 1, cursor.1);
        } else {
            cursor = (cursor.0 - 1, cursor.1);
        }

        if !GameBoard::check_duplicate(self.board, tetrimono, cursor) {
            self.tetrimono = Some(tetrimono);
            self.cursor = cursor;
            return true;
        }

        if angle != 0 {
            cursor = (self.cursor.0 + 1, self.cursor.1);
        } else {
            cursor = (self.cursor.0 - 1, self.cursor.1);
        }

        if !GameBoard::check_duplicate(self.board, tetrimono, cursor) {
            self.tetrimono = Some(tetrimono);
            self.cursor = cursor;
            return true;
        }

        if angle != 0 && angle > 0 {
            cursor = (cursor.0, cursor.1 - 1);
        } else if angle != 0 && angle < 0 {
            cursor = (cursor.0, cursor.1 + 1);
        } else if angle == 0 && is_right {
            cursor = (cursor.0, cursor.1 - 1);
        } else if angle == 0 && !is_right {
            cursor = (cursor.0, cursor.1 + 1);
        } else {
            panic!("srs 4. failed");
        }

        if !GameBoard::check_duplicate(self.board, tetrimono, cursor) {
            self.tetrimono = Some(tetrimono);
            self.cursor = cursor;
            return true;
        }

        return false;
    }

    fn check_duplicate(
        board: [[u8; WIDTH_WITH_PADDING]; HEIGHT_WITH_PADDING],
        tetrimono: Tetromino,
        cursor: (usize, usize),
    ) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if tetrimono.figure[y][x] == 0 {
                    continue;
                }
                if tetrimono.figure[y][x] * board[cursor.0 + y][cursor.1 + x] != 0 {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn is_game_over(&mut self) -> bool {
        if self.tetrimono.is_none() {
            return true;
        }

        return GameBoard::check_duplicate(self.board, self.tetrimono.unwrap(), self.cursor);
    }

    // 以下雑
    pub fn clear_lines(&mut self) -> u8 {
        let start_pos = HEIGHT_PADDING / 2;
        let mut clear_line_count = 0;

        let mut y = start_pos;
        while y < start_pos + MAX_HEIGHT {
            if !self.clear_line(y) {
                y += 1;
                continue;
            }

            self.shift_lines(y); // TODO スゴクムダ？
            clear_line_count += 1;
        }

        return clear_line_count;
    }

    fn clear_line(&mut self, y: usize) -> bool {
        let start_pos = HEIGHT_PADDING / 2;
        for x in start_pos..(start_pos + MAX_WIDTH) {
            if self.board[y][x] == 0 || self.board[y][x] == 9 {
                return false;
            }
        }

        for x in start_pos..(start_pos + MAX_WIDTH) {
            self.board[y][x] = 0;
        }
        return true;
    }

    fn shift_lines(&mut self, from_y: usize) {
        let start_pos = HEIGHT_PADDING / 2;

        let mut y = from_y;
        while y < start_pos + MAX_HEIGHT {
            for x in start_pos..(start_pos + MAX_WIDTH) {
                self.board[y][x] = self.board[y + 1][x];
            }
            y += 1;
        }
    }
}

impl TetrimonoQueue {
    pub fn is_enough_mono(&mut self) -> bool {
        return self.queue.len() >= MIN_QUEUE_MONO_COUNT;
    }

    pub fn add_set(&mut self, is_random: bool) {
        let mut set_of_mono = TetrominoType::all();
        if is_random {
            set_of_mono.shuffle(&mut thread_rng());
        }

        self.queue.extend(set_of_mono);
    }

    pub fn dequeue(&mut self) -> Tetromino {
        let mono_typ = self.queue.remove(0);
        return mono_typ.get_tetromono();
    }
}
