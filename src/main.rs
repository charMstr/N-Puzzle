use std::num::ParseIntError;
use std::io;
use std::fmt;
use rand::Rng;

#[cfg(test)]
mod test;

#[derive(Debug)]
struct Puzzle {
    grid: Vec<u8>,
    size:u8,
}

/// redefinition of Display in order to format our grid in a puzzle form
impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(for (i,val) in self.grid.iter().enumerate() {
            if i as u8 % self.size == 0 {   // putting "chariot return" in order to format the output
                write!(f,"\n");
            }
            if *val == 0 { // printing blanck cell instead of '0'
                write!(f, "  | ");
            } 
            else {
                write!(f, "{} | ", self.grid[i]);
            }
        })
    }
}

impl Puzzle {

    /// this method build a random puzzle with the puzzle size gave in argument
    /// input : Puzzle::new_from_input(3)   output : vec!(0..9)
    /// this constructor will be usefull to have a determinated state in order to run some tests on the other functions
    fn new_from_input(size: u8) -> Puzzle {
        let puz : Puzzle = Puzzle {
            grid: (0..size*size).map(u8::from).collect(),
            size
        };
        puz
    }

    /// this method build a random puzzle with the puzzle size gave in argument
    /// input : Puzzle::new_from_input_random(3)   output : random UNIQUE value beetween 0..size*size
    fn new_from_input_random(size: u8) -> Puzzle {
        let mut rng = rand::thread_rng();

        let mut tab: Vec<u8> = (0..size*size).map(u8::from).collect();
        let mut my_grid: Vec<u8> = Vec::new();

        while tab.len() > 0 {
            let r = rng.gen_range(0..tab.len());
            my_grid.push(tab[r]);
            tab.remove(r);
        }
        let puz : Puzzle = Puzzle { grid: my_grid, size };
        puz
    }

    /// this method build a puzzle thanks to the file given as an argument in the programm run
    /*fn new_from_file() -> Puzzle {

    }*/

    /// this method will be use in order to get the index of our blanck space in our grid
    /// my_puzzle = vec!(3,0,5,4,1,2,6,7,8)
    /// input : my_puzzle.getblanck_cell_index()    output : 1
    fn get_blanck_cell_index(&self) -> u8 {
        self.get_index_cell(0)
    }

    /// this method will be use in order to get the index in the grid or the number in parameter
    /// my_puzzle = vec!(3,0,5,4,1,2,6,7,8)
    /// input : my_puzzle.get_index_cell(2)   output: 5
    fn get_index_cell(&self, cell:u8) -> u8 {
        let mut index:u8 = 0;
        for (i,val) in self.grid.iter().enumerate() {
            if *val == cell {
                index = i as u8;
            }
        }
        index
    }

    /// this method will be use in order to know if a cell in our grid is possible to move or not
    /// my_puzzle = vec!(3,0,2,4,1,5,6,7,8)
    /// input : my_puzzle.is_cell_movable(2)    output : true
    /// input : my_puzzle.is_cell_movable(8)    output : false
    fn is_cell_movable(&self,cell : u8) -> bool {
        self.is_cell_adjacent(0, cell)
    }

    /// this method will be use in order to move a cell in our grid
    /// input : my_puzzle.move_cell(2)  output: false
    fn move_cell(&mut self,cell : u8) -> bool {
        let index_cell = self.get_index_cell(cell) as usize;
        let index_blanck = self.get_blanck_cell_index() as usize;
        if self.is_cell_movable(cell) {
            self.grid[index_blanck] = cell;
            self.grid[index_cell] = 0;
            return true
        }
        false
    }

    /// this method will be use to say if a cell is adjacent to an other
    /// my_puzzle = vec!(3,0,5,4,1,2,6,7,8)
    /// input : my_puzzle.is_cell_adjacent(1,2) output : true
    fn is_cell_adjacent(&self,cell_one: u8, cell_two:u8) -> bool {
        let cell_one = self.get_index_cell(cell_one);
        let cell_two = self.get_index_cell(cell_two);
        /* 
        //NOOB VERSION
        // checking for right and left move.
        if (cell_one-1 == cell_two && cell_one % self.size != 0 ) || (cell_one+1 == cell_two && cell_one+1 % self.size != 0) {
            return true
        }
        // checking for up and down move.
        if (cell_one-self.size == cell_two ) || (cell_one+self.size == cell_two) {
            return true
        }
        false*/

        // RUST MASTER VERSION
        (cell_one % self.size != 0 && cell_one-1 == cell_two)
            || (cell_one.checked_add(1).map(|n| n == cell_two && n % self.size != 0).unwrap_or(false))
            || (cell_one.checked_sub(self.size).map(|n| n == cell_two).unwrap_or(false))
            || (cell_one.checked_add(self.size).map(|n| n == cell_two).unwrap_or(false))

    }

    /// this method will be use to return a Vec of the cells which are in right position
    /// my_puzzle = vec!(3,0,2,4,1,5,6,7,8)
    /// input : my_puzzle.get_cell_adjacent_serie()   output : vec!(6,7,8)
    fn get_cell_adjacent_serie(&self) -> Vec<u8> {
        let mut my_serie:Vec<u8> = Vec::new();

        let mut count = 0;
        let mut reset = false;
        loop {
            count += 1;
            if count >= self.size*self.size {
                break;
            }

            if self.is_cell_adjacent(count, count+1) {
                if reset {
                    my_serie = Vec::new();
                }
                if my_serie.len() == 0 {
                    my_serie.push(count);
                }
                my_serie.push(count+1);
                if reset {
                    reset = false;
                }
            }
            else {
                reset = true;
            }

        }
        my_serie
    }

    /// this method will return true if the puzzle is completed. False if not
    fn is_win(&self) -> bool {
        let test = self.get_cell_adjacent_serie();
        if test.len() == (self.size as usize * self.size as usize)-1 {
            return true
        }
        false
    }

    /// this function will return the index to go in order to make the best move
    fn get_best_move() -> usize {
        3
    }
}


fn main() -> Result<(), ParseIntError> {

    println!("Give a size of the puzzle : (maximum 255)");

    let mut choice : String = String::new();
    io::stdin().read_line(&mut choice).expect("failed to readline");
    let choice :u8 = match choice.trim().parse::<u8>() {
        Ok(res) => res,
        Err(e) => {
            println!("{}",e);
            return Result::Err(e);
        }
    };

    let my_puzzle = Puzzle::new_from_input_random(choice);
    println!("my puzzle : {}",my_puzzle);

    let mut i=0;
    while !my_puzzle.is_win() && i < 3 {
        i += 1;
        //my_puzzle.move_cell(cell);
        println!("my puzzle : {}",my_puzzle);
    }

    Ok(())
}