mod base;
mod kalah;
mod oware;
mod avalache;

#[macro_export]
macro_rules! full_game {
    // MAIN
    ($b:ident($s:literal) $(= $def_both:tt)? $(| $def_right:tt)?; $($a:tt $(=> $r:expr $(=> $e:expr)?)?,)*) => { {
        #[allow(unused_imports)]
        use crate::{
            full_game,
            MancalaBoard,
            MoveResult::{Capture,Done,ExtraTurn,IllegalMove},
            Side::{Left,Right},
            BankCollector::{None, Side, Both},
            Winner,
        };

        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut $b = MancalaBoard::<$s>::default();
        $( $b.left = $def_both; $b.right = $def_both; )?
        $( $b.right = $def_right; )?

        $(
            #[allow(unused_variables)]
            let result = full_game!(# $b $a);
            $(
                assert_eq!(result, $r $(, $e)?);
            )?
        )*
    } };
    // GAMEPLAY
    (# $b:ident (move k $index:literal $side:ident)) => { // kalah
        $b.move_piece_kalah($side, $index)
    };
    (# $b:ident (move o $index:literal $side:ident)) => { // oware
        $b.move_piece_oware($side, $index)
    };
    (# $b:ident (move a $index:literal $side:ident [$collector:expr])) => { // avalache
        $b.move_piece_avalache($side, $index, $collector)
    };
    (# $b:ident (capture $index:literal $side:ident)) => {
        $b.capture_kalah($side, $index)
    };
    (# $b:ident (oware collect $index:literal $side:ident [$collector:expr])) => {
        $b.oware_collect($side, $index, $collector)
    };
    // UTILS
    (# $b:ident (collect dishes)) => {
        $b.collect_dishes()
    };
    // CHECKS
    (# $b:ident (expect $board:tt)) => {{
        assert_eq!($b.right_bank, $board[0], "Right bank");
        assert_eq!($b.left, $board[1..=6], "Left dishes");
        assert_eq!($b.left_bank, $board[7], "Left bank");
        assert_eq!($b.right, $board[8..=13], "Right dishes");
    }};
    (# $b:ident (winner $winner:expr)) => {
        assert_eq!($b.winner(), $winner)
    };
    (# $b:ident (game_over $bool:literal)) => {
        assert_eq!($b.game_over(), $bool)
    };
    // PRINTS
    (# $b:ident (print expect)) => {
        println!("(expect {:?}),", $b);
    };
    (# $b:ident (print move $index:literal $side:ident)) => {
        println!("(move {} {} => {:?}),", $index, stringify!($side), $b.move_from_side($index, $side));
    };
    // CUSTOM CODE
    (# $b:ident $c:block) => {
        $c
    };
}
