use crate::sudoku_puzzle::SudokuPuzzle;
use crate::solver::QueueItem;
use crate::squares;
use crate::stopwatch::Stopwatch;
use crate::config;
use std::collections::HashMap;

pub trait Constraint {
    fn process(&mut self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle) -> Result<(), ConstraintViolation>;
    fn name(&self) -> String;
    fn elapsed_millis(&self) -> u128;
    fn elapsed_sub_millis(&self) -> Option<u128> { None }
}

#[derive(Debug)]
pub struct ConstraintViolation {
    pub msg: String,
}

pub struct DoubleDoubleBox {
    stopwatch: Stopwatch,
}
impl DoubleDoubleBox {
    pub fn new() -> Self {
        {
            let stopwatch = Stopwatch::new();
            DoubleDoubleBox {
                stopwatch
            }
        }
    }
    fn _process_deprecated(&self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle) -> Result<(), ConstraintViolation> {
        if puzzle.get_square(queue_item.row,queue_item.column).count_remaining() == 2 {
            let bitmap = puzzle.get_square(queue_item.row,queue_item.column).get_bitmap();

            [1,1,1,2,2,2,3,3,3].iter().zip([1,2,3,1,2,3,1,2,3].iter())
                .map(|(i,j)| { (i+((box_util::index_to_box(queue_item.row)-1)*3),
                                j+((box_util::index_to_box(queue_item.column)-1)*3))})
                .filter(|(i,j)| *i!=queue_item.row || *j!=queue_item.column)
                .for_each(|(i,j)| {
                    if puzzle.get_square(i,j).get_bitmap() == bitmap {
                        // bingo!
                        [1,1,1,2,2,2,3,3,3].iter().zip([1,2,3,1,2,3,1,2,3].iter())
                            .map(|(i2,j2)| { (i2+((box_util::index_to_box(queue_item.row)-1)*3),
                                              j2+((box_util::index_to_box(queue_item.column)-1)*3))})
                            .filter(|(i2,j2)| *i2!=queue_item.row || *j2!=queue_item.column)
                            .filter(|(i2,j2)| *i2!=i || *j2!=j)
                            .for_each(|(i,j)| puzzle.get_square(i,j).bitmap_elimination(bitmap));
                    }
                });
        }
        Ok(())
    }
}
impl Constraint for DoubleDoubleBox {
    fn name(&self) -> String {
        "DoubleDoubleBox".to_string()
    }
    fn elapsed_millis(&self) -> u128 {
        self.stopwatch.elapsed_millis()
    }
    fn process(&mut self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle) -> Result<(), ConstraintViolation> {
        self.stopwatch.start();
        let row = queue_item.row;
        let col = queue_item.column;
        if puzzle.get_square(row,col).count_remaining() == 2 {
            let bitmap = puzzle.get_square(row,col).get_bitmap();
            box_util::box8(row,col).iter().for_each(|(i,j)| {
                if puzzle.get_square(*i,*j).get_bitmap() == bitmap {
                    // bingo!
                    box_util::box9(row,col).iter()
                        .filter(|(i2,j2)| *i2!=row || *j2!=col)
                        .filter(|(i2,j2)| *i2!=*i || *j2!=*j)
                        .for_each(|(i,j)| puzzle.get_square(*i,*j).bitmap_elimination(bitmap));
                }
            });
        }
        self.stopwatch.stop();
        Ok(())
    }
}

pub struct DoubleDoubleColumn {
    stopwatch: Stopwatch,
}
impl DoubleDoubleColumn {
    pub fn new() -> Self {
        {
            let stopwatch = Stopwatch::new();
            DoubleDoubleColumn {
                stopwatch
            }
        }
    }
}
impl Constraint for DoubleDoubleColumn {
    fn name(&self) -> String {
        "DoubleDoubleColumn".to_string()
    }
    fn elapsed_millis(&self) -> u128 {
        self.stopwatch.elapsed_millis()
    }
    fn process(&mut self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle) -> Result<(), ConstraintViolation> {
        self.stopwatch.start();
        if puzzle.get_square(queue_item.row,queue_item.column).count_remaining() == 2 {
            let bitmap = puzzle.get_square(queue_item.row,queue_item.column).get_bitmap();
            (1..=9)
                .filter(|r| *r != queue_item.row)
                .for_each(|r| {
                    if puzzle.get_square(r, queue_item.column).get_bitmap() == bitmap {
                        // bingo!
                        (1..=9).filter(|r2| *r2 != queue_item.row && *r2 != r)
                            .for_each(|r| puzzle.get_square(r, queue_item.column).bitmap_elimination(bitmap));
                    }
                });
        }
        self.stopwatch.stop();
        Ok(())
    }
}

pub struct DoubleDoubleRow {
    stopwatch: Stopwatch,
}
impl DoubleDoubleRow {
    pub fn new() -> Self {
        {
            let stopwatch = Stopwatch::new();
            DoubleDoubleRow {
                stopwatch
            }
        }
    }
}
impl Constraint for DoubleDoubleRow {
    fn name(&self) -> String {
        "DoubleDoubleRow".to_string()
    }
    fn elapsed_millis(&self) -> u128 {
        self.stopwatch.elapsed_millis()
    }
    fn process(&mut self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle) -> Result<(), ConstraintViolation> {
        self.stopwatch.start();
        if puzzle.get_square(queue_item.row,queue_item.column).count_remaining() == 2 {
            let bitmap = puzzle.get_square(queue_item.row,queue_item.column).get_bitmap();
            (1..=9)
                .filter(|c| *c != queue_item.column)
                .for_each(|c| {
                    if puzzle.get_square(queue_item.row, c).get_bitmap() == bitmap {
                        // bingo!
                        (1..=9).filter(|c2| *c2 != queue_item.column && *c2 != c)
                            .for_each(|c| puzzle.get_square(queue_item.row, c).bitmap_elimination(bitmap));
                    }
                });
        }
        self.stopwatch.stop();
        Ok(())
    }
}

