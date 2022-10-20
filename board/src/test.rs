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
    assert_eq!(board.values, [6, 0, 5, 5, 5, 5, 4, 0, 0, 4, 4, 0, 5, 5]);

    assert_eq!(board.move_from_side(2, Side::Left), MoveResult::ExtraTurn);
    assert_eq!(board.values, [6, 0, 0, 6, 6, 6, 5, 1, 0, 4, 4, 0, 5, 5]);
    assert_eq!(board.move_from_side(6, Side::Left), MoveResult::Capture(11));
    assert_eq!(board.values, [6, 0, 0, 6, 6, 6, 0, 2, 1, 5, 5, 1, 5, 5]);
    board.capture(11, Side::Left);
    assert_eq!(board.values, [6, 0, 0, 6, 0, 6, 0, 9, 1, 5, 5, 0, 5, 5]);

    // testing minor things
    assert_eq!(board.game_over(), false);
    assert_eq!(board.winner(), Winner::Side(Side::Left)); // 6 < 9
    assert_eq!(board.move_from_side(0, Side::Right), MoveResult::IllegalMove);
    assert_eq!(board.move_from_side(4, Side::Right), MoveResult::IllegalMove);

    assert_eq!(board.move_from_side(1, Side::Right), MoveResult::Done);
    assert_eq!(board.values, [6, 0, 0, 6, 0, 6, 0, 9, 0, 6, 5, 0, 5, 5]);

    assert_eq!(board.move_from_side(3, Side::Left), MoveResult::Done);
    assert_eq!(board.values, [6, 0, 0, 0, 1, 7, 1, 10, 1, 7, 5, 0, 5, 5]);

    assert_eq!(board.move_from_side(2, Side::Right), MoveResult::Capture(2));
    assert_eq!(board.values, [7, 1, 1, 0, 1, 7, 1, 10, 1, 0, 6, 1, 6, 6]);
    board.capture(2, Side::Right);
    assert_eq!(board.values, [8, 1, 0, 0, 1, 7, 1, 10, 1, 0, 6, 1, 6, 6]);

    assert_eq!(board.move_from_side(6, Side::Left), MoveResult::ExtraTurn);
    assert_eq!(board.values, [8, 1, 0, 0, 1, 7, 0, 11, 1, 0, 6, 1, 6, 6]);
    assert_eq!(board.move_from_side(5, Side::Left), MoveResult::Done);
    assert_eq!(board.values, [8, 1, 0, 0, 1, 0, 1, 12, 2, 1, 7, 2, 7, 6]);

    assert_eq!(board.move_from_side(2, Side::Right), MoveResult::Done);
    assert_eq!(board.values, [8, 1, 0, 0, 1, 0, 1, 12, 2, 0, 8, 2, 7, 6]);

    assert_eq!(board.move_from_side(4, Side::Left), MoveResult::Capture(5));
    assert_eq!(board.values, [8, 1, 0, 0, 0, 1, 1, 12, 2, 0, 8, 2, 7, 6]);
    board.capture(5, Side::Left);
    assert_eq!(board.values, [8, 1, 0, 0, 0, 0, 1, 20, 2, 0, 8, 2, 0, 6]);

    assert_eq!(board.move_from_side(1, Side::Right), MoveResult::Done);
    assert_eq!(board.values, [8, 1, 0, 0, 0, 0, 1, 20, 0, 1, 9, 2, 0, 6]);

    assert_eq!(board.move_from_side(6, Side::Left), MoveResult::ExtraTurn);
    assert_eq!(board.values, [8, 1, 0, 0, 0, 0, 0, 21, 0, 1, 9, 2, 0, 6]);
    assert_eq!(board.move_from_side(1, Side::Left), MoveResult::Capture(2));
    assert_eq!(board.values, [8, 0, 1, 0, 0, 0, 0, 21, 0, 1, 9, 2, 0, 6]);
    board.capture(2, Side::Left);
    assert_eq!(board.values, [8, 0, 0, 0, 0, 0, 0, 23, 0, 0, 9, 2, 0, 6]);

    assert_eq!(board.game_over(), true);
    board.collect_dishes();
    assert_eq!(board.values, [25, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0]);
    assert_eq!(board.winner(), Winner::Side(Side::Right)); // 25 > 23
}
