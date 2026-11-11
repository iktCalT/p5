use crate::manager::ManagerError;

#[derive(Debug, Clone, Copy)]
pub enum TetrominoType {
    I, J, L, O, S, Z, T
}

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

pub enum WallKick {
    Normal, I, O
}

#[derive(Debug, Clone)]
pub struct Position(pub isize, pub isize);

#[derive(Debug, Clone)]
pub struct TetrominoState(Vec<Position>);

pub struct Tetromino {
    ttype: TetrominoType,
    states: Vec<TetrominoState>,
    wall_kick: WallKick,
    rotation: Rotation,
}

impl Tetromino {
    pub fn new(ttype: TetrominoType) -> Self {
        let to_states = |states: [[[isize; 2]; 4]; 4]| {
            states.iter()
                .map(|state| TetrominoState(state.iter()
                    .map(|position| Position(position[0], position[1]))
                    .collect()))
                .collect::<Vec<TetrominoState>>()
        };

        let (states, wall_kick) = match ttype {
            TetrominoType::I => (to_states([
                [[-1,  0], [ 0,  0], [ 1,  0], [ 2,  0]],
                [[ 1,  1], [ 1,  0], [ 1, -1], [ 1, -2]],
                [[-1, -1], [ 0, -1], [ 1, -1], [ 2, -1]],
                [[ 0,  1], [ 0,  0], [ 0, -1], [ 0, -2]],
            ]), WallKick::I),
            TetrominoType::J => (to_states([
                [[-1,  1], [-1,  0], [ 0,  0], [ 1,  0]],
                [[ 0,  1], [ 1,  1], [ 0,  0], [ 0, -1]],
                [[-1,  0], [ 0,  0], [ 1,  0], [ 1, -1]],
                [[ 0,  1], [ 0,  0], [ 0, -1], [-1, -1]],
            ]), WallKick::Normal),
            TetrominoType::L => (to_states([
                [[-1,  0], [ 0,  0], [ 1,  0], [ 1,  1]],
                [[ 0,  1], [ 0,  0], [ 0, -1], [ 1, -1]],
                [[-1, -1], [-1,  0], [ 0,  0], [ 1,  0]],
                [[-1,  1], [ 0,  1], [ 0,  0], [ 0, -1]],
            ]), WallKick::Normal),
            TetrominoType::O => (to_states([
                [[ 0,  1], [ 1,  1], [ 0,  0], [ 1,  0]],
                [[ 0,  1], [ 1,  1], [ 0,  0], [ 1,  0]],
                [[ 0,  1], [ 1,  1], [ 0,  0], [ 1,  0]],
                [[ 0,  1], [ 1,  1], [ 0,  0], [ 1,  0]],
            ]), WallKick::O),
            TetrominoType::S => (to_states([
                [[-1,  0], [ 0,  0], [ 0,  1], [ 1,  1]],
                [[ 0,  1], [ 0,  0], [ 1,  0], [ 1, -1]],
                [[-1, -1], [ 0, -1], [ 0,  0], [ 1,  0]],
                [[-1,  1], [-1,  0], [ 0,  0], [ 0, -1]],
            ]), WallKick::Normal),
            TetrominoType::Z => (to_states([
                [[-1,  1], [ 0,  1], [ 0,  0], [ 1,  0]],
                [[ 1,  1], [ 0,  0], [ 1,  0], [ 0, -1]],
                [[-1,  0], [ 0,  0], [ 0, -1], [ 1, -1]],
                [[ 0,  1], [-1,  0], [ 0,  0], [-1, -1]],
            ]), WallKick::Normal),
            TetrominoType::T => (to_states([
                [[-1,  0], [ 0,  0], [ 1,  0], [ 0,  1]],
                [[ 0,  1], [ 0,  0], [ 1,  0], [ 0, -1]],
                [[-1,  0], [ 0,  0], [ 1,  0], [ 0, -1]],
                [[ 0,  1], [-1,  0], [ 0,  0], [ 0, -1]],
            ]), WallKick::Normal),
        };

        Self { 
            ttype, 
            states,
            wall_kick,
            rotation: Rotation::Origin
        }
    }

    /// Try to do a movement, but don't change the real state of tetromino
    pub fn peek_state(&self, action: TetrisAction) -> Result<TetrominoState, ManagerError> {
        match action {
            TetrisAction::C => Ok(self.states[self.rotation.clockwise() as usize].clone()),
            TetrisAction::W => Ok(self.states[self.rotation.counter_clockwise() as usize].clone()),
            _ => todo!()
        }
    }

    pub fn rotate(&mut self, action: TetrisAction) -> Result<(), ManagerError> {
        match action {
            TetrisAction::C => self.rotation = self.rotation.clockwise(),
            TetrisAction::W => self.rotation = self.rotation.counter_clockwise(),
            TetrisAction::L | TetrisAction::R | TetrisAction::D | 
            TetrisAction::H | TetrisAction::E => return Err(ManagerError::ActionError)
        };
        Ok(())
    }
}