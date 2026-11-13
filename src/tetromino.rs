use std::ops::{Add, AddAssign};

use crate::{manager::ManagerError, setting::Settings};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TetrominoType {
    I, J, L, O, S, Z, T
}

impl Into<TetrominoType> for &str {
    fn into(self) -> TetrominoType {
        match self {
            "i" => TetrominoType::I,
            "j" => TetrominoType::J,
            "l" => TetrominoType::L,
            "o" => TetrominoType::O,
            "s" => TetrominoType::S,
            "z" => TetrominoType::Z,
            "t" => TetrominoType::T,
            _ => panic!("Cannot convert {} into TetrominoType", self)
        }
    }
}

impl TetrominoType {
    fn states(&self) -> [TetrominoState; 4] {
        // TODO
        let state = match *self {
            TetrominoType::I => [
                [[-1,  0], [ 0,  0], [ 1,  0], [ 2,  0]],
                [[ 1,  1], [ 1,  0], [ 1, -1], [ 1, -2]],
                [[-1, -1], [ 0, -1], [ 1, -1], [ 2, -1]],
                [[ 0,  1], [ 0,  0], [ 0, -1], [ 0, -2]],
            ],
            TetrominoType::J => [
                [[-1,  1], [-1,  0], [ 0,  0], [ 1,  0]],
                [[ 0,  1], [ 1,  1], [ 0,  0], [ 0, -1]],
                [[-1,  0], [ 0,  0], [ 1,  0], [ 1, -1]],
                [[ 0,  1], [ 0,  0], [ 0, -1], [-1, -1]],
            ],
            TetrominoType::L => [
                [[-1,  0], [ 0,  0], [ 1,  0], [ 1,  1]],
                [[ 0,  1], [ 0,  0], [ 0, -1], [ 1, -1]],
                [[-1, -1], [-1,  0], [ 0,  0], [ 1,  0]],
                [[-1,  1], [ 0,  1], [ 0,  0], [ 0, -1]],
            ],
            TetrominoType::O => [
                [[ 0,  1], [ 1,  1], [ 0,  0], [ 1,  0]],
                [[ 0,  1], [ 1,  1], [ 0,  0], [ 1,  0]],
                [[ 0,  1], [ 1,  1], [ 0,  0], [ 1,  0]],
                [[ 0,  1], [ 1,  1], [ 0,  0], [ 1,  0]],
            ],
            TetrominoType::S => [
                [[-1,  0], [ 0,  0], [ 0,  1], [ 1,  1]],
                [[ 0,  1], [ 0,  0], [ 1,  0], [ 1, -1]],
                [[-1, -1], [ 0, -1], [ 0,  0], [ 1,  0]],
                [[-1,  1], [-1,  0], [ 0,  0], [ 0, -1]],
            ],
            TetrominoType::Z => [
                [[-1,  1], [ 0,  1], [ 0,  0], [ 1,  0]],
                [[ 1,  1], [ 0,  0], [ 1,  0], [ 0, -1]],
                [[-1,  0], [ 0,  0], [ 0, -1], [ 1, -1]],
                [[ 0,  1], [-1,  0], [ 0,  0], [-1, -1]],
            ],
            TetrominoType::T => [
                [[-1,  0], [ 0,  0], [ 1,  0], [ 0,  1]],
                [[ 0,  1], [ 0,  0], [ 1,  0], [ 0, -1]],
                [[-1,  0], [ 0,  0], [ 1,  0], [ 0, -1]],
                [[ 0,  1], [-1,  0], [ 0,  0], [ 0, -1]],
            ],
        };
        state.iter()
            .map(|row| 
                TetrominoState(row.iter()
                .map(|pos| Position(pos[0], pos[1]))
                .collect())
            ).collect::<Vec<TetrominoState>>().try_into().unwrap()
    }

    fn wall_kicks(&self) -> Vec<Vec<Position>> {
        // TODO
        let wall_kicks = match *self {
            TetrominoType::O => [
                [[ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0]],
                [[ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0]],
                [[ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0]],
                [[ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0]],
            ],
            TetrominoType::I => [
                [[ 0,  0], [-1,  0], [ 2,  0], [-1,  0], [ 2,  0]],
                [[ 0,  0], [ 1,  0], [ 1,  0], [ 1,  1], [ 1, -2]],
                [[ 0,  0], [ 2,  0], [-1,  0], [ 2, -1], [-1, -1]],
                [[ 0,  0], [ 0,  0], [ 0,  0], [ 0, -2], [ 0,  1]],
            ],
            TetrominoType::J | TetrominoType::L | TetrominoType::S | 
            TetrominoType::T | TetrominoType::Z => [
                [[ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0]],
                [[ 0,  0], [ 1,  0], [ 1, -1], [ 0,  2], [ 1,  2]],
                [[ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0], [ 0,  0]],
                [[ 0,  0], [-1,  0], [-1, -1], [ 0,  2], [-1,  2]],
            ],
        };

        wall_kicks.iter()
            .map(|row| row.iter().map(|pos| Position(pos[0], pos[1])).collect())
            .collect()
    }

    pub fn state(&self, rotation: Rotation) -> TetrominoState {
        self.states().get(rotation as usize).unwrap().clone()
    }

    pub fn wall_kick(&self, rotation: Rotation) -> Vec<Position> {
        self.wall_kicks().get(rotation as usize).unwrap().clone()
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

    pub fn state(&self) -> TetrominoState {
        self.ttype.state(self.rotation)
    }

    /// Try to rotate, but don't change the actual states
    pub fn peek_rotate(&self, action: TetrisAction) -> Result<TetrominoState, ManagerError> {
        match action {
            TetrisAction::C => Ok(self.ttype.state(self.rotation.clockwise())),
            TetrisAction::W => Ok(self.ttype.state(self.rotation.counter_clockwise())),
            _ => Err(ManagerError::ActionError)
        }
    }

    pub fn rotate(&mut self, action: TetrisAction) -> Result<(), ManagerError> {
        match action {
            TetrisAction::C => self.rotation = self.rotation.clockwise(),
            TetrisAction::W => self.rotation = self.rotation.counter_clockwise(),
            TetrisAction::L | TetrisAction::R | TetrisAction::D | 
            TetrisAction::H | TetrisAction::E | TetrisAction::Q => return Err(ManagerError::ActionError)
        };
        Ok(())
    }
}