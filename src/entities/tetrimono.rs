use super::config::TETRIMONO_MATRIX_SIZE;

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub figure: [[u8; 4]; 4],
    pub matrix_size: usize,
    pub rotation: i64,
}

#[derive(Clone, Copy)]
pub enum TetrominoType {
    T = 1,
    L = 2,
    J = 3,
    O = 4,
    S = 5,
    Z = 6,
    I = 7,
}

impl TetrominoType {
    pub fn get_tetromono(self) -> Tetromino {
        return Tetromino::from(self);
    }

    pub fn all() -> Vec<TetrominoType> {
        return Vec::from([
            TetrominoType::T,
            TetrominoType::L,
            TetrominoType::J,
            TetrominoType::O,
            TetrominoType::S,
            TetrominoType::Z,
            TetrominoType::I,
        ]);
    }
}

impl Tetromino {
    fn new(figure: [[u8; 4]; 4], matrix_size: usize) -> Self {
        return Tetromino {
            figure: figure,
            matrix_size: matrix_size,
            rotation: 0,
        };
    }

    // 0,0,0,0
    // 0,1,0,0
    // 1,1,1,0
    // 0,0,0,0
    fn type_t() -> Self {
        let mut figure = [[0; TETRIMONO_MATRIX_SIZE]; TETRIMONO_MATRIX_SIZE];
        figure[1][0] = 1;
        figure[1][1] = 1;
        figure[1][2] = 1;
        figure[2][1] = 1;
        return Tetromino::new(figure, 3);
    }

    // 0,0,0,0
    // 0,0,1,0
    // 1,1,1,0
    // 0,0,0,0
    fn type_l() -> Self {
        let mut figure = [[0; TETRIMONO_MATRIX_SIZE]; TETRIMONO_MATRIX_SIZE];
        figure[1][0] = 2;
        figure[1][1] = 2;
        figure[1][2] = 2;
        figure[2][2] = 2;
        return Tetromino::new(figure, 3);
    }

    // 0,0,0,0
    // 1,0,0,0
    // 1,1,1,0
    // 0,0,0,0
    fn type_j() -> Self {
        let mut figure = [[0; TETRIMONO_MATRIX_SIZE]; TETRIMONO_MATRIX_SIZE];
        figure[1][0] = 3;
        figure[1][1] = 3;
        figure[1][2] = 3;
        figure[2][0] = 3;
        return Tetromino::new(figure, 3);
    }

    // 0,0,0,0
    // 0,0,0,0
    // 1,1,0,0
    // 1,1,0,0
    fn type_o() -> Self {
        let mut figure = [[0; TETRIMONO_MATRIX_SIZE]; TETRIMONO_MATRIX_SIZE];
        figure[0][0] = 4;
        figure[0][1] = 4;
        figure[1][0] = 4;
        figure[1][1] = 4;
        return Tetromino::new(figure, 0);
    }

    // 0,0,0,0
    // 0,1,1,0
    // 1,1,0,0
    // 0,0,0,0
    fn type_s() -> Self {
        let mut figure = [[0; TETRIMONO_MATRIX_SIZE]; TETRIMONO_MATRIX_SIZE];
        figure[1][0] = 5;
        figure[1][1] = 5;
        figure[2][1] = 5;
        figure[2][2] = 5;
        return Tetromino::new(figure, 3);
    }

    // 0,0,0,0
    // 1,1,0,0
    // 0,1,1,0
    // 0,0,0,0
    fn type_z() -> Self {
        let mut figure = [[0; TETRIMONO_MATRIX_SIZE]; TETRIMONO_MATRIX_SIZE];
        figure[1][1] = 6;
        figure[1][2] = 6;
        figure[2][0] = 6;
        figure[2][1] = 6;
        return Tetromino::new(figure, 3);
    }

    // 0,0,0,0
    // 0,0,0,0
    // 1,1,1,1
    // 0,0,0,0
    fn type_i() -> Self {
        let mut figure = [[0; TETRIMONO_MATRIX_SIZE]; TETRIMONO_MATRIX_SIZE];
        figure[1][0] = 7;
        figure[1][1] = 7;
        figure[1][2] = 7;
        figure[1][3] = 7;
        return Tetromino::new(figure, 4);
    }

    pub fn from(typ: TetrominoType) -> Self {
        match typ {
            TetrominoType::I => Tetromino::type_i(),
            TetrominoType::J => Tetromino::type_j(),
            TetrominoType::Z => Tetromino::type_z(),
            TetrominoType::S => Tetromino::type_s(),
            TetrominoType::L => Tetromino::type_l(),
            TetrominoType::O => Tetromino::type_o(),
            TetrominoType::T => Tetromino::type_t(),
        }
    }

    pub fn rotate_right(&mut self) {
        if self.matrix_size < 1 {
            return;
        }
        let x = self.matrix_size / 2;
        let y = self.matrix_size - 1;

        let mut j = 0;
        while j < x {
            let mut i = j;
            while i < y - j {
                let tmp = self.figure[i][j];
                self.figure[i][j] = self.figure[j][y - i];
                self.figure[j][y - i] = self.figure[y - i][y - j];
                self.figure[y - i][y - j] = self.figure[y - j][i];
                self.figure[y - j][i] = tmp;
                i += 1;
            }
            j += 1;
        }
        self.rotation += 90;
    }

    // TODO まだ
    pub fn rotate_left(&mut self) {
        if self.matrix_size < 1 {
            return;
        }
        let x = self.matrix_size / 2;
        let y = self.matrix_size - 1;

        let mut j = 0;
        while j < x {
            let mut i = j;
            while i < y - j {
                let tmp = self.figure[i][j];
                self.figure[i][j] = self.figure[j][y - i];
                self.figure[j][y - i] = self.figure[y - i][y - j];
                self.figure[y - i][y - j] = self.figure[y - j][i];
                self.figure[y - j][i] = tmp;
                i += 1;
            }
            j += 1;
        }
        self.rotation -= 90;
    }
}
