use crate::full_game;

#[test]
fn bulk_calculation() {
    for i in 0..1000 {
        full_game!(
            board(6) = [i,0,0,0,0,0] | [0;6];
            (move k 0 Left),
            (expect [
                0,
                i/13,
                (i+12)/13,
                (i+11)/13,
                (i+10)/13,
                (i+9)/13,
                (i+8)/13,
                (i+7)/13,
                (i+6)/13,
                (i+5)/13,
                (i+4)/13,
                (i+3)/13,
                (i+2)/13,
                (i+1)/13,
            ]),
        );
    }
}

#[test]
fn capture() {
    // this seems trivial, but I would have caught the bug in no time :/
    // no idea how to write a good generated test for this
    full_game!(
        board(6) = [0,4,0,0,0,0] | [0,0,0,0,0,0];
        (move k 1 Left) => Done(Left, 5),
    );
    full_game!(
        board(6) = [0,4,0,0,0,0] | [1,0,0,0,0,0];
        (move k 1 Left) => Capture(Left, 5),
    );
}