pub struct AllCannotBeEliminated {
    stopwatch: Stopwatch,
}
impl AllCannotBeEliminated {
    pub fn new() -> Self {
        {
            let stopwatch = Stopwatch::new();
            AllCannotBeEliminated {
                stopwatch
            }
        }
    }
}
impl Constraint for AllCannotBeEliminated {
    fn name(&self) -> String {
        "AllCannotBeEliminated".to_string()
    }
    fn elapsed_millis(&self) -> u128 {
        self.stopwatch.elapsed_millis()
    }
    fn process(&mut self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle) -> Result<(), ConstraintViolation> {
        self.stopwatch.start();
        let row = queue_item.row;
        let col = queue_item.column;
        self.stopwatch.stop();
        match puzzle.get_square(row,col).count_remaining() {
            0 => Err(ConstraintViolation{msg: "Contradiction".to_string()}),
            _ => Ok(()),
        }
    }
}

pub struct ProcessOfElimination {
    stopwatch: Stopwatch,
    stopwatch2: Stopwatch,
    map: HashMap<(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize), bool>,
    is_memoization_enabled: bool,
}
impl ProcessOfElimination {
    pub fn new() -> Self {
        {
            let stopwatch = Stopwatch::new();
            let stopwatch2 = Stopwatch::new();
            ProcessOfElimination {
                stopwatch,
                stopwatch2,
                map: HashMap::new(),
                is_memoization_enabled: true,
            }
        }
    }
    fn already_processed(&self, key: (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize)) -> bool {
        self.map.contains_key(&key)
    }
    fn set_already_processed(&mut self, key: (usize, usize, usize, usize, usize, usize, usize, usize, usize, usize)) {
        self.map.insert(key, true);
    }
}
impl Constraint for ProcessOfElimination {
    fn elapsed_sub_millis(&self) -> Option<u128> {
        Some(self.stopwatch2.elapsed_millis())
    }
    fn name(&self) -> String {
        match self.is_memoization_enabled {
            true => "ProcessOfEliminationWithMemoization".to_string(),
            false => "ProcessOfElimination".to_string(),
        }
    }
    fn elapsed_millis(&self) -> u128 {
        self.stopwatch.elapsed_millis()
    }
    fn process(&mut self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle)
               -> Result<(), ConstraintViolation>
    {

        if !config::IS_POE_ENABLED {
            return Ok(());
        }

        self.stopwatch.start();

        //self.stopwatch2.start();

        let function_list = vec![
            // row/column/box squares 9 follow this format: f(queue_item) => Vec<(i,j)>
            Box::new(|qitem: &QueueItem| {
                // row squares 9
                //vec![(1 as usize,1 as usize)]})
                //(1..=9).map(|c| (queue_item.row, c as usize)).collect()})
                (1..=9).map(|c| (qitem.row, c as usize)).collect()})
                as Box<dyn Fn(&QueueItem) -> Vec<(usize, usize)>>,
            Box::new(|qitem: &QueueItem| {
                // column squares 9
                (1..=9).map(|r| (r as usize, qitem.column)).collect()})
                as Box<dyn Fn(&QueueItem) -> Vec<(usize, usize)>>,
            Box::new(|qitem: &QueueItem| {
                // box squares 9
                let row_box = box_util::index_to_box(qitem.row);
                let col_box = box_util::index_to_box(qitem.column);
                [1,1,1,2,2,2,3,3,3]
                    .iter()
                    .zip([1,2,3,1,2,3,1,2,3].iter())
                    .map(|(i,j)| {
                        let row_offset = (row_box - 1) * 3;
                        let col_offset = (col_box - 1) * 3;
                        (i+row_offset, j+col_offset)
                    })
                    .collect()})
                as Box<dyn Fn(&QueueItem) -> Vec<(usize, usize)>>,
        ];

        //self.stopwatch2.stop();
        //self.stopwatch2.start();

        let mut function_list_index = 0;
        let return_early = function_list
            .iter()
            .try_fold((), |_acc, f| {
                // self.stopwatch2.start(); // 44%

                // this is an expensive constraint
                // under what heuristics would it make sense to continue?
                //
                // summary:
                // 1. get all 9 bitmaps for the row (or column or box)
                // 2. calculate (bitwise) whether all values are:
                //    set ever
                //    set an odd number of times
                //    set more than once
                // 3. every value must have been set at least once, or error
                // 4. calculate set_exactly_once as bitwise operation from odd and more than once
                // ==> this next step looks algorithmically ugly
                // 5. for all possible values which are set exactly once
                //      look through the row for all squares which have the value set (expecting one)
                //      select the value for that square
                //      error handling: if no squares or multiple squares found, contradiction

                // memoization
                // self.stopwatch2.start(); // 0%
                let unique_key = match function_list_index {
                    0 => queue_item.row,
                    1 => 10 + queue_item.column,
                    2 => {
                        let row_box_number = box_util::index_to_box(queue_item.row);
                        let col_box_number = box_util::index_to_box(queue_item.column);
                        100 + (10 * row_box_number) + col_box_number
                    },
                    _ => panic!("unexpected value for function_list_index!!"),
                };
                // self.stopwatch2.stop(); // 0%

                // deduction within my row, column, box

                // self.stopwatch2.start(); // 28%
                // // get all 9 bitmaps
                // // and already selected squares
                // // let mut bitmaps: Vec<usize> = vec![];
                // let mut already_selected: Vec<bool> = vec![false; 9];
                // f(queue_item)
                //     .iter()
                //     .for_each(|(i,j)| {
                //         let sq = puzzle.get_square(*i,*j);
                //         // bitmaps.push(sq.get_bitmap());
                //         if sq.has_single_value() {
                //             let single_value = sq.get_single_value();
                //             already_selected[single_value-1] = true;
                //         }
                //     });
                // self.stopwatch2.stop(); // 28%

                //self.stopwatch2.start(); // 3%
                let bitmaps: Vec<usize> = f(queue_item)
                    .iter()
                    .map(|(i,j)| puzzle.get_square(*i,*j).get_bitmap())
                    .collect();
                //self.stopwatch2.stop(); // 3%

                // from bitmaps, get already selected values
                let mut already_selected: Vec<bool> = vec![false; 9];
                bitmaps
                    .iter()
                    .filter(|&bitmap| squares::DerivedSquare::number_of_bits(*bitmap) == 1)
                    .for_each(|bitmap| {
                        match bitmap {
                            1 => already_selected[0] = true,
                            2 => already_selected[1] = true,
                            4 => already_selected[2] = true,
                            8 => already_selected[3] = true,
                            16 => already_selected[4] = true,
                            32 => already_selected[5] = true,
                            64 => already_selected[6] = true,
                            128 => already_selected[7] = true,
                            256 => already_selected[8] = true,
                            _ => panic!("This should not happen"),
                        }
                    });

                // // self.stopwatch2.start(); // 4%
                // let mut already_selected: Vec<bool> = vec![false; 9];
                // f(queue_item)
                //     .iter()
                //     .for_each(|(i,j)| {
                //         if puzzle.get_square(*i,*j).has_single_value() {
                //             let single_value = puzzle.get_square(*i,*j).get_single_value();
                //             already_selected[single_value-1] = true;
                //         }
                //     });
                // // self.stopwatch2.stop(); // 4%

                let already_processed_key = (
                    unique_key,
                    bitmaps[0],
                    bitmaps[1],
                    bitmaps[2],
                    bitmaps[3],
                    bitmaps[4],
                    bitmaps[5],
                    bitmaps[6],
                    bitmaps[7],
                    bitmaps[8],
                );
                if self.is_memoization_enabled && self.already_processed(already_processed_key) {
                    //self.stopwatch2.stop();
                    return Ok(());
                }

                //self.stopwatch2.start(); // 0%
                let (mut set_ever, mut set_odd, mut set_two_plus_times): (usize, usize, usize) = (0,0,0);
                bitmaps
                    .iter()
                    .for_each(|bitmap| {
                        set_ever = set_ever | bitmap;
                        set_odd = set_odd ^ bitmap;
                        set_two_plus_times = set_two_plus_times | (set_ever & (!set_odd & 511));
                    });
                //self.stopwatch2.stop(); // 0%

                if set_ever != 511 {
                    //self.stopwatch2.stop();
                    return Err::<(),ConstraintViolation>
                        (ConstraintViolation{msg: "Contradiction".to_string()});
                }

                let set_exactly_once = set_odd & (511 & !set_two_plus_times);

                //self.stopwatch2.stop(); // 44%
                //self.stopwatch2.start(); // 75% -> 3% -> 10%

                let mut iterator_error: Option<ConstraintViolation> = None;
                // have to filter out the squares which are InitialSquares!
                // filter out values which are "set exactly once" only because
                // a square has it already selected!
                (1..=9)
                    .map(|i| i as usize)
                    .filter(|possible_value|
                            !already_selected[possible_value-1])
                    .filter(|possible_value|
                            squares::get_bitmap_values()[*possible_value] & set_exactly_once != 0)
                    .for_each(|possible_value| {
                        //self.stopwatch2.start();
                        let mut found = false;
                        let mut found_two_plus = false;
                        f(queue_item)
                            .iter()
                            .for_each(|(i,j)| {
                                let s = puzzle.get_square(*i,*j);
                                if s.is_possible(possible_value) {
                                    found_two_plus = found;
                                    found = true;
                                    s.select(possible_value);
                                }
                            });
                        if !found {
                            iterator_error =
                                Some(ConstraintViolation{msg:"should be found".to_string()});
                        }
                        if found_two_plus {
                            iterator_error =
                                Some(ConstraintViolation
                                     {msg:"should not be found more than once!".to_string()});
                        }
                        //self.stopwatch2.stop();
                    });
                //self.stopwatch2.stop(); // 75% -> 3% -> 10%
                match iterator_error {
                    Some(e) => {
                        return Err(e);
                    }
                    _ => ()
                }
                function_list_index = function_list_index + 1;
                self.set_already_processed(already_processed_key);
                Ok(())
            });

        self.stopwatch.stop();
        //self.stopwatch2.stop();
        match return_early {
            Ok(_) => {
                Ok(())
            },
            Err(_) => {
                Err(ConstraintViolation{msg: "Contradiction".to_string()})
            }
        }

    }
    // e.g. in 4-sudoku, you have (23) (24) (1234) (34), you would select 1
    // given a number of bitmaps, how to count if certain position has
    // exactly 1 member set? Using bit operations
    // translate above to (0110) (0101) (1111) (0011)

