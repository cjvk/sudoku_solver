//mod squares;

use crate::squares::InitialSquare;
use crate::squares::DerivedSquare;
use crate::squares::Square;
use crate::config;

pub struct SudokuPuzzle {
    pub initial_strings: Vec<String>,
    pub rows: Vec<Vec<Box<dyn Square>>>,
}
impl SudokuPuzzle {
    pub fn new(initial_strings: Vec<String>) -> Self {
        assert_eq!(initial_strings.len(), 9);
        initial_strings
            .iter()
            .for_each(|s| assert_eq!(s.len(), 9));
        let rows = SudokuPuzzle::calculate_rows_from_initial_strings(&initial_strings);
        SudokuPuzzle {
            initial_strings,
            rows: rows,
        }
    }
    pub fn clone_from(from_puzzle: &SudokuPuzzle) -> Self {
        // this is buggy (it always creates derived squares)
        // update: bug fixed! (uses clone)
        let mut rows: Vec<Vec<Box<dyn Square>>> = vec![];

        let mut initial_strings: Vec<String> = vec![];
        for i in 0..9 {
            initial_strings.push(from_puzzle.initial_strings[i].clone());
        }

        from_puzzle.rows
            .iter()
            .for_each(|from_row| {
                let mut row: Vec<Box<dyn Square>> = vec![];
                (*from_row)
                    .iter()
                    .for_each(|sq| {
                        // let _unused = sq.clone(); // this works
                        // let _unused: Box<dyn Square> = (*sq).clone(); // so does this
                        // row.push(Box::new(DerivedSquare::copy(sq.get_bitmap())));
                        // row.push(sq.clone());
                        // row.push(Box::new(*(sq.clone())));
                        // row.push(Box::new(DerivedSquare::copy(sq.get_bitmap()))); // this works
                        row.push((*sq).clone()); // so does this
                    });
                rows.push(row);
            });

        SudokuPuzzle {
            initial_strings,
            rows,
        }
    }
    pub fn get_square(&mut self, row: usize, column: usize) -> &mut Box<dyn Square> {
        &mut self.rows[row-1][column-1]
    }
    pub fn debug_print_puzzle(&self) {
        if config::DEBUG_PRINT_ENABLED {
            self.print_puzzle();
        }
    }
    pub fn print_puzzle(&self) {
        println!("-----------");
        self.rows
            .iter()
            .for_each(|row| {
                //println!("{}", (*row)[0].count_remaining());
                print!("|");
                (*row)
                    .iter()
                    .for_each(|boxsquare| {
                        //print!(".")
                        match (**boxsquare).has_single_value() {
                            true => print!("{}", (**boxsquare).get_single_value_nomut()),
                            false => print!(" "),
                        }
                    });
                println!("|");
                // *row
                    // .iter()
                    // .for_each(|boxsquare| {
                    //     print!("{}", '.');
                    // });
            });
        println!("-----------");
    }
    fn calculate_rows_from_initial_strings(initial_strings: &Vec<String>) -> Vec<Vec<Box<dyn Square>>> {
        let mut rows: Vec<Vec<Box<dyn Square>>> = vec![];

        initial_strings
            .iter()
            .for_each(|s| {
                let mut row: Vec<Box<dyn Square>> = vec![];
                (*s)
                    .chars()
                    .for_each(|c| {
                        match c {
                            ' ' => row.push(Box::new(DerivedSquare::new())),
                            _ => row.push(Box::new(InitialSquare::new(c.to_string().parse::<usize>().unwrap()))),
                        }
                    });
                rows.push(row);
            });

        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_pass() {
        assert!(true);
    }

    fn empty_puzzle() -> Vec<String> {
        [
            "         ",
            "         ",
            "         ",
            "         ",
            "         ",
            "         ",
            "         ",
            "         ",
            "         ",
        ]
            .iter()
            .cloned()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn create() {
        let puzzle = SudokuPuzzle::new(empty_puzzle());
    }

    #[test]
    fn clone() {
        let mut puzzle = SudokuPuzzle::new(empty_puzzle());
        puzzle.get_square(1,1).select(1);
        assert_eq!(puzzle.get_square(1,1).count_remaining(), 1);
        assert_eq!(puzzle.get_square(1,2).count_remaining(), 9);

        let mut cloned_puzzle = SudokuPuzzle::clone_from(&puzzle);
        assert_eq!(cloned_puzzle.get_square(1,1).count_remaining(), 1);
        assert_eq!(cloned_puzzle.get_square(1,2).count_remaining(), 9);

        cloned_puzzle.get_square(1,2).select(2);
        assert_eq!(puzzle.get_square(1,1).count_remaining(), 1);
        assert_eq!(puzzle.get_square(1,2).count_remaining(), 9);
        assert_eq!(cloned_puzzle.get_square(1,1).count_remaining(), 1);
        assert_eq!(cloned_puzzle.get_square(1,2).count_remaining(), 1);

        let mut another_clone = SudokuPuzzle::clone_from(&cloned_puzzle);
        let bitmap1 = another_clone.get_square(1,1).get_bitmap();
        let bitmap2 = another_clone.get_square(1,2).get_bitmap();
        another_clone.get_square(1,3).bitmap_elimination(bitmap1);
        another_clone.get_square(1,3).bitmap_elimination(bitmap2);
        assert_eq!(another_clone.get_square(1,1).count_remaining(), 1);
        assert_eq!(another_clone.get_square(1,2).count_remaining(), 1);
        assert_eq!(another_clone.get_square(1,3).count_remaining(), 7);
        let mut yet_another = SudokuPuzzle::clone_from(&another_clone);
        yet_another.get_square(4,1).bitmap_elimination(bitmap1);
        assert_eq!(another_clone.get_square(4,1).count_remaining(), 9);
        assert_eq!(yet_another.get_square(4,1).count_remaining(), 8);
    }
    
}
