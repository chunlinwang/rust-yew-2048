use rand::{Rng};
use rand::rngs::OsRng;

#[derive(Clone, Debug)]
pub struct Matrix {
    pub direction: u64,
    pub data: [[usize; 4]; 4],
    pub occupied_field: [bool; 16],
    pub score: u64,
}

impl Matrix {
    pub fn new() -> Matrix {
        let mut rng = OsRng::default();
        let mut data = [[0; 4]; 4];
        let mut occupied_field = [false; 16];
        let mut init_value_count: u64 = 0;

        while init_value_count < 2 {
            let index_num = rng.gen_range(0..16);

            let x = (index_num / 4) as usize;
            let y = (index_num % 4) as usize;

            if data[x][y] == 0 {
                data[x][y] = 2;
                occupied_field[index_num] = true;
                init_value_count += 1;
            }
        }

        Matrix {
            direction: rng.gen_range(0..4),
            data,
            occupied_field,
            score: 0,
        }
    }

    pub fn move_to(&mut self, direction: Direction) -> bool {
        for y in 0..4 {
            match direction {
                Direction::UP => {
                    self.move_up_line(y);
                }
                Direction::DOWN => {
                    self.move_down_line(y);
                }
                Direction::LEFT => {
                    self.move_left_line(y);
                }
                Direction::RIGHT => {
                    self.move_right_line(y);
                }
                _ => panic!("error"),
            }
        }

        self.get_new_field_index()
    }

    fn get_new_field_index(&mut self) -> bool {
        let mut rng = OsRng::default();
        let mut positions = Vec::new();
        for index in 0..16 {
            if self.occupied_field[index] == false {
                positions.push(index);
            }
        }
        let reset_count = positions.iter().count();

        if reset_count > 0 {
            let index_num: usize = rng.gen_range(0..reset_count);
            let (x, y) = ((positions[index_num] / 4) as usize, (positions[index_num] % 4) as usize);
            self.occupied_field[positions[index_num]] = true;
            self.data[x][y] = 2;

            return true;
        }

        return false;
    }

    fn move_right_line(&mut self, index: usize) {
        let mut cur_index = 3;
        let mut next_index = 2;
        loop {
            if self.data[index][cur_index] != 0 && self.data[index][cur_index] == self.data[index][next_index] {
                self.data[index][cur_index] *= 2;
                self.data[index][next_index] = 0;

                if next_index > 1 {
                    cur_index = next_index - 1;
                    next_index = cur_index - 1;
                } else {
                    break;
                }
            } else {
                if self.data[index][next_index] != 0 {
                    cur_index -= 1;
                }

                if self.data[index][next_index + 1] != 0 && next_index + 1 != cur_index {
                    cur_index = next_index;
                }

                if next_index > 0 {
                    next_index -= 1;
                } else {
                    break;
                }
            }
        }

        let mut newline = [0; 4];
        let mut turn_cur = 3;
        let mut loop_index = 3;
        loop {
            if self.data[index][loop_index] != 0 {
                newline[turn_cur] = self.data[index][loop_index];
                if turn_cur > 0 {
                    turn_cur -= 1;
                }
            }

            if loop_index > 0 {
                loop_index -= 1;
            } else {
                break;
            }
        }

        self.reset_occupied_field_line(index, Direction::RIGHT);
        self.redraw_horizontal_line(newline, index);
    }

    fn move_left_line(&mut self, index: usize) {
        let mut cur_index = 0;
        let mut next_index = 1;
        while next_index < 4 {
            if self.data[index][cur_index] != 0 && self.data[index][cur_index] == self.data[index][next_index] {
                self.data[index][cur_index] *= 2;
                self.data[index][next_index] = 0;
                cur_index = next_index + 1;
                next_index = cur_index + 1;
            } else {
                if self.data[index][next_index] != 0 {
                    cur_index += 1;
                }
                if self.data[index][next_index - 1] != 0 && next_index - 1 != cur_index {
                    cur_index = next_index;
                }

                next_index += 1;
            }
        }

        let mut newline = [0; 4];
        let mut turn_cur = 0;
        for elem_index in 0..4 {
            if self.data[index][elem_index] != 0 {
                newline[turn_cur] = self.data[index][elem_index];
                turn_cur += 1;
            }
        }

        self.reset_occupied_field_line(index, Direction::LEFT);
        self.redraw_horizontal_line(newline, index);
    }

