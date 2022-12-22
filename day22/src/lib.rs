use std::io::{BufRead, BufReader};

use log::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    path: Vec<Instruction>,
    map: Vec<Vec<char>>,
    cube_size: i32,
    max_x: i32,
    max_y: i32,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        self.max_y = self.map.len() as i32;
        self.max_x = self.map.iter().map(|v| v.len()).max().unwrap() as i32;
        self.cube_size = (std::cmp::min(1 + self.max_x, 1 + self.max_y) / 3) as i32;

        debug!(
            "maxX:{}, maxY:{}, cube_size:{}",
            self.max_x, self.max_y, self.cube_size
        );
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let open_tile = '.';
        let mut x = 0;
        let mut y = 0;
        let mut facing = Facing::Right;
        // Find start position
        loop {
            assert!(x < self.map[0].len());
            if self.map[y][x] == open_tile {
                break;
            }
            x += 1;
        }

        debug!("P1 start_position: ({x}, {y})");
        for instruction in &self.path {
            debug!("P1 apply {facing:?} {instruction:?} @ ({x}, {y})");
            (facing, x, y) = self.apply_instruction_part1(instruction, facing, x, y);
        }

        let final_row = y + 1;
        let final_col = x + 1;
        let final_facing = match facing {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        };
        let answer = (1000 * final_row) + (4 * final_col) + final_facing;
        let answer = answer as ResultType;
        debug!("{answer} = {final_row} {final_col} {final_facing}");

        Ok(answer)
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        let open_tile = '.';
        let mut x = 0;
        let mut y = 0;
        let mut facing = Facing::Right;
        // Find start position
        loop {
            assert!(x < self.map[0].len());
            if self.map[y][x] == open_tile {
                break;
            }
            x += 1;
        }

        debug!("start_position: ({x}, {y})");
        debug!("face_size: {}", self.cube_size);
        for instruction in &self.path {
            debug!("apply {facing:?} {instruction:?} @ ({x}, {y})");

            if is_full {
                (facing, x, y) = self.apply_instruction_part2_full(instruction, facing, x, y);
            } else {
                (facing, x, y) = self.apply_instruction_part2_sample(instruction, facing, x, y);
            }
        }

        let final_row = y + 1;
        let final_col = x + 1;
        let final_facing = match facing {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        };
        let answer = (1000 * final_row) + (4 * final_col) + final_facing;
        let answer = answer as ResultType;
        debug!("{answer} = {final_row} {final_col} {final_facing}");

        Ok(answer)
    }
}

impl Solution {
    fn apply_instruction_part1(
        &self,
        instruction: &Instruction,
        facing: Facing,
        x: usize,
        y: usize,
    ) -> (Facing, usize, usize) {
        use Facing::*;

        let (wall_tile, invalid_tile) = ('#', ' ');

        let mut nx = x as i32;
        let mut ny = y as i32;
        let mut facing = facing;
        match instruction {
            Instruction::Forward(v) => {
                for _ in 0..*v {
                    let (dx, dy) = match facing {
                        Right => (1, 0),
                        Down => (0, 1),
                        Left => (-1, 0),
                        Up => (0, -1),
                    };
                    let ox = nx;
                    let oy = ny;

                    nx += dx;
                    ny += dy;

                    if nx < 0 {
                        nx = self.max_x - 1;
                    }
                    if nx >= self.max_x {
                        nx = 0;
                    }
                    if ny < 0 {
                        ny = self.max_y - 1;
                    }
                    if ny >= self.max_y {
                        ny = 0;
                    }

                    let mut y = ny as usize;
                    let mut x = nx as usize;

                    loop {
                        if self.map[y].get(x).unwrap_or(&invalid_tile) != &invalid_tile {
                            break;
                        }
                        nx += dx;
                        ny += dy;
                        if nx < 0 {
                            nx = self.max_x - 1;
                        }
                        if nx >= self.max_x {
                            nx = 0;
                        }
                        if ny < 0 {
                            ny = self.max_y - 1;
                        }
                        if ny >= self.max_y {
                            ny = 0;
                        }
                        x = nx as usize;
                        y = ny as usize;
                    }

                    // Don't move iff would hit a wall
                    if self.map[y][x] == wall_tile {
                        nx = ox;
                        ny = oy;
                        break;
                    }
                }
            }
            Instruction::Clockwise => {
                facing = facing.next(true);
            }
            Instruction::CounterClockwise => {
                facing = facing.next(false);
            }
            Instruction::None => unreachable!(),
        };

        (facing, nx.try_into().unwrap(), ny.try_into().unwrap())
    }