    // for each of rowsquares9, columnsquares9, boxsquares9
    // get the bitmaps for all the squares
    // go through the bitmaps, calculate
    // set_ever (whether that bit was ever set)
    // set_odd (was the bit set an odd number of times)
    // set_two_plus_times (set more than 2x)

    // set_ever is used to calculate a constraint violation
    // (I need checked exceptions !) // constraint violation should be defined in this file
    // set_odd and set_two_plus_times are combined to make set_exactly_once

    // for all the values which are set exactly once
    // go find the square which has this value as possible and select it
    // repeat this for columns and boxes
}

pub struct NoRowDuplicates {
    stopwatch: Stopwatch,
}
impl NoRowDuplicates {
    pub fn new() -> Self {
        {
            let stopwatch = Stopwatch::new();
            NoRowDuplicates {
                stopwatch
            }
        }
    }
}
impl Constraint for NoRowDuplicates {
    fn name(&self) -> String {
        "NoRowDuplicates".to_string()
    }
    fn elapsed_millis(&self) -> u128 {
        self.stopwatch.elapsed_millis()
    }
    fn process(&mut self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle) -> Result<(), ConstraintViolation> {
        self.stopwatch.start();
        let row = queue_item.row;
        let col = queue_item.column;
        if puzzle.get_square(row,col).has_single_value() {
            let bitmap = puzzle.get_square(row,col).get_bitmap();
            (1..10)
                .filter(|c| *c != col)
                .for_each(|c| puzzle.get_square(row,c).bitmap_elimination(bitmap));
        }
        self.stopwatch.stop();
        Ok(())
    }
}

