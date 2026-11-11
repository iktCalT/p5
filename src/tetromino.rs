#[derive(Debug, Clone, Copy)]
pub enum TetrominoType {
    I, J, L, O, S, Z, T
}

pub enum TetrisAction {
    C, W, L, R, D, H, E
}

pub enum Rotation {
    Origin, Right, Inverse, Left
}

pub enum WallKick {
    Normal, I, O
}

pub struct Position(pub isize, pub isize);
pub struct Tetromino {
    ttype: TetrominoType,
    states: Vec<Vec<Position>>,
    wall_kick: WallKick,
    rotation: Rotation,
}


impl Tetromino {
    pub fn new(ttype: TetrominoType) -> Self {
        let to_states = |s: [[[isize;2];4];4]| {
            s.iter().map(|a| 
                a.iter().map(|b| Position(b[0], b[1])).collect()).collect::<Vec<Vec<Position>>>()
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


    fn rotate(&self, action: TetrisAction) {
        
    }
}