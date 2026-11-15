use std::ops::{Add, AddAssign};

use crate::{manager::ManagerError, setting::SETTINGS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TetrominoType {
    I, J, L, O, S, T, Z
}

impl Into<TetrominoType> for &str {
    fn into(self) -> TetrominoType {
        match self {
            "i" => TetrominoType::I,
            "j" => TetrominoType::J,
            "l" => TetrominoType::L,
            "o" => TetrominoType::O,
            "s" => TetrominoType::S,
            "t" => TetrominoType::T,
            "z" => TetrominoType::Z,
            _ => panic!("Cannot convert {} into TetrominoType", self)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TetrisAction {
    /// Rotate 90 degrees clockwise
    C, 
    /// Rotate 90 degrees counter-clockwise
    W, 
    /// Move piece left
    L, 
    /// Move piece right
    R, 
    /// Move piece down
    D, 
    /// Hard drop piece
    H, 
    /// No action
    E, 
    /// Quit game
    Q,
    /// Show board (for debugging)
    S
}

impl Into<TetrisAction> for u8 {
    fn into(self) -> TetrisAction {
        match self.to_ascii_lowercase() {
            b'c' => TetrisAction::C,
            b'w' => TetrisAction::W,
            b'l' => TetrisAction::L,
            b'r' => TetrisAction::R,
            b'd' => TetrisAction::D,
            b'h' => TetrisAction::H,
            b'e' => TetrisAction::E,
            b'q' => TetrisAction::Q,
            _ => panic!("Invalid input command")
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Origin=0, Right, Inverse, Left
}

impl Rotation {
    pub fn clockwise(&self) -> Rotation {
        match self {
            Rotation::Origin => Rotation::Right,
            Rotation::Right => Rotation::Inverse,
            Rotation::Inverse => Rotation::Left,
            Rotation::Left => Rotation::Origin
        }
    }

    pub fn counter_clockwise(&self) -> Rotation {
        match self {
            Rotation::Origin => Rotation::Left,
            Rotation::Right => Rotation::Origin,
            Rotation::Inverse => Rotation::Right,
            Rotation::Left => Rotation::Inverse
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WallKick {
    Normal, I, O
}

impl Into<WallKick> for &str {
    fn into(self) -> WallKick {
        match self {
            "Normal" => WallKick::Normal,
            "I" => WallKick::I,
            "O" => WallKick::O,
            _ => panic!("Cannot convert {} into WallKick", self)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position(pub isize, pub isize);

impl Add<Self> for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Position(
            self.0 + rhs.0,
            self.1 + rhs.1
        )
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TetrominoState(pub Vec<Position>);

pub struct Tetromino {
    pub ttype: TetrominoType,
    pub rotation: Rotation,
}

impl Tetromino {
    pub fn new(ttype: TetrominoType) -> Self {
        Self {
            ttype,
            rotation: Rotation::Origin
        }
    }

    pub fn state(&self) -> &TetrominoState {
        SETTINGS.state(&self.ttype, &self.rotation)
    }

    /// Try to rotate, but don't change the actual states
    pub fn peek_rotate(&self, action: TetrisAction) -> Result<&TetrominoState, ManagerError> {
        match action {
            TetrisAction::C => Ok(SETTINGS.state(&self.ttype, &self.rotation.clockwise())),
            TetrisAction::W => Ok(SETTINGS.state(&self.ttype, &self.rotation.counter_clockwise())),
            _ => Err(ManagerError::ActionError)
        }
    }

    pub fn rotate(&mut self, action: TetrisAction) -> Result<(), ManagerError> {
        match action {
            TetrisAction::C => self.rotation = self.rotation.clockwise(),
            TetrisAction::W => self.rotation = self.rotation.counter_clockwise(),
            TetrisAction::L | TetrisAction::R | TetrisAction::D | 
            TetrisAction::H | TetrisAction::E | TetrisAction::Q | 
            TetrisAction::S => return Err(ManagerError::ActionError)
        };
        Ok(())
    }
}