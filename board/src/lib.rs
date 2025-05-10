#![cfg_attr(not(feature = "std"), no_std)]

mod base;
mod util;
mod variants;

#[cfg(test)]
mod test;

use core::ops::Not;

pub type MUInt = u32;

#[deprecated(
    since = "0.1.2",
    note = "Please use the MancalaBoard::<6>::default() instead"
)]
pub fn basic_board() -> MancalaBoard<6> {
    MancalaBoard {
        left: [4; 6],
        right: [4; 6],
        ..Default::default()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MoveResult {
    Done(Side, usize),
    Capture(Side, usize),
    ExtraTurn,
    IllegalMove,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Winner {
    Side(Side),
    Tie,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum BankCollector {
    Both,
    Side(Side),
    None,
}

impl BankCollector {
    pub const fn quantity(&self) -> usize {
        match self {
            BankCollector::Both => 2,
            BankCollector::Side(_) => 1,
            BankCollector::None => 0,
        }
    }
}

impl From<Side> for BankCollector {
    fn from(value: Side) -> Self {
        BankCollector::Side(value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MancalaBoard<const S: usize> {
    pub left: [MUInt; S],
    pub left_bank: MUInt,
    pub right: [MUInt; S],
    pub right_bank: MUInt,
}

impl<const S: usize> Default for MancalaBoard<S> {
    fn default() -> Self {
        Self {
            left: [4; S],
            right: [4; S],
            left_bank: 0,
            right_bank: 0,
        }
    }
}

impl<const S: usize> MancalaBoard<S> {
    pub const S: usize = S;

    #[inline]
    pub const fn new(initial: MUInt) -> MancalaBoard<S> {
        Self {
            left: [initial; S],
            right: [initial; S],
            left_bank: 0,
            right_bank: 0,
        }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        S
    }

    #[inline]
    pub const fn from_side(initial: [MUInt; S]) -> MancalaBoard<S> {
        Self {
            left: initial,
            right: initial,
            left_bank: 0,
            right_bank: 0,
        }
    }

    #[inline]
    pub const fn from_sides(left: [MUInt; S], right: [MUInt; S]) -> MancalaBoard<S> {
        Self {
            left,
            right,
            left_bank: 0,
            right_bank: 0,
        }
    }
}
