use bevy::prelude::*;

use crate::entities::{event::BoardUpdateEvent, tetris::GameBoard};

use super::components::BoardUnit;

const BOARD_UNIT_SIZE: f32 = 20.;
const BOARD_BOTTOM_LEFT: Vec3 = Vec3::new(-7. * BOARD_UNIT_SIZE, -12. * BOARD_UNIT_SIZE, 0.);

pub fn create_board(mut commands: Commands, board: Res<GameBoard>) {
    for (i, row) in board.board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let _y = i as f32;
            let _x = j as f32;
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: match_color(*col),
                        custom_size: Some(Vec2::new(1., 1.) * BOARD_UNIT_SIZE),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: BOARD_BOTTOM_LEFT
                            + Vec3::new(_x * BOARD_UNIT_SIZE, _y * BOARD_UNIT_SIZE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(BoardUnit((j, i)));
        }
    }
}

pub fn update_board(
    mut event_r: EventReader<BoardUpdateEvent>,
    board: Res<GameBoard>,
    mut query: Query<(&mut BoardUnit, &mut Sprite)>,
) {
    if event_r.is_empty() {
        return;
    }

    let clone = board.get_written_clone();
    for (unit, mut sprite) in query.iter_mut() {
        let cell = clone[unit.0 .1][unit.0 .0];
        sprite.color = match_color(cell);
    }

    event_r.clear();
}

fn match_color(cell: u8) -> Color {
    match cell {
        0 => Color::GRAY,
        1 => Color::PURPLE,
        2 => Color::ORANGE,
        3 => Color::BLUE,
        4 => Color::YELLOW,
        5 => Color::GREEN,
        6 => Color::RED,
        7 => Color::ALICE_BLUE,
        _ => Color::DARK_GRAY,
    }
}