pub struct NoColumnDuplicates {
    stopwatch: Stopwatch,
}
impl NoColumnDuplicates {
    pub fn new() -> Self {
        {
            let stopwatch = Stopwatch::new();
            NoColumnDuplicates {
                stopwatch
            }
        }
    }
}
impl Constraint for NoColumnDuplicates {
    fn name(&self) -> String {
        "NoColumnDuplicates".to_string()
    }
    fn elapsed_millis(&self) -> u128 {
        self.stopwatch.elapsed_millis()
    }
    fn process(&mut self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle) -> Result<(), ConstraintViolation> {
        self.stopwatch.start();
        let row = queue_item.row;
        let col = queue_item.column;
        if puzzle.get_square(row,col).has_single_value() {
            let bitmap = puzzle.get_square(row,col).get_bitmap();
            (1..10)
                .filter(|r| *r != row)
                .for_each(|r| puzzle.get_square(r,col).bitmap_elimination(bitmap));
        }
        self.stopwatch.stop();
        Ok(())
    }
}

pub mod box_util {
    pub fn box8(row: usize, column: usize) -> Vec<(usize, usize)> {
        box9(row, column).iter()
            .map(|(i,j)| (*i,*j))
            .filter(|(i,j)| (*i)!=row || (*j)!=column)
            .collect()
    }
    pub fn box9(row: usize, column: usize) -> Vec<(usize, usize)> {
        let row_box = index_to_box(row);
        let column_box = index_to_box(column);

        [1,1,1,2,2,2,3,3,3].iter().zip([1,2,3,1,2,3,1,2,3].iter())
            .map(|(i,j)| (*i as usize, *j as usize))
            .map(|(i,j)| {
                // row_box of 1 is add 0 to everything
                // row_box of 2 is add 3 to everything
                // row_box of 3 is add 6 to everything
                let row_offset = (row_box - 1) * 3;
                let column_offset = (column_box - 1) * 3;
                (i+row_offset, j+column_offset)
            })
            .collect()
    }
    pub fn index_to_box(row: usize) -> usize {
        // calling it row but works for column
        // 1-3 => 1
        // 4-6 => 2
        // 7-9 => 3
        match row {
            1 ..= 3 => 1,
            4 ..= 6 => 2,
            7 ..= 9 => 3,
            _ => panic!("non-sudoku value!!"),
        }
    }
    fn _box_to_range(box_number: usize) -> std::ops::Range<usize> {
        match box_number {
            1 => (1..4),
            2 => (4..7),
            3 => (7..10),
            _ => panic!("non-sudoku value!!"),
        }
    }
}

