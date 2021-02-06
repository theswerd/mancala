fn main() {
    let mut mancala_board: [i32; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    
    print_board(mancala_board);
    mancala_board[4] = 12;
    print_board(mancala_board);
}

fn print_board(amounts: [i32; 12]) {
    println!(
        " _____________\n(     {:2}      )\n ‾‾‾‾‾‾‾‾‾‾‾‾‾",
        amounts[0]
    );
    for index in 1..5 {
        println!(" ____  |  ____");
        println!("( {:2} ) | ( {:2} )", amounts[index], amounts[index + 6]);
        println!(" ‾‾‾‾  |  ‾‾‾‾");
    }
    println!(
        " _____________\n(     {:2}      )\n ‾‾‾‾‾‾‾‾‾‾‾‾‾",
        amounts[6]
    );
}

/*
 ___________
(    0      )
 ‾‾‾‾‾‾‾‾‾‾‾
 ___  |  ___
( 1 ) | ( 7 )
 ‾‾‾  |  ‾‾‾
 ___  |  ___
( 2 ) | ( 8 )
 ‾‾‾  |  ‾‾‾
 ___  |  ___
( 3 ) | ( 9 )
 ‾‾   |  ‾‾
 ___  |  ____
( 4 ) | ( 10 )
 ‾‾‾  |  ‾‾‾‾
 ___  |  ____
( 5 ) | ( 11 )
 ‾‾‾  |  ‾‾‾‾
 ___________
(     6     )
 ‾‾‾‾‾‾‾‾‾‾‾
*/
