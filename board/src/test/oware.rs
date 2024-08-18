use crate::full_game;

// followed https://meatfighter.com/oware/

#[test]
fn sample_oware_game() {
    full_game!(
        board(6);

        (expect [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4]),

        (move o 5 Left => Done(Right, 3)),
        (expect [0, 4, 4, 4, 4, 4, 0, 0, 5, 5, 5, 5, 4, 4]),

        // (move o 5 Right => Done(Left, 4)),
        // (expect [0, 4, 4, 4, 4, 4, 0, 0, 5, 5, 5, 5, 4, 4]),
    );
}

#[test]
fn oware_collect_test() {
    full_game!(
        board(6) = [3*12,0,0,0,0,0] | [0,0,0,0,0,0];

        (expect [0, 3*12,0,0,0,0,0, 0, 0,0,0,0,0,0]),

        (move o 0 Left => Done(Left, 5)),
        (expect [0, 3,3,3,3,3,3, 0, 3,3,3,3,3,3]),

        (oware collect 0 Right [Left] => 48),
        (expect [0, 0,0,0,0,0,0, 48, 0,0,0,0,0,0]),
        // (move o 5 Right => Done(Left, 4)),
        // (expect [0, 4, 4, 4, 4, 4, 0, 0, 5, 5, 5, 5, 4, 4]),
    );
}
