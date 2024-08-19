use crate::full_game;

#[test]
fn sample_kalah_game() {
    full_game!(
        board(6);
        // imagine if I had a bug and needed to rewrite all of this 🤣
        // luckily it only happened 6 times before 🤣🤣🤣

        (expect [0, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4]),

        (move k 0 Left) => Done(Left, 4),
        (expect [0, 0, 5, 5, 5, 5, 4, 0, 4, 4, 4, 4, 4, 4]),

        (move k 3 Right) => Done(Left, 0),
        (expect [1, 1, 5, 5, 5, 5, 4, 0, 4, 4, 4, 0, 5, 5]),

        (move k 1 Left) => ExtraTurn,
        (expect [1, 1, 0, 6, 6, 6, 5, 1, 4, 4, 4, 0, 5, 5]),
        (move k 0 Left) => Capture(Left, 1),
        (expect [1, 0, 1, 6, 6, 6, 5, 1, 4, 4, 4, 0, 5, 5]),
        (capture 1 Left),
        (expect [1, 0, 0, 6, 6, 6, 5, 7, 4, 4, 4, 0, 0, 5]),

        (move k 2 Right) => ExtraTurn,
        (expect [2, 0, 0, 6, 6, 6, 5, 7, 4, 4, 0, 1, 1, 6]),
        (move k 3 Right) => Done(Right, 4),
        (expect [2, 0, 0, 6, 6, 6, 5, 7, 4, 4, 0, 0, 2, 6]),


        // testing minor stuff
        (game_over false),
        (winner Winner::Side(Left)), // 7 > 2
        (move k 0 Left) => IllegalMove,
        (move k 2 Right) => IllegalMove,


        (move k 2 Left) => Done(Right, 1),
        (expect [2, 0, 0, 0, 7, 7, 6, 8, 5, 5, 0, 0, 2, 6]),
        
        (move k 4 Right) => ExtraTurn,
        (expect [3, 0, 0, 0, 7, 7, 6, 8, 5, 5, 0, 0, 0, 7]),
        (move k 1 Right) => ExtraTurn,
        (expect [4, 0, 0, 0, 7, 7, 6, 8, 5, 0, 1, 1, 1, 8]),
        (move k 0 Right) => Done(Right, 5),
        (expect [4, 0, 0, 0, 7, 7, 6, 8, 0, 1, 2, 2, 2, 9]),

        (move k 3 Left) => Done(Right, 3),
        (expect [4, 0, 0, 0, 0, 8, 7, 9, 1, 2, 3, 3, 2, 9]),

        (move k 4 Right) => ExtraTurn,
        (expect [5, 0, 0, 0, 0, 8, 7, 9, 1, 2, 3, 3, 0, 10]),
        (move k 3 Right) => ExtraTurn,
        (expect [6, 0, 0, 0, 0, 8, 7, 9, 1, 2, 3, 0, 1, 11]),
        (move k 2 Right) => Done(Right, 5),
        (expect [6, 0, 0, 0, 0, 8, 7, 9, 1, 2, 0, 1, 2, 12]),

        (move k 5 Left) => Done(Right, 5),
        (expect [6, 0, 0, 0, 0, 8, 0, 10, 2, 3, 1, 2, 3, 13]),

        (move k 0 Right) => Done(Right, 2),
        (expect [6, 0, 0, 0, 0, 8, 0, 10, 0, 4, 2, 2, 3, 13]),
        
        (move k 4 Left) => Done(Right, 5),
        (expect [6, 0, 0, 0, 0, 0, 1, 11, 1, 5, 3, 3, 4, 14]),
        
        (move k 3 Right) => ExtraTurn,
        (expect [7, 0, 0, 0, 0, 0, 1, 11, 1, 5, 3, 0, 5, 15]),
        (move k 0 Right) => Done(Right, 1),
        (expect [7, 0, 0, 0, 0, 0, 1, 11, 0, 6, 3, 0, 5, 15]),
        
        (move k 5 Left) => ExtraTurn,
        (expect [7, 0, 0, 0, 0, 0, 0, 12, 0, 6, 3, 0, 5, 15]),
        
        (game_over true),
        (collect dishes), // optional, since the winner function checks for all the dishes
        (expect [36, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0]),
        (winner Winner::Side(Right)),
    );
}