    fn move_up_line(&mut self, index: usize) {
        let mut cur_index = 0;
        let mut next_index = 1;
        while next_index < 4 {
            if self.data[cur_index][index] != 0 && self.data[cur_index][index] == self.data[next_index][index] {
                self.data[cur_index][index] *= 2;
                self.data[next_index][index] = 0;
                cur_index = next_index + 1;
                next_index = cur_index + 1;
            } else {
                if self.data[next_index][index] != 0 {
                    cur_index += 1;
                }
                if self.data[next_index - 1][index] != 0 && next_index - 1 != cur_index {
                    cur_index = next_index;
                }
                next_index += 1;
            }
        }

        let mut newline = [0; 4];
        let mut turn_cur = 0;
        for elem_index in 0..4 {
            if self.data[elem_index][index] != 0 {
                newline[turn_cur] = self.data[elem_index][index];
                turn_cur += 1;
            }
        }

        self.reset_occupied_field_line(index, Direction::UP);
        self.redraw_vertical_line(newline, index);
    }

    fn move_down_line(&mut self, index: usize) {
        let mut cur_index = 3;
        let mut next_index = 2;
        loop {
            if self.data[cur_index][index] != 0 && self.data[cur_index][index] == self.data[next_index][index] {
                self.data[cur_index][index] *= 2;
                self.data[next_index][index] = 0;
                if index == 0 {
                    println!("{:?}", (cur_index, next_index));
                }
                if next_index > 1 {
                    cur_index = next_index - 1;
                    next_index = cur_index - 1;
                } else {
                    break;
                }
            } else {
                if self.data[next_index][index] != 0 {
                    cur_index -= 1;
                }

                if self.data[next_index + 1][index] != 0 && next_index + 1 != cur_index {
                    cur_index = next_index;
                }

                if next_index > 0 {
                    next_index -= 1;
                    if index == 0 {
                        println!("{:?}", (cur_index, next_index));
                    }
                } else {
                    break;
                }
            }
        }

        let mut newline = [0; 4];
        let mut turn_cur = 3;
        let mut loop_index = 3;
        loop {
            if self.data[loop_index][index] != 0 {
                newline[turn_cur] = self.data[loop_index][index];
                if turn_cur > 0 {
                    turn_cur -= 1;
                }
            }

            if loop_index > 0 {
                loop_index -= 1;
            } else {
                break;
            }
        }


        self.reset_occupied_field_line(index, Direction::DOWN);

        self.redraw_vertical_line(newline, index);
    }

    fn redraw_vertical_line(&mut self, newline: [usize; 4], index: usize) {
        for x in 0..4 {
            self.data[x][index] = newline[x];
            if self.data[x][index] > 0 {
                self.occupied_field[x * 4 + index] = true;
            }
        }
    }

    fn redraw_horizontal_line(&mut self, newline: [usize; 4], index: usize) {
        for y in 0..4 {
            self.data[index][y] = newline[y];
            if self.data[index][y] > 0 {
                self.occupied_field[index * 4 + y] = true;
            }
        }
    }

    fn reset_occupied_field_line(&mut self, index: usize, direction: Direction) {
        match direction {
            Direction::UP | Direction::DOWN => {
                for x in 0..4 {
                    self.occupied_field[x * 4 + index] = false;
                }
            }
            Direction::LEFT | Direction::RIGHT => {
                for y in 0..4 {
                    self.occupied_field[index * 4 + y] = false;
                }
            }
        }
    }
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}