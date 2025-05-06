use crate::{BankCollector, MUInt, MancalaBoard, MoveResult, Side};

#[derive(Clone, Copy)]
pub struct Variant {
    bank_collect: SideAction,
    capture: SideAction,
    clockwise_collect: ClockwiseCollect,
    avalache: bool,
}

#[derive(Clone, Copy)]
pub enum SideAction {
    Always,
    PlayerSide,
    OpponentSide,
    Never,
}

#[derive(Clone, Copy)]
pub enum ClockwiseCollect {
    Always(&'static dyn Fn() -> ()),
    PlayerSide(&'static dyn Fn() -> ()),
    OpponentSide(&'static dyn Fn() -> ()),
    Never,
}

impl Variant {
    pub const KALAH: Variant = Variant {
        bank_collect: SideAction::PlayerSide,
        capture: SideAction::PlayerSide,
        clockwise_collect: ClockwiseCollect::Never,
        avalache: false,
    };
    // apparently this is a thing
    // https://www.pandaclub.ch/it/gioco-mancala/
    pub const KALAH_WWF: Variant = Variant {
        bank_collect: SideAction::PlayerSide,
        capture: SideAction::Always,
        clockwise_collect: ClockwiseCollect::Never,
        avalache: false,
    };
    pub const OWARE: Variant = Variant {
        bank_collect: SideAction::Never,
        capture: SideAction::Never,
        clockwise_collect: ClockwiseCollect::OpponentSide(&(|| {})),
        avalache: false,
    };
}

impl<const S: usize> MancalaBoard<S> {
    pub fn move_from_variant(&mut self, side: Side, index: usize, collector_side: Side, variant: Variant) -> MoveResult {
        loop {
            let move_result = self.move_from_side(
                side,
                index,
                match variant.bank_collect {
                    SideAction::Always => BankCollector::Both,
                    SideAction::Never => BankCollector::None,
                    SideAction::PlayerSide|SideAction::OpponentSide => BankCollector::Side(collector_side),
                }
            );
            if variant.avalache {
                
            } else {
                break move_result
            }
        }
    }

    /// Moves the selected dish into the hand and moves them in an anti-clockwise direction, while using the side's bank.
    pub fn move_piece_kalah(&mut self, side: Side, index: usize) -> MoveResult {
        self.move_from_side(side, index, BankCollector::Side(side))
    }

    /// Moves the selected dish into the hand and moves them in an anti-clockwise direction, without using banks.
    pub fn move_piece_oware(&mut self, side: Side, index: usize) -> MoveResult {
        self.move_from_side(side, index, BankCollector::None)
    }

    /// Moves the selected dish into the hand and moves them in an anti-clockwise direction, if it ends on a non-empty dish it will repeat.
    pub fn move_piece_avalache(&mut self, mut side: Side, mut index: usize, bank_collector: impl Into<BankCollector>) -> MoveResult {
        let bank_collector = bank_collector.into();
        loop {
            let current_move = self.move_from_side(side, index, bank_collector);
            match current_move {
                MoveResult::IllegalMove | MoveResult::ExtraTurn => break current_move,
                MoveResult::Done(s, i) | MoveResult::Capture(s, i) => {
                    if self.side_to_dishes(s)[i] <= 1 {
                        break current_move
                    }
                    side = s;
                    index = i;
                },
            }
        }
    }

    /// Captures the selected and the opposing dish, and places them in the selected side
    pub fn capture_kalah(&mut self, side: Side, index: usize) {
        self.capture_from_side(side, index, side);
    }

    /// Collects all dishes that contain 2 or 3 pieces consecutively in a clockwise direction
    /// Returns the total amount of pieces collected
    pub fn oware_collect(&mut self, mut side: Side, mut index: usize, collector_side: Side) -> MUInt {
        let mut output = 0;
        loop {
            let dishes = self.side_to_dishes(side);
            if (2..=3).contains(&dishes[index]) {
                output += dishes[index];
                *self.side_bank(collector_side) += dishes[index];
                self.side_to_dishes_mut(side)[index] = 0;
                if index == 0 {
                    side = !side;
                    index = S - 1;
                } else {
                    index -= 1;
                }
            } else {
                break
            }
        }
        output
    }
}
