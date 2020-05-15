use crate::sudoku_puzzle::SudokuPuzzle;
use crate::guess_ordering::SimpleGuessOrderingByTuple;
use crate::constraint::box_util;
use crate::constraint::Constraint;
use crate::constraint::ConstraintViolation;
use crate::constraint::NoRowDuplicates;
use crate::constraint::NoColumnDuplicates;
use crate::constraint::NoBoxDuplicates;
use crate::constraint::ProcessOfElimination;
use crate::constraint::AllCannotBeEliminated;
use crate::constraint::DoubleDoubleRow;
use crate::constraint::DoubleDoubleColumn;
use crate::constraint::DoubleDoubleBox;
use crate::debug;
use crate::config;

use std::collections::HashMap;
use std::collections::VecDeque;

pub struct QueueItem {
    pub row: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct Progress {
    pub depth: usize,
    pub total_guesses: usize,
    pub max_depth_reached: usize,
}

impl Progress {
    pub fn new(depth: usize) -> Self {
        Progress {
            depth,
            total_guesses: 0,
            max_depth_reached: depth,
        }
    }
    pub fn clone(&self) -> Self {
        Progress {
            depth: self.depth,
            total_guesses: self.total_guesses,
            max_depth_reached: self.max_depth_reached,
        }
    }
}

pub struct Solver {
    // puzzle: &'a mut SudokuPuzzle,
    puzzle: SudokuPuzzle,
    guess_ordering: SimpleGuessOrderingByTuple,
    progress_list: Vec<Progress>,
    // queue_item_list: Vec<QueueItem>,
    queue_item_list: VecDeque<QueueItem>,
    constraint_queue: Vec<Box<dyn Constraint>>,
    guess: Option<(usize, usize)>, // during search, enqueue only the guess
    constraint_times: HashMap<String, u128>,
}

impl Solver {
    pub fn get_progress_list(&self) -> &Vec<Progress> {
        &self.progress_list
    }
    pub fn get_depth(&mut self) -> usize {
        self.get_progress().depth
    }
    pub fn get_progress(&mut self) -> &mut Progress {
        let last = self.progress_list.len()-1;
        &mut self.progress_list[last]
    }
    pub fn get_total_guesses(&self) -> usize {
        let last = self.progress_list.len()-1;
        self.progress_list[last].total_guesses
    }
    pub fn get_max_depth_reached(&self) -> usize {
        let last = self.progress_list.len()-1;
        self.progress_list[last].max_depth_reached
    }
    pub fn get_constraint_times_map(&self) -> HashMap<String, u128> {
        let mut map: HashMap<String, u128> = HashMap::new();
        for i in 0..self.constraint_queue.len() {
            let constraint = &self.constraint_queue[i];
            let name = constraint.name();
            let value = constraint.elapsed_millis() +
                self.constraint_times.get(&name).unwrap();
            map.insert(name, value);
            match constraint.elapsed_sub_millis() {
                Some(millis) => {
                    let name = "POESubMeasurement".to_string();
                    let value = millis + self.constraint_times.get(&name).unwrap();
                    map.insert(name, value);
                },
                None => (),
            };
        }
        // &self.constraint_times
        map
    }
    pub fn new(puzzle: SudokuPuzzle, guess_ordering: SimpleGuessOrderingByTuple) -> Self {
        let mut constraint_queue: Vec<Box<dyn Constraint>> = vec![];
        constraint_queue.push(Box::new(NoRowDuplicates::new()));
        constraint_queue.push(Box::new(NoColumnDuplicates::new()));
        constraint_queue.push(Box::new(NoBoxDuplicates::new()));
        constraint_queue.push(Box::new(ProcessOfElimination::new()));
        constraint_queue.push(Box::new(AllCannotBeEliminated::new()));
        constraint_queue.push(Box::new(DoubleDoubleRow::new()));
        constraint_queue.push(Box::new(DoubleDoubleColumn::new()));
        constraint_queue.push(Box::new(DoubleDoubleBox::new()));

        let mut child_map: HashMap<String, u128> = HashMap::new();
        constraint_queue.iter().for_each(|constraint| {
            child_map.insert(constraint.name(), 0);
            match constraint.elapsed_sub_millis() {
                Some(_) => {
                    child_map.insert("POESubMeasurement".to_string(), 0);
                    ()
                }
                None => (),
            };
            ()
        });

        Solver {
            puzzle: puzzle,
            guess_ordering,
            progress_list: vec![Progress::new(0)],
            queue_item_list: VecDeque::new(),
            constraint_queue,
            guess: None,
            constraint_times: child_map,
        }
    }
    pub fn with_guess(&mut self, row: usize, col: usize) {
        self.guess = Some((row, col));
    }
    pub fn with_progress<'a>(&'a mut self, progress_list: &Vec<Progress>) -> &'a mut Self {
        // chose not to build this out, as progress_list is not an Option but has 2 different values
        let mut cloned_progress_list = vec![];
        progress_list
            .iter()
            .for_each(|progress| { cloned_progress_list.push((*progress).clone())});
        let depth = progress_list.len();
        if depth > config::MAX_SEARCH_DEPTH {
            panic!("max depth reached");
        }
        cloned_progress_list.push(Progress::new(depth));