pub struct NoBoxDuplicates {
    stopwatch: Stopwatch,
}
impl NoBoxDuplicates {
    pub fn new() -> Self {
        {
            let stopwatch = Stopwatch::new();
            NoBoxDuplicates {
                stopwatch
            }
        }
    }
    fn _process_deprecated(&self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle) -> Result<(), ConstraintViolation> {
        let row = queue_item.row;
        let col = queue_item.column;
        if puzzle.get_square(row,col).has_single_value() {
            let bitmap = puzzle.get_square(row,col).get_bitmap();

            // can I use two ranges and zip??

            let row_box = box_util::index_to_box(row);
            let col_box = box_util::index_to_box(col);

            // the two ranges will be in principle
            // [1,1,1,2,2,2,3,3,3] and [1,2,3,1,2,3,1,2,3]
            // but there will be an offset

            [1,1,1,2,2,2,3,3,3]
                .iter()
                .zip([1,2,3,1,2,3,1,2,3].iter())
                .map(|(i,j)| {
                    // row_box of 1 is add 0 to everything
                    // row_box of 2 is add 3 to everything
                    // row_box of 3 is add 6 to everything
                    let row_offset = (row_box - 1) * 3;
                    let col_offset = (col_box - 1) * 3;
                    (i+row_offset, j+col_offset)
                })
                .filter(|(i,j)| *i!=row || *j!=col)
                .for_each(|(i,j)| puzzle.get_square(i,j).bitmap_elimination(bitmap));

        }
        Ok(())
    }
}
impl Constraint for NoBoxDuplicates {
    fn name(&self) -> String {
        "NoBoxDuplicates".to_string()
    }
    fn elapsed_millis(&self) -> u128 {
        self.stopwatch.elapsed_millis()
    }
    fn process(&mut self, queue_item: &QueueItem, puzzle: &mut SudokuPuzzle) -> Result<(), ConstraintViolation> {
        self.stopwatch.start();
        let row = queue_item.row;
        let col = queue_item.column;
        if puzzle.get_square(row,col).has_single_value() {
            let bitmap = puzzle.get_square(row,col).get_bitmap();
            box_util::box8(row,col).iter()
                .for_each(|(i,j)| puzzle.get_square(*i,*j).bitmap_elimination(bitmap));
        }
        self.stopwatch.stop();
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::sudoku_puzzle::SudokuPuzzle;
    use crate::constraint::Constraint;
    use crate::constraint::NoRowDuplicates;
    use crate::constraint::NoColumnDuplicates;
    use crate::constraint::NoBoxDuplicates;
    use crate::constraint::ProcessOfElimination;
    use crate::solver::QueueItem;
    use crate::config;
    use super::box_util;

    #[test]
    fn test_break_loop() {
        let mut b = false;

        for i in (0..10) {
            if i == 5 {
                b = true;
                break;
            }
        }

        assert!(b);

        let mut b = false;
        'outer: for i in (0..10) {
            'inner: for j in (0..10) {
                if j == 8 {
                    break 'outer;
                }
            }
            if i == 1 {
                panic!("should not reach 1");
            }
        }
        

    }

    #[test]
    fn test_try_fold() {

        fn foo (i: usize) -> Result<usize,()> {
            println!("entering foo, i={}", i);
            match i {
                4 => Err(()),
                _ => Ok(i),
            }
        }

        // let a: (usize, usize, usize) = (1, 2, 3);

        // let result = [1 as usize, 2, 3]
        //     .iter()
        //     .try_fold(0 as usize, |acc, &x| foo(x));

        // assert!(result == Ok(3));

        let mut it = [1 as usize, 2, 3, 4, 5].iter();
        let result = it.try_fold(0 as usize, |acc, x| {
            println!("about to call foo({}), acc={:?}", *x, acc);
            foo(*x)
        });

        assert!(result == Err(()));
        assert!(it.len() == 1);
        assert!(it.next() == Some(&5));
        
    }

    #[test]
    fn test_process_of_elimination() {
        let sample_puzzle: Vec<String> =

        [
            "18       ",
            "2  8     ",
            "3     8  ",
            "4 8      ",
            "5   8    ",
            "6      8 ",
            "7    8   ",
            "        8",
            "         ",
        ]
        .iter()
        .cloned()
        .map(|s| s.to_string())
        .collect();

        let mut puzzle = SudokuPuzzle::new(sample_puzzle);

        let mut constraint1 = ProcessOfElimination:: new();
        let mut constraint1: Box<dyn Constraint> = Box::new(ProcessOfElimination::new());
        let mut constraint2: Box<dyn Constraint> = Box::new(NoColumnDuplicates::new());
        let mut constraint3: Box<dyn Constraint> = Box::new(NoRowDuplicates::new());

        // reminder: (1..10) goes from 1-9

        assert!(!puzzle.get_square(9,1).has_single_value());
        [(1,1),(2,1),(3,1),(4,1),(5,1),(6,1),(7,1),(1,2),(2,4),(3,7),(4,3),(5,5),(6,8),(7,6),(8,9),(1,1),]
            .iter()
            .for_each(|(i,j)| {
                constraint1.process(&QueueItem{row:*i, column:*j}, &mut puzzle);
                constraint2.process(&QueueItem{row:*i, column:*j}, &mut puzzle);
                constraint3.process(&QueueItem{row:*i, column:*j}, &mut puzzle);
            });
        // hack for now
        if config::IS_POE_ENABLED {
            assert!(puzzle.get_square(9,1).has_single_value());
        }
        
    }

    #[test]
    fn test_no_box_duplicates2() {
         /*
-----------
|261943587|
|385721694|
|749586123|
|654218739|
|138479256|
|972365841|
|526194378|
|413857962|
|897632415|
-----------
         */
       
        let sample_puzzle: Vec<String> =

        [
            "261943587",
            "385721694",
            "74 5 6 23",
            "654218739",
            " 384 925 ",
            "972365841",
            "52 1 4 78",
            "413857962",
            "897632415",
        ]
        .iter()
        .cloned()
        .map(|s| s.to_string())
        .collect();

        let mut puzzle = SudokuPuzzle::new(sample_puzzle);

        let mut constraint = NoBoxDuplicates::new();
        let mut constraint: Box<dyn Constraint> = Box::new(NoBoxDuplicates::new());

        // reminder: (1..10) goes from 1-9

        // box 1 (upper left)
        let (row, col) = (3,3);
        let mystery_value = 9;
        let range1 = [1,1,1,2,2,2,3,3,3];
        let range2 = [1,2,3,1,2,3,1,2,3];
        assert!(!puzzle.get_square(row,col).has_single_value());
        range1
            .iter()
            .zip(range2.iter())
            .map(|(i,j)| (*i as usize, *j as usize))
            .filter(|(i,j)| *i!=row || *j!=col)
            .for_each(|(i,j)| {
                constraint.process(&QueueItem{row:i, column:j}, &mut puzzle);
            });
        assert!(puzzle.get_square(row,col).has_single_value());
        assert!(puzzle.get_square(row,col).get_single_value() == mystery_value);

        fn assertions(
            row: usize,
            col: usize,
            mystery_value: usize,
            range1: Vec<usize>,
            range2: Vec<usize>,
            puzzle: &mut SudokuPuzzle,
            constraint: &mut Box<dyn Constraint>
        ) -> () {
            assert!(!puzzle.get_square(row,col).has_single_value());
            range1
                .iter()
                .zip(range2.iter())
                .map(|(i,j)| (*i as usize, *j as usize))
                .filter(|(i,j)| *i!=row || *j!=col)
                .for_each(|(i,j)| {
                    println!("i,j={},{}",i,j);
                    constraint.process(&QueueItem{row:i, column:j}, puzzle);
                });
            assert!(puzzle.get_square(row,col).has_single_value());
            assert!(puzzle.get_square(row,col).get_single_value() == mystery_value);
        }

        // box 2 (upper middle)
        let (row, col, mystery_value) = (3,5,8);
        let (range1,range2) = ([1,1,1,2,2,2,3,3,3].to_vec(),[4,5,6,4,5,6,4,5,6].to_vec());
        assertions(row, col, mystery_value, range1, range2, &mut puzzle, &mut constraint);

        // box 3 (upper right)
        let (row, col, mystery_value) = (3,7,1);
        let (range1,range2) = ([1,1,1,2,2,2,3,3,3].to_vec(),[7,8,9,7,8,9,7,8,9].to_vec());
        assertions(row, col, mystery_value, range1, range2, &mut puzzle, &mut constraint);

        // box 4 (middle left)
        let range1 = [4,4,4,5,5,5,6,6,6].to_vec();
        let range2 = [1,2,3,1,2,3,1,2,3].to_vec();
        assertions(5,1,1,range1,range2, &mut puzzle, &mut constraint);
        
        // box 5 (middle middle)
        let range1 = [4,4,4,5,5,5,6,6,6].to_vec();
        let range2 = [4,5,6,4,5,6,4,5,6].to_vec();
        assertions(5,5,7,range1,range2, &mut puzzle, &mut constraint);
        
        // box 6 (middle right)
        let range1 = [4,4,4,5,5,5,6,6,6].to_vec();
        let range2 = [7,8,9,7,8,9,7,8,9].to_vec();
        assertions(5,9,6,range1,range2, &mut puzzle, &mut constraint);
        
        // box 7 (bottom left)
        let range1 = [7,7,7,8,8,8,9,9,9].to_vec();
        let range2 = [1,2,3,1,2,3,1,2,3].to_vec();
        assertions(7,3,6,range1,range2, &mut puzzle, &mut constraint);
        
        // box 8 (bottom middle)
        let range1 = [7,7,7,8,8,8,9,9,9].to_vec();
        let range2 = [4,5,6,4,5,6,4,5,6].to_vec();
        assertions(7,5,9,range1,range2, &mut puzzle, &mut constraint);
        
        // box 9 (bottom right)
        let range1 = [7,7,7,8,8,8,9,9,9].to_vec();
        let range2 = [7,8,9,7,8,9,7,8,9].to_vec();
        assertions(7,7,3,range1,range2, &mut puzzle, &mut constraint);
        
    }

    fn test_no_box_duplicates() {
        
        let sample_puzzle: Vec<String> =

        [
            "261      ",
            "385      ",
            "74       ",
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
        .collect();

        let mut puzzle = SudokuPuzzle::new(sample_puzzle);

        let mut constraint = NoBoxDuplicates::new();
        let mut constraint: Box<dyn Constraint> = Box::new(NoBoxDuplicates::new());

        // reminder: (1..10) goes from 1-9

        let (row, col) = (3,3);
        assert!(!puzzle.get_square(row,col).has_single_value());
        [1,1,1,2,2,2,3,3,3]
            .iter()
            .zip([1,2,3,1,2,3,1,2,3].iter())
            .map(|(i,j)| (*i as usize, *j as usize))
            .filter(|(i,j)| *i!=row || *j!=col)
            .for_each(|(i,j)| {
                constraint.process(&QueueItem{row:i, column:j}, &mut puzzle);
            });
        assert!(puzzle.get_square(row,col).has_single_value());
        assert!(puzzle.get_square(row,col).get_single_value() == 9);

    }

    #[test]
    fn box_iterators() {
        let actual = box_util::box8(5,5);
        assert_eq!(actual.len(), 8);
        let expected = [(4,4),(4,5),(4,6),(5,4),(5,6),(6,4),(6,5),(6,6)];
        (0..8).for_each(|i| assert_eq!(actual.get(i),expected.get(i)));
    }

    #[test]
    fn box_vectors() {
        {
            // upper left box
            let example = box_util::box9(1,1);
            let rest = vec![
                box_util::box9(1,2),
                box_util::box9(1,3),
                box_util::box9(2,1),
                box_util::box9(2,2),
                box_util::box9(2,3),
                box_util::box9(3,1),
                box_util::box9(3,2),
                box_util::box9(3,3),
            ];
            assert_eq!(example.len(), 9);
            rest.iter().for_each(|el| {
                assert_eq!(el.len(), 9);
                for i in (0..9) {
                    assert_eq!(example.get(i), el.get(i));
                }
            });
        }
        {
            // upper middle box
            let example = box_util::box9(1,4);
            let rest = vec![
                box_util::box9(1,5),
                box_util::box9(1,6),
                box_util::box9(2,4),
                box_util::box9(2,5),
                box_util::box9(2,6),
                box_util::box9(3,4),
                box_util::box9(3,5),
                box_util::box9(3,6),
            ];
            assert_eq!(example.len(), 9);
            rest.iter().for_each(|el| {
                assert_eq!(el.len(), 9);
                for i in (0..9) {
                    assert_eq!(example.get(i), el.get(i));
                }
            });
        }
        {
            // upper right box
            let example = box_util::box9(1,7);
            let rest = vec![
                box_util::box9(1,8),
                box_util::box9(1,9),
                box_util::box9(2,7),
                box_util::box9(2,8),
                box_util::box9(2,9),
                box_util::box9(3,7),
                box_util::box9(3,8),
                box_util::box9(3,9),
            ];
            assert_eq!(example.len(), 9);
            rest.iter().for_each(|el| {
                assert_eq!(el.len(), 9);
                for i in (0..9) {
                    assert_eq!(example.get(i), el.get(i));
                }
            });
        }
        {
            // middle left box
            let example = box_util::box9(4,1);
            let rest = vec![
                box_util::box9(4,2),
                box_util::box9(4,3),
                box_util::box9(5,1),
                box_util::box9(5,2),
                box_util::box9(5,3),
                box_util::box9(6,1),
                box_util::box9(6,2),
                box_util::box9(6,3),
            ];
            assert_eq!(example.len(), 9);
            rest.iter().for_each(|el| {
                assert_eq!(el.len(), 9);
                for i in (0..9) {
                    assert_eq!(example.get(i), el.get(i));
                }
            });
        }
        {
            // middle middle box
            let example = box_util::box9(4,4);
            let rest = vec![
                box_util::box9(4,5),
                box_util::box9(4,6),
                box_util::box9(5,4),
                box_util::box9(5,5),
                box_util::box9(5,6),
                box_util::box9(6,4),
                box_util::box9(6,5),
                box_util::box9(6,6),
            ];
            assert_eq!(example.len(), 9);
            rest.iter().for_each(|el| {
                assert_eq!(el.len(), 9);
                for i in (0..9) {
                    assert_eq!(example.get(i), el.get(i));
                }
            });
        }
        {
            // middle right box
            let example = box_util::box9(4,7);
            let rest = vec![
                box_util::box9(4,8),
                box_util::box9(4,9),
                box_util::box9(5,7),
                box_util::box9(5,8),
                box_util::box9(5,9),
                box_util::box9(6,7),
                box_util::box9(6,8),
                box_util::box9(6,9),
            ];
            assert_eq!(example.len(), 9);
            rest.iter().for_each(|el| {
                assert_eq!(el.len(), 9);
                for i in (0..9) {
                    assert_eq!(example.get(i), el.get(i));
                }
            });
        }
        {
            // bottom left box
            let example = box_util::box9(7,1);
            let rest = vec![
                box_util::box9(7,2),
                box_util::box9(7,3),
                box_util::box9(8,1),
                box_util::box9(8,2),
                box_util::box9(8,3),
                box_util::box9(9,1),
                box_util::box9(9,2),
                box_util::box9(9,3),
            ];
            assert_eq!(example.len(), 9);
            rest.iter().for_each(|el| {
                assert_eq!(el.len(), 9);
                for i in (0..9) {
                    assert_eq!(example.get(i), el.get(i));
                }
            });
        }
        {
            // bottom middle box
            let example = box_util::box9(7,4);
            let rest = vec![
                box_util::box9(7,5),
                box_util::box9(7,6),
                box_util::box9(8,4),
                box_util::box9(8,5),
                box_util::box9(8,6),
                box_util::box9(9,4),
                box_util::box9(9,5),
                box_util::box9(9,6),
            ];
            assert_eq!(example.len(), 9);
            rest.iter().for_each(|el| {
                assert_eq!(el.len(), 9);
                for i in (0..9) {
                    assert_eq!(example.get(i), el.get(i));
                }
            });
        }
        {
            // bottom right box
            let example = box_util::box9(7,7);
            let rest = vec![
                box_util::box9(7,8),
                box_util::box9(7,9),
                box_util::box9(8,7),
                box_util::box9(8,8),
                box_util::box9(8,9),
                box_util::box9(9,7),
                box_util::box9(9,8),
                box_util::box9(9,9),
            ];
            assert_eq!(example.len(), 9);
            rest.iter().for_each(|el| {
                assert_eq!(el.len(), 9);
                for i in (0..9) {
                    assert_eq!(example.get(i), el.get(i));
                }
            });
        }
    }

    #[test]
    fn box_index_to_box() {
        assert!(box_util::index_to_box(1) == 1);
        assert!(box_util::index_to_box(2) == 1);
        assert!(box_util::index_to_box(3) == 1);
        assert!(box_util::index_to_box(4) == 2);
        assert!(box_util::index_to_box(5) == 2);
        assert!(box_util::index_to_box(6) == 2);
        assert!(box_util::index_to_box(7) == 3);
        assert!(box_util::index_to_box(8) == 3);
        assert!(box_util::index_to_box(9) == 3);
    }

    #[test]
    fn puzzle_almost_solved_no_column_duplicates() {
        /*
-----------
|261943587|
|385721694|
|749586123|
|654218739|
|138479256|
|972365841|
|526194378|
|413857962|
|897632415|
-----------
         */
        let sample_puzzle: Vec<String> =

        [
            "261943587",
            "385721694",
            "749586123",
            "654218739",
            "138479256",
            "972365841",
            "526194378",
            "413857962",
            "         ",
        ]
        .iter()
        .cloned()
        .map(|s| s.to_string())
        .collect();

        let mut puzzle = SudokuPuzzle::new(sample_puzzle);

        let mut constraint = NoColumnDuplicates::new();
        let mut constraint: Box<dyn Constraint> = Box::new(NoColumnDuplicates::new());

        // reminder: (1..10) goes from 1-9

        (1..10)
            .for_each(|c| assert!(!puzzle.get_square(9,c).has_single_value()));

        // process the first column and test
        (1..9)
            .for_each(|r| { constraint.process(&QueueItem{row:r, column:1}, &mut puzzle); });
        (1..2)
            .for_each(|c| assert!(puzzle.get_square(9,c).has_single_value()));
        (2..10)
            .for_each(|c| assert!(!puzzle.get_square(9,c).has_single_value()));

        // second row - 9th row
        (2..10)
            .for_each(|column| {
                (1..9)
                    .for_each(|r| { constraint.process(&QueueItem{row:r, column:column}, &mut puzzle); });
                (1..column+1)
                    .for_each(|c| assert!(puzzle.get_square(9,c).has_single_value()));
                (column+1..10)
                    .for_each(|c| assert!(!puzzle.get_square(9,c).has_single_value()));
            });
        
        // test all the values now
        assert!(puzzle.get_square(9,1).get_single_value() == 8);
        assert!(puzzle.get_square(9,2).get_single_value() == 9);
        assert!(puzzle.get_square(9,3).get_single_value() == 7);
        assert!(puzzle.get_square(9,4).get_single_value() == 6);
        assert!(puzzle.get_square(9,5).get_single_value() == 3);
        assert!(puzzle.get_square(9,6).get_single_value() == 2);
        assert!(puzzle.get_square(9,7).get_single_value() == 4);
        assert!(puzzle.get_square(9,8).get_single_value() == 1);
        assert!(puzzle.get_square(9,9).get_single_value() == 5);
    }

    #[test]
    fn puzzle_almost_solved_no_row_duplicates() {
        /*
-----------
|261943587|
|385721694|
|749586123|
|654218739|
|138479256|
|972365841|
|526194378|
|413857962|
|897632415|
-----------
         */
        let sample_puzzle: Vec<String> =

        [
            "26194358 ",
            "38572169 ",
            "74958612 ",
            "65421873 ",
            "13847925 ",
            "97236584 ",
            "52619437 ",
            "41385796 ",
            "89763241 ",
        ]
        .iter()
        .cloned()
        .map(|s| s.to_string())
        .collect();

        let mut puzzle = SudokuPuzzle::new(sample_puzzle);

        let mut constraint = NoRowDuplicates::new();
        let mut constraint: Box<dyn Constraint> = Box::new(NoRowDuplicates::new());

        // reminder: (1..10) goes from 1-9

        (1..10)
            .for_each(|r| assert!(!puzzle.get_square(r,9).has_single_value()));

        // process the first row and test
        (1..9)
            .for_each(|c| { constraint.process(&QueueItem{row:1, column:c}, &mut puzzle); });
        (1..2)
            .for_each(|r| assert!(puzzle.get_square(r,9).has_single_value()));
        (2..10)
            .for_each(|r| assert!(!puzzle.get_square(r,9).has_single_value()));

        // second row - 9th row
        (2..10)
            .for_each(|row| {
                (1..9)
                    .for_each(|c| { constraint.process(&QueueItem{row:row, column:c}, &mut puzzle); });
                (1..row+1)
                    .for_each(|r| assert!(puzzle.get_square(r,9).has_single_value()));
                (row+1..10)
                    .for_each(|r| assert!(!puzzle.get_square(r,9).has_single_value()));
            });
        
        // test all the values now
        assert!(puzzle.get_square(1,9).get_single_value() == 7);
        assert!(puzzle.get_square(2,9).get_single_value() == 4);
        assert!(puzzle.get_square(3,9).get_single_value() == 3);
        assert!(puzzle.get_square(4,9).get_single_value() == 9);
        assert!(puzzle.get_square(5,9).get_single_value() == 6);
        assert!(puzzle.get_square(6,9).get_single_value() == 1);
        assert!(puzzle.get_square(7,9).get_single_value() == 8);
        assert!(puzzle.get_square(8,9).get_single_value() == 2);
        assert!(puzzle.get_square(9,9).get_single_value() == 5);
    }

    #[test]
    fn puzzle_does_not_throw_exception() {
        let sample_puzzle: Vec<String> =
            [
                "     35  ",
                "    2 6 4",
                "74     23",
                "  4 1    ",
                "13 4 9 56",
                "    6 8  ",
                "52     7 ",
                "4   5    ",
                "  76     ",
            ]
            .iter()
            .cloned()
            .map(|s| s.to_string())
            .collect();

        let mut puzzle = SudokuPuzzle::new(sample_puzzle);

        let mut constraint = NoRowDuplicates::new();
        let mut constraint: Box<dyn Constraint> = Box::new(NoRowDuplicates::new());

        constraint.process(&QueueItem{row:1, column: 1}, &mut puzzle);
    }

    #[test]
    fn always_pass() {
        assert!(true);
    }
}