    fn apply_instruction_part2_sample(
        &self,
        instruction: &Instruction,
        facing: Facing,
        x: usize,
        y: usize,
    ) -> (Facing, usize, usize) {
        use Facing::*;

        let mut nx = x as i32;
        let mut ny = y as i32;
        let mut facing = facing;
        match instruction {
            Instruction::Forward(v) => {
                for _ in 0..*v {
                    let mut n_facing = facing;
                    let (dx, dy) = match facing {
                        Right => (1, 0),
                        Down => (0, 1),
                        Left => (-1, 0),
                        Up => (0, -1),
                    };
                    let ox = nx;
                    let oy = ny;

                    nx += dx;
                    ny += dy;

                    let fc_ox = self.face_coord(ox);
                    let fc_oy = self.face_coord(oy);
                    let fc_nx = self.face_coord(nx);
                    let fc_ny = self.face_coord(ny);

                    debug!("fc {fc_ox},{fc_oy} => {fc_nx},{fc_ny}");
                    if fc_ox != fc_nx || fc_oy != fc_ny {
                        match ((fc_ox, fc_oy), (fc_nx, fc_ny)) {
                            // face 1 => 4, no facing changes, etc
                            ((2, 0), (2, 1)) => {}
                            // face 4 => 6, facing becomes Down
                            ((2, 1), (3, 1)) => {
                                n_facing = Down;
                                nx = (fc_nx + 1) * self.cube_size - 1 - (ny % self.cube_size);
                                ny = 2 * self.cube_size;
                            }
                            // face 6 => 5, no facing changes, etc
                            ((3, 2), (2, 2)) => {}
                            // face 5 => 2, facing becomes Up
                            ((2, 2), (2, 3)) => {
                                n_facing = Up;
                                nx = self.cube_size - 1 - (nx % self.cube_size);
                                ny = (2 * self.cube_size) - 1;
                            }
                            // face 2 => 3, no facing changes, etc
                            ((0, 1), (1, 1)) => {}
                            // face 3 => 4, no facing changes, etc
                            ((1, 1), (2, 1)) => {}
                            // face 3 => 1, facing becomes Right
                            ((1, 1), (1, 0)) => {
                                n_facing = Right;
                                ny = nx % self.cube_size;
                                nx = self.cube_size * 2;
                            }
                            _ => unreachable!(
                                "face transition: ({fc_ox},{fc_oy}) -> ({fc_nx},{fc_ny})"
                            ),
                        }
                    }

                    debug!("{facing:?} => {n_facing:?} ({nx},{ny})");

                    let y = ny as usize;
                    let x = nx as usize;

                    // Don't move iff would hit a wall
                    if self.map[y][x] == '#' {
                        debug!("wall hit @ ({x},{y})");
                        nx = ox;
                        ny = oy;
                        break;
                    }
                    facing = n_facing;
                }
            }
            Instruction::Clockwise => {
                facing = facing.next(true);
            }
            Instruction::CounterClockwise => {
                facing = facing.next(false);
            }
            Instruction::None => unreachable!(),
        };

        (facing, nx.try_into().unwrap(), ny.try_into().unwrap())
    }