        self.progress_list = cloned_progress_list;
        
        self
    }
    
    pub fn get_puzzle(&mut self) -> &mut SudokuPuzzle {
        &mut self.puzzle
    }
    fn enqueue(&mut self, qitem: QueueItem) {
        self.queue_item_list.push_back(qitem);
    }
    fn clean_all(&mut self) {
        (1..=0).for_each(|r| (1..=9).for_each(|c| self.puzzle.get_square(r,c).clean()))
    }
    fn copy_from_solution(&mut self, possible_solution: &mut SudokuPuzzle) {
        debug::debug(format!("(depth={}) entering copy_from_solution!!!", self.get_depth()));
        for i in 1..=9 {
            for j in 1..=9 {
                let correct_value = possible_solution.get_square(i,j).get_single_value();
                self.puzzle.get_square(i,j).select(correct_value);
            }
        }
        match self.validate_solution() {
            Ok(_) => (),
            Err(e) => panic!("did not expect the possible solution to fail validation! (e={})", e.msg),
        }
    }
    fn validate_solution(&mut self) -> Result<bool, ConstraintViolation> {
        // validate all rows contain all values exactly once
        for r in 1..=9 {
            let mut seen = [0 as usize; 9];
            for c in 1..=9 {
                if self.get_puzzle().get_square(r,c).has_single_value() {
                    let value = self.get_puzzle().get_square(r,c).get_single_value();
                    seen[value-1] = seen[value-1] + 1;
                }
            }
            for i in 0.. seen.len() {
                if seen[i] != 1 {
                    return Err(ConstraintViolation{msg: "rows must contain all values once".to_string()});
                }
            }
        }
        // validate all columns contain all values exactly once
        for c in 1..=9 {
            let mut seen = [0 as usize; 9];
            for r in 1..=9 {
                if self.get_puzzle().get_square(r,c).has_single_value() {
                    let value = self.get_puzzle().get_square(r,c).get_single_value();
                    seen[value-1] = seen[value-1] + 1;
                }
            }
            for i in 0.. seen.len() {
                if seen[i] != 1 {
                    return Err(ConstraintViolation{msg: "columns must contain all values once".to_string()});
                }
            }
        }
        // validate all boxes contain all values exactly once
        let mut error_seen: Option<Result<bool, ConstraintViolation>> = None;
        [(1,1),(1,4),(1,7),(4,1),(4,4),(4,7),(7,1),(7,4),(7,7),]
            .iter()
            .for_each(|(r,c)| {
                let mut seen = [0 as usize; 9];
                box_util::box9(*r,*c).iter().for_each(|(i,j)| {
                    if self.get_puzzle().get_square(*i,*j).has_single_value() {
                        let value = self.get_puzzle().get_square(*i,*j).get_single_value();
                        seen[value-1] = seen[value-1] + 1;
                    }
                });
                for i in 0.. seen.len() {
                    if seen[i] != 1 {
                        error_seen = Some(Err(ConstraintViolation{msg: "boxes must contain all values once".to_string()}));
                    }
                }
            });

        match error_seen {
            Some(e) => e,
            None => Ok(true),
        }
    }
    fn done_yet(&mut self) -> Result<bool, ConstraintViolation> {
        for i in 1..=9 {
            for j in 1..=9 {
                if !self.get_puzzle().get_square(i,j).has_single_value() {
                    return Ok(false);
                }
            }
        }

        self.validate_solution()
    }
    fn enqueue_all_dirty_and_mark_clean(&mut self) -> () {
        (1..=9).for_each(|r| (1..=9).for_each(|c| {
            if self.puzzle.get_square(r,c).is_dirty() {
                self.enqueue(QueueItem{row: r, column: c});
                self.puzzle.get_square(r,c).clean();
            }
        }));
    }
    fn process(&mut self, qitem: QueueItem) -> Result<(), ConstraintViolation> {
        for i in 0..self.constraint_queue.len() {
            // self.constraint_queue[i].process(&qitem, &mut self.puzzle)?;
            let constraint = &mut self.constraint_queue[i];
            constraint.process(&qitem, &mut self.puzzle)?;
        }
        Ok(())
    }
    pub fn solve(&mut self) -> Result<(), ConstraintViolation> {
        //
        // improvements:
        //   1. do not need _all_ guesses, really only need guesses for one node
        //   2. given the above, "randomize guesses" can refer to one or both of:
        //      2.1 given an ordering, which node should I pick with an order?
        //      2.2 given a node, which guess should come first?
        //   3. given a new solver, only enqueue the recently selected node
        //

        match self.guess {
            Some((row, col)) => {
                self.enqueue(QueueItem{row: row, column: col});
            },
            None => {
                // enqueue all InitialSquare objects
                (1..=9).for_each(|r| (1..=9).for_each(|c| {
                    if self.puzzle.get_square(r,c).has_single_value() {
                        self.enqueue(QueueItem{row: r, column: c})
                    }
                }));
            }
        };

        self.clean_all();

        let max_runs = 30;
        'outer: for _run_number in 0..max_runs {
            if self.queue_item_list.len() > 0 {
                debug::debug(format!("(depth={}) running through queue of length {}", self.get_depth(), self.queue_item_list.len()));
                // run through queue completely
                'inner: while !self.queue_item_list.is_empty() {
                    let qitem = self.queue_item_list.pop_front().unwrap();
                    let result = self.process(qitem);
                    match result {
                        Ok(_) => (),
                        Err(e) => {
                            return Err(e);
                        }
                    };
                    // check if finished after every constraint has processed
                    match self.done_yet() {
                        Ok(is_done) => {
                            if is_done {
                                break 'outer;
                            }
                        }
                        Err(constraint_violation) => {
                            return Err(constraint_violation);
                        }
                    }
                }
                debug::debug(format!("(depth={}) finished running through queue!", self.get_depth()));
                self.enqueue_all_dirty_and_mark_clean();
            } else {
                // search
                let guess_list = self.guess_ordering.clone().guess_list(self.get_puzzle());
                debug::debug(format!("(depth={}) guess list: {:?}", self.get_depth(), guess_list));
                // note there should be many correct guesses - one per node!

                // not sure, but we might not need this given that contradictions are searched for
                let correct_guess: Option<(usize, usize, usize)> = None;

                for index in 0..guess_list.len() {
                    // preparation
                    let guess: &(usize, usize, usize) = &guess_list[index];
                    let i = guess.0;
                    let j = guess.1;
                    let value = guess.2;
                    debug::debug(format!("(depth={}) guess: (i,j,value)=({},{},{})", self.get_depth(), i, j, value));
                    // clone the puzzle -- this also copies square state
                    let mut puzzle_clone = SudokuPuzzle::clone_from(self.get_puzzle());
                    // apply the guess
                    puzzle_clone.get_square(i,j).select(value);
                    self.get_progress().total_guesses = self.get_progress().total_guesses + 1;
                    // create the new solver
                    let mut new_solver: Solver = Solver::new(puzzle_clone, self.guess_ordering.clone());
                    new_solver.with_progress(self.get_progress_list());
                    new_solver.with_guess(i,j);
                    // and solve
                    match new_solver.solve() {
                        Ok(_) => {
                            debug::debug(format!("(depth={}) guess ({},{},{}) was correct!", self.get_depth(), i, j, value));
                            // the guess was correct
                            // copy state from that puzzle to this puzzle
                            // (much faster than applying the correct guess)
                            self.copy_from_solution(new_solver.get_puzzle());

                            // bookkeeping
                            self.get_progress().total_guesses = self.get_progress().total_guesses +
                                new_solver.get_total_guesses();
                            let new_max_depth = new_solver.get_max_depth_reached();
                            let current_max_depth = self.get_progress().max_depth_reached;
                            if new_max_depth > current_max_depth {
                                self.get_progress().max_depth_reached = new_max_depth;
                            }
                            let delta_constraint_times = new_solver.get_constraint_times_map();
                            for i in 0..self.constraint_queue.len() {
                                let constraint = &self.constraint_queue[i];
                                let name = constraint.name();
                                let new_value = delta_constraint_times.get(&name).unwrap() +
                                    self.constraint_times.get(&name).unwrap();
                                self.constraint_times.insert(name, new_value);
                                match constraint.elapsed_sub_millis() {
                                    Some(_) => {
                                        let name = "POESubMeasurement".to_string();
                                        let value = delta_constraint_times.get(&name).unwrap() +
                                            self.constraint_times.get(&name).unwrap();
                                        self.constraint_times.insert(name, value);
                                    },
                                    None => (),
                                }
                            }

                            return Ok(())
                        }
                        Err(e) => {
                            debug::debug(format!("(depth={}) guess ({},{},{}) was incorrect, eliminating {} (e={})", self.get_depth(), i,j,value,value, e.msg));
                            self.puzzle.get_square(i,j).eliminate(value);

                            // bookkeeping
                            self.get_progress().total_guesses = self.get_progress().total_guesses +
                                new_solver.get_total_guesses();
                            let new_max_depth = new_solver.get_max_depth_reached();
                            let current_max_depth = self.get_progress().max_depth_reached;
                            if new_max_depth > current_max_depth {
                                self.get_progress().max_depth_reached = new_max_depth;
                            }
                            let delta_constraint_times = new_solver.get_constraint_times_map();
                            for i in 0..self.constraint_queue.len() {
                                let constraint = &self.constraint_queue[i];
                                let name = constraint.name();
                                let new_value = delta_constraint_times.get(&name).unwrap() +
                                    self.constraint_times.get(&name).unwrap();
                                self.constraint_times.insert(name, new_value);
                                match constraint.elapsed_sub_millis() {
                                    Some(_) => {
                                        let name = "POESubMeasurement".to_string();
                                        let value = delta_constraint_times.get(&name).unwrap() +
                                            self.constraint_times.get(&name).unwrap();
                                        self.constraint_times.insert(name, value);
                                    },
                                    None => (),
                                }
                            }

                            // if a contradiction has been reached, do not keep guessing blindly
                            if self.puzzle.get_square(i,j).count_remaining() == 0 {
                                debug::debug(format!("(depth={}) all values eliminated for ({},{})", self.get_depth(), i, j));
                                let msg = format!("all values eliminated for a square, ({},{}), depth={}", i,j, self.get_depth());
                                return Err(ConstraintViolation{msg: msg});
                            }
                            continue;
                        }
                    }
                }

                if correct_guess == None {
                    return Err(ConstraintViolation{msg: "no correct guesses".to_string()});
                }

            }

        };
        Ok(())
    }

}

