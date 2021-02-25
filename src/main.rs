use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Matrix {
    pub direction: u64,
    pub data: [[usize; 4]; 4],
    pub field_occupied: [bool; 16],
}


impl Matrix {
    pub fn get_new_field_index(&mut self) -> bool {
        let mut positions = Vec::new();
        for index in 0..16 {
            if self.field_occupied[index] == false {
                positions.push(index);
            }
        }
        let reset_count =  positions.iter().count();

        if reset_count > 0 {
            println!(" p {:?}", positions);

            let mut rng = thread_rng();
            let index_num: usize = rng.gen_range(0..reset_count);
            self.field_occupied[positions[index_num]] = true;
            println!(" fils {:?}", self.field_occupied);

            let (x, y) = ((positions[index_num] / 4) as usize, (positions[index_num] % 4) as usize);
            println!(" new i: {:?}", index_num);
            println!(" new f: {:?}", (x, y));


            self.data[x][y] = 1;

            return true;
        }

        return false;
    }

    pub fn new() -> Matrix {
        let mut rng = thread_rng();

        let mut data = [[0; 4]; 4];
        let mut field_occupied = [false; 16];
        let mut init_value_count: u64 = 0;

        while init_value_count < 2 {
            let index_num = rng.gen_range(0..16);

            let x = (index_num / 4) as usize;
            let y =(index_num % 4) as usize;

            if data[x][y] == 0 {
                data[x][y] = 1;
                field_occupied[index_num] = true;
                init_value_count += 1;
            }
        }


        Matrix {
            direction: rng.gen_range(0..4),
            data,
            field_occupied,
        }
    }

    pub fn move_right_line (&mut self, index: usize) {
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

                if self.data[index][next_index + 1] == 0 && next_index + 1 != cur_index {
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
        for y in 0..4 {
            self.data[index][y] = newline[y];
            if self.data[index][y] > 0 {
                self.field_occupied[index*4 + y] = true;
            }
        }
    }

    pub fn move_left_line (&mut self, index: usize) {
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
                if self.data[index][next_index - 1] == 0 && next_index - 1 != cur_index {
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
        for y in 0..4 {
            self.data[index][y] = newline[y];
            if self.data[index][y] > 0 {
                self.field_occupied[index*4 + y] = true;
            }
        }
    }

    pub fn move_up_line (&mut self, index: usize) {
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
                if self.data[next_index - 1][index] == 0 && next_index - 1 != cur_index {
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
        for x in 0..4 {
            self.data[x][index] = newline[x];
            if self.data[x][index] > 0 {
                self.field_occupied[x * 4 + index] = true;
            }
        }
    }

    pub fn move_down_line (&mut self, index: usize) {
        let mut cur_index = 3;
        let mut next_index = 2;
        loop {
            if self.data[cur_index][index] != 0 && self.data[cur_index][index] == self.data[next_index][index] {
                self.data[cur_index][index] *= 2;
                self.data[next_index][index] = 0;

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

                if self.data[next_index + 1][index] == 0 && next_index + 1 != cur_index {
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
        for x in 0..4 {
            self.data[x][index] = newline[x];
            if self.data[x][index] > 0  {
                self.field_occupied[x * 4 + index] = true;
            }
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

        println!("{:?}",self.field_occupied);


        return self.get_new_field_index();
    }

    fn reset_occupied_field_line (&mut self, index: usize, direction: Direction) {
        match direction {
            Direction::UP | Direction::DOWN => {
                for x in 0..4 {
                    self.field_occupied[x * 4 + index] = false;
                }
            }
            Direction::LEFT | Direction::RIGHT => {
                for y in 0..4 {
                    self.field_occupied[index*4 + y] = false;
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

fn main() {
    let mut m = Matrix::new();
    println!("Hello, world! {:?}", m);
    let mut rng = thread_rng();

    for i in 0..100 {
        println!("ite {:?}", i);
        match rng.gen_range(0..4) {
            0 => {
                println!("up!");
                if m.move_to(Direction::UP) == false {
                    println!("up f! {:?}", m.data);

                    break;
                } else {
                    println!("up t! {:?}", m.data);

                }
            }
            1 => {
                println!("down!");
                if m.move_to(Direction::DOWN) == false {
                    println!("down f! {:?}", m.data);

                    break;
                } else {
                    println!("down t! {:?}", m.data);

                }
            }
            2 => {
                println!("left!");

                if m.move_to(Direction::LEFT) == false {
                    println!("left f! {:?}", m.data);

                    break;
                }  else {
                    println!("left t! {:?}", m.data);

                }
            }
            3 => {
                println!("right!");

                if m.move_to(Direction::RIGHT) == false {
                    println!("right f! {:?}", m.data);

                    break;
                } else {
                    println!("right t! {:?}", m.data);

                }
            }
            _ => panic!("error")
        }
    }
}