    fn apply_instruction_part2_full(
        &self,
        instruction: &Instruction,
        facing: Facing,
        x: usize,
        y: usize,
    ) -> (Facing, usize, usize) {
        use Facing::*;

        let mut nx = x as i32;
        let mut ny = y as i32;
        let mut facing = facing;
        match instruction {
            Instruction::Forward(v) => {
                for _ in 0..*v {
                    let mut n_facing = facing;
                    let (dx, dy) = match facing {
                        Right => (1, 0),
                        Down => (0, 1),
                        Left => (-1, 0),
                        Up => (0, -1),
                    };
                    let ox = nx;
                    let oy = ny;

                    nx += dx;
                    ny += dy;

                    debug!("pc {ox},{oy} => {nx},{ny}");

                    let fc_ox = self.face_coord(ox);
                    let fc_oy = self.face_coord(oy);
                    let fc_nx = self.face_coord(nx);
                    let fc_ny = self.face_coord(ny);

                    debug!("fc {fc_ox},{fc_oy} => {fc_nx},{fc_ny}");
                    if fc_ox != fc_nx || fc_oy != fc_ny {
                        match ((fc_ox, fc_oy), (fc_nx, fc_ny)) {
                            // face 1 => 2, no facing changes, etc
                            ((1, 0), (2, 0)) => {}
                            // face 1 => 3, no facing changes, etc
                            ((1, 0), (1, 1)) => {}
                            // face 1 => 5, facing becomes Right
                            ((1, 0), (0, 0)) => {
                                debug!("face 1 left to face 5");
                                n_facing = Right;
                                ny = 3 * self.cube_size - 1 - (ny % self.cube_size);
                                nx = 0
                            }
                            // face 1 => 6, facing becomes Right
                            ((1, 0), (1, 3)) => {
                                debug!("face 1 up to face 6");
                                n_facing = Right;
                                ny = 3 * self.cube_size + (nx % self.cube_size);
                                nx = 0;
                            }
                            // face 2 => 1, no facing changes, etc
                            ((2, 0), (1, 0)) => {}
                            // face 2 => 3, facing becomes Left
                            ((2, 0), (2, 1)) => {
                                debug!("face 2 down to face 3");
                                n_facing = Left;
                                ny = self.cube_size + (nx % self.cube_size);
                                nx = 2 * self.cube_size - 1;
                            }
                            // face 2 => 4, facing becomes Left
                            ((2, 0), (3, 0)) => {
                                debug!("face 2 right to face 4");
                                n_facing = Left;
                                ny = 3 * self.cube_size - 1 - (ny % self.cube_size);
                                nx = 2 * self.cube_size - 1;
                            }
                            // face 2 => 6, facing becomes Up
                            ((2, 0), (2, 3)) => {
                                debug!("face 2 up to face 6");
                                n_facing = Up;
                                nx %= self.cube_size;
                                ny = 4 * self.cube_size - 1
                            }
                            // face 3 => 1, no facing changes, etc
                            ((1, 1), (1, 0)) => {}
                            // face 3 => 2, facing becomes Up
                            ((1, 1), (2, 1)) => {
                                debug!("face 3 right to face 2");
                                n_facing = Up;
                                nx = 2 * self.cube_size + (ny % self.cube_size);
                                ny = self.cube_size - 1;
                            }
                            // face 3 => 4, no facing changes, etc
                            ((1, 1), (1, 2)) => {}
                            // face 3 => 5, facing becomes Down
                            ((1, 1), (0, 1)) => {
                                n_facing = Down;
                                nx = ny % self.cube_size;
                                ny = 2 * self.cube_size;
                            }
                            // face 4 => 2, facing becomes Left
                            ((1, 2), (2, 2)) => {
                                debug!("face 4 right to face 2");
                                n_facing = Left;
                                ny = self.cube_size - 1 - (ny % self.cube_size);
                                nx = 3 * self.cube_size - 1;
                            }
                            // face 4 => 3, no facing changes, etc
                            ((1, 2), (1, 1)) => {}
                            // face 4 => 5, no facing changes, etc
                            ((1, 2), (0, 2)) => {}
                            // face 4 => 6, facing becomes Left
                            ((1, 2), (1, 3)) => {
                                n_facing = Left;
                                ny = 3 * self.cube_size + (nx % self.cube_size);
                                nx = self.cube_size - 1;
                            }
                            // face 5 => 1, facing becomes Right
                            ((0, 2), (3, 2)) => {
                                debug!("face 5 left to face 1");
                                n_facing = Right;
                                ny = self.cube_size - 1 - (ny % self.cube_size);
                                nx = self.cube_size;
                            }
                            // face 5 => 3, facing becomes Right
                            ((0, 2), (0, 1)) => {
                                n_facing = Right;
                                ny = self.cube_size + (nx % self.cube_size);
                                nx = self.cube_size;
                            }
                            // face 5 => 4, no facing changes, etc
                            ((0, 2), (1, 2)) => {}
                            // face 5 => 6, no facing changes, etc
                            ((0, 2), (0, 3)) => {}

                            // face 6 => 1, facing becomes Down
                            ((0, 3), (3, 3)) => {
                                debug!("face 6 left to face 1");
                                n_facing = Down;
                                nx = self.cube_size + (ny % self.cube_size);
                                ny = 0;
                            }
                            // face 6 => 2, facing becomes Down
                            ((0, 3), (0, 0)) => {
                                debug!("face 6 down to face 2");
                                n_facing = Down;
                                nx = 2 * self.cube_size + (nx % self.cube_size);
                                ny = 0;
                            }
                            // face 6 => 4, facing becomes Up
                            ((0, 3), (1, 3)) => {
                                n_facing = Up;
                                nx = self.cube_size + (ny % self.cube_size);
                                ny = 3 * self.cube_size - 1;
                            }
                            // face 6 => 5, no facing changes, etc
                            ((0, 3), (0, 2)) => {}
                            _ => unreachable!(
                                "face transition: ({fc_ox},{fc_oy}) -> ({fc_nx},{fc_ny})"
                            ),
                        }
                    }

                    debug!("{facing:?} => {n_facing:?} ({nx},{ny})");

                    let y = ny as usize;
                    let x = nx as usize;

                    // Don't move iff would hit a wall
                    if self.map[y][x] == '#' {
                        debug!("wall hit @ ({x},{y})");
                        nx = ox;
                        ny = oy;
                        break;
                    }
                    facing = n_facing;
                }
            }
            Instruction::Clockwise => {
                facing = facing.next(true);
            }
            Instruction::CounterClockwise => {
                facing = facing.next(false);
            }
            Instruction::None => unreachable!(),
        };

        (facing, nx.try_into().unwrap(), ny.try_into().unwrap())
    }