#[cfg(test)]
mod tests {

    use crate::sudoku_puzzle::SudokuPuzzle;
    use crate::guess_ordering;
    use super::*;

    #[test]
      fn create_solver2() {
          let s = Solver::new(get_sample_puzzle(), get_ordering());
          assert_eq!(s.get_progress_list().len(), 1);
          [(0,0)]
              .iter().for_each(|(i,expected_depth)| {
                  assert_eq!(s.get_progress_list()[*i].depth, *expected_depth);
              });

          let mut s2 = Solver::new(get_sample_puzzle(), get_ordering());
          s2.with_progress(s.get_progress_list());
          // re-assert s
          assert_eq!(s.get_progress_list().len(), 1);
          [(0,0)]
              .iter().for_each(|(i,expected_depth)| {
                  assert_eq!(s.get_progress_list()[*i].depth, *expected_depth);
              });
          // and now s2
          assert_eq!(s2.get_progress_list().len(), 2);
          [(0,0),(1,1)]
              .iter().for_each(|(i,expected_depth)| {
                  assert_eq!(s2.get_progress_list()[*i].depth, *expected_depth);
              });

          let mut s3 = Solver::new(get_sample_puzzle(), get_ordering());
          s3.with_progress(s2.get_progress_list());
          // re-assert s
          assert_eq!(s.get_progress_list().len(), 1);
          [(0,0)]
              .iter().for_each(|(i,expected_depth)| {
                  assert_eq!(s.get_progress_list()[*i].depth, *expected_depth);
              });
          // also s2
          assert_eq!(s2.get_progress_list().len(), 2);
          [(0,0),(1,1)]
              .iter().for_each(|(i,expected_depth)| {
                  assert_eq!(s2.get_progress_list()[*i].depth, *expected_depth);
              });
          // also s3
          assert_eq!(s3.get_progress_list().len(), 3);
          [(0,0),(1,1),(2,2)]
              .iter().for_each(|(i,expected_depth)| {
                  assert_eq!(s3.get_progress_list()[*i].depth, *expected_depth);
              });
          
      }

    #[test]
    fn test_progress() {
        let mut p: Progress = Progress::new(0);
        assert_eq!(p.depth, 0);
        assert_eq!(p.total_guesses, 0);

        let p2: Progress = p.clone();
        assert_eq!(p2.depth, 0);
        assert_eq!(p2.total_guesses, 0);

        p.total_guesses = 1;
        assert_eq!(p.total_guesses, 1);
        assert_eq!(p2.total_guesses, 0);
        
    }

    #[test]
    fn create_solver() {
        let mut s = Solver::new(get_sample_puzzle(), get_ordering());
    }

    #[test]
    fn test_vectors() {
        let v = vec![1,2,3,4,5];
        assert_eq!(v[1], 2);
    }

    #[test]
    fn always_pass() {
        assert!(true);
    }

    fn get_ordering() -> guess_ordering::SimpleGuessOrderingByTuple {
        guess_ordering::SimpleGuessOrderingByTuple::new((4,3,2,5,6,7,8,9))
    }

    fn get_sample_puzzle() -> SudokuPuzzle {
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

        SudokuPuzzle::new(sample_puzzle)
    }

}
