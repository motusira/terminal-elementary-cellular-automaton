use rand::{thread_rng, Rng};

fn main() {
    println!("Enter rule number(0-255):");
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();
    let rule: u8 = inp
        .trim_end()
        .parse()
        .expect("Error: the rule is encoded with a number from 0 to 255!");
    inp.clear();
    println!("Enter the desired number of live cells at start in percent in whole numbers (at 0 there will be one live cell in the center):");
    std::io::stdin().read_line(&mut inp).unwrap();
    let distribution: u8 = inp
        .trim_end()
        .parse()
        .expect("Error: enter an integer percentage from 0 to 100!");
    inp.clear();
    println!("Enter the number of simulation steps:");
    std::io::stdin().read_line(&mut inp).unwrap();
    let simulation_steps: u32 = inp
        .trim_end()
        .parse()
        .expect("Error: enter a positive integer!");
    let mut cells = Grid::new(rule, distribution);
    cells.simulate(simulation_steps);
}

//all possible cell sequences
const BIRTH_RULES: [[bool; 3]; 8] = [
    [true, true, true],
    [true, true, false],
    [true, false, true],
    [true, false, false],
    [false, true, true],
    [false, true, false],
    [false, false, true],
    [false, false, false],
];

const SIZE: usize = 110;

//grid - сurrent state
//new_grid - next state
//they are swapped after update
struct Grid {
    grid: [bool; SIZE],
    new_grid: [bool; SIZE],
    rule: [bool; 8],
}

impl Grid {
    fn new(rule_number: u8, distribution: u8) -> Self {
        let mut grid = [false; SIZE];

        if distribution > 100 {
            panic!("Error: it is not possible to take more than 100%");
        }

        if distribution != 0 {
            let mut rng = thread_rng();
            for i in 0..((SIZE * distribution as usize) / 100) {
                let idx: usize = thread_rng().gen_range(0..SIZE);
                grid[idx] = true;
            }
        } else {
            grid[SIZE / 2] = true;
        }

        let rule = Self::gen_rule(rule_number);
        Grid {
            grid,
            new_grid: [false; SIZE],
            rule,
        }
    }

    //for each cell, check whether it and its neighbors correspond to a certain state
    fn update(&mut self) {
        let mut neibs = [false; 3];
        let mut i = 0;
        while i < self.grid.len() {
            //left neighbor check
            if i == 0 {
                neibs[0] = self.grid[SIZE - 1];
            } else {
                neibs[0] = self.grid[i - 1];
            }
            //right neighbor check
            if i == SIZE - 1 {
                neibs[2] = self.grid[0];
            } else {
                neibs[2] = self.grid[i + 1];
            }
            //checking the cell itself
            neibs[1] = self.grid[i];
            //checking the current state to determine the state of the cell at the next stage
            for (idx, elem) in BIRTH_RULES.iter().enumerate() {
                if self.rule[idx] && &neibs[..] == &elem[..] {
                    self.new_grid[i] = true;
                }
            }
            i += 1
        }
        self.grid = self.new_grid;
        self.new_grid = [false; SIZE];
    }

    fn render(&self) {
        for i in self.grid {
            if i {
                print!("@");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }

    fn simulate(&mut self, i: u32) {
        self.render();
        for _i in 0..i {
            self.update();
            self.render();
        }
    }

    //representation of a number as 8 bits(Wolfram code)
    fn gen_rule(mut n: u8) -> [bool; 8] {
        let mut rule: [bool; 8] = [false; 8];
        let mut answ: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let mut idx: usize = 0;
        while n != 0u8 {
            let a = n % 2;
            answ[idx] = a;
            n /= 2;
            idx += 1;
        }
        answ.reverse();
        for (idx, i) in answ.iter().enumerate() {
            if *i == 1 {
                rule[idx] = true;
            }
        }
        rule
    }
}