    fn face_coord(&self, p: i32) -> i32 {
        let mut p = p;
        p += self.cube_size * 4;
        p %= self.cube_size * 4;
        p / self.cube_size
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();

        let mut stage = 0;
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                stage += 1;
                continue;
            }
            match stage {
                0 => solution.map.push(line.chars().collect()),
                1 => solution.path = Vec::<Instruction>::from_str(&line).unwrap(),
                _ => unreachable!(),
            }
        }
        Ok(solution)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Forward(u32),
    Clockwise,
    CounterClockwise,
    None,
}

trait MyFromStr {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    where
        Self: std::marker::Sized;
}

impl MyFromStr for Vec<Instruction> {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        let mut instructions = Vec::new();
        let mut cur_instruction = None;
        for c in s.chars() {
            match c {
                'R' => {
                    if cur_instruction != None {
                        instructions.push(cur_instruction);
                    }
                    instructions.push(Clockwise);
                    cur_instruction = None;
                }
                'L' => {
                    if cur_instruction != None {
                        instructions.push(cur_instruction);
                    }
                    instructions.push(CounterClockwise);
                    cur_instruction = None;
                }
                d if c.is_numeric() => {
                    let mut v = match cur_instruction {
                        None => 0,
                        Forward(v) => v,
                        instruction => {
                            instructions.push(instruction);
                            0
                        }
                    };
                    v *= 10;
                    v += d.to_digit(10).unwrap();
                    cur_instruction = Forward(v);
                }
                _ => unreachable!(),
            }
        }
        if cur_instruction != None {
            instructions.push(cur_instruction);
        }
        Ok(instructions)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn next(&self, clockwise: bool) -> Self {
        use Facing::*;

        match self {
            Right if clockwise => Down,
            Down if clockwise => Left,
            Left if clockwise => Up,
            Up if clockwise => Right,
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
        }
    }
}
