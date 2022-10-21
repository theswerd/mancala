use crate::{
    MancalaBoard,
    Side,
    MoveResult,
    Winner,
};

#[test]
fn sample_game() {
    let mut board = MancalaBoard::default();

    assert_eq!(board.move_from_side(1, Side::Left), MoveResult::Done);
    assert_eq!(board.values, [0, 0, 5, 5, 5, 5, 4, 0, 4, 4, 4, 4, 4, 4]);

    assert_eq!(board.move_from_side(4, Side::Right), MoveResult::Capture(1));
    assert_eq!(board.values, [1, 1, 5, 5, 5, 5, 4, 0, 4, 4, 4, 0, 5, 5]);
    board.capture(1, Side::Right);
    assert_eq!(board.values, [7, 0, 5, 5, 5, 5, 4, 0, 4, 4, 4, 0, 5, 0]);

    assert_eq!(board.move_from_side(2, Side::Left), MoveResult::ExtraTurn);
    assert_eq!(board.values, [7, 0, 0, 6, 6, 6, 5, 1, 4, 4, 4, 0, 5, 0]);
    assert_eq!(board.move_from_side(6, Side::Left), MoveResult::Capture(11));
    assert_eq!(board.values, [7, 0, 0, 6, 6, 6, 0, 2, 5, 5, 5, 1, 5, 0]);
    board.capture(11, Side::Left);
    assert_eq!(board.values, [7, 0, 0, 0, 6, 6, 0, 9, 5, 5, 5, 0, 5, 0]);

    // testing minor things
    assert_eq!(board.game_over(), false);
    assert_eq!(board.winner(), Winner::Side(Side::Left)); // 7 < 9
    assert_eq!(board.move_from_side(0, Side::Right), MoveResult::IllegalMove);
    assert_eq!(board.move_from_side(4, Side::Right), MoveResult::IllegalMove);

    assert_eq!(board.move_from_side(2, Side::Right), MoveResult::ExtraTurn);
    assert_eq!(board.values, [8, 0, 0, 0, 6, 6, 0, 9, 5, 0, 6, 1, 6, 1]);
    assert_eq!(board.move_from_side(6, Side::Right), MoveResult::ExtraTurn);
    assert_eq!(board.values, [9, 0, 0, 0, 6, 6, 0, 9, 5, 0, 6, 1, 6, 0]);
    assert_eq!(board.move_from_side(1, Side::Right), MoveResult::Capture(13));
    assert_eq!(board.values, [9, 0, 0, 0, 6, 6, 0, 9, 0, 1, 7, 2, 7, 1]);
    board.capture(13, Side::Right);
    assert_eq!(board.values, [10, 0, 0, 0, 6, 6, 0, 9, 0, 1, 7, 2, 7, 0]);

    assert_eq!(board.move_from_side(4, Side::Left), MoveResult::Done);
    assert_eq!(board.values, [10, 0, 0, 0, 0, 7, 1, 10, 1, 2, 8, 2, 7, 0]);

    assert_eq!(board.move_from_side(4, Side::Right), MoveResult::Capture(13));
    assert_eq!(board.values, [10, 0, 0, 0, 0, 7, 1, 10, 1, 2, 8, 0, 8, 1]);
    board.capture(13, Side::Right);
    assert_eq!(board.values, [11, 0, 0, 0, 0, 7, 1, 10, 1, 2, 8, 0, 8, 0]);

    assert_eq!(board.move_from_side(6, Side::Left), MoveResult::ExtraTurn);
    assert_eq!(board.values, [11, 0, 0, 0, 0, 7, 0, 11, 1, 2, 8, 0, 8, 0]);
    assert_eq!(board.move_from_side(5, Side::Left), MoveResult::Done);
    assert_eq!(board.values, [11, 0, 0, 0, 0, 0, 1, 12, 2, 3, 9, 1, 9, 0]);

    assert_eq!(board.move_from_side(1, Side::Right), MoveResult::Done);
    assert_eq!(board.values, [11, 0, 0, 0, 0, 0, 1, 12, 0, 4, 10, 1, 9, 0]);

    assert_eq!(board.move_from_side(6, Side::Left), MoveResult::ExtraTurn);
    assert_eq!(board.values, [11, 0, 0, 0, 0, 0, 0, 13, 0, 4, 10, 1, 9, 0]);

    assert_eq!(board.game_over(), true);
    board.collect_dishes();
    assert_eq!(board.values, [35, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0]);
    assert_eq!(board.winner(), Winner::Side(Side::Right)); // 35 > 13
}
