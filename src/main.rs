mod squares;
mod guess_ordering;
mod solver;
mod sudoku_puzzle;
mod constraint;
mod test;
mod debug;
mod sample_puzzles;
mod config;
mod stopwatch;

use std::collections::HashMap;
use std::io;
use sample_puzzles::Puzzle;
use std::io::Write;
use stopwatch::Stopwatch;

#[derive(Clone)]
enum Mode {
    Search,
    RunWithoutSearch,
    ProfileWorldsHardest,
}
/*

to profile successfully:
  - add stopwatch to all constraints
  - solver collects elapsed data for constraints
  - solver adds to this whatever returns from sub-solvers

*/
/*

Conclusions:
  - OrderingStrategy can be passed an immutable reference to solver
  - 

Q: If a struct Foo contains a String name field (with getter and setter), and
   also contains a Bar bar field (with getter only), then can multiple references
   be out to a Foo object? Presumably yes - and those can all [obviously] be
   used to access Foo::get_name(&self). But what about foo.get_bar()? Still yes
   But if the returned item is not mutable, how would you handle a case where
   you want multiple references out to Foo, all of which should be able to
   make a call to foo.get_bar().some_modification()?
   Is there such a thing as get_bar_ref()?

So, taking this further, if a square object knows its own state, then making a
call to square.select() or square.eliminate() is a far different thing than
a call to square.values_remaining() or square.count_remaining(), which is
required by GuessOrdering.

So the next step is to build additional objects, and test them, with a better
object model. Basically Square is done, perhaps Constraint should be next?

*/

fn profile_strategy() {
    //
    // ProcessOfElimination takes a TON of time (but reduces guesses):
    //   e.g. 480ms w/ 100 guesses vs 229ms w/ 365 guesses
    //        466ms w/ 99 guesses vs 260ms w/ 423 guesses
    //
    // So either speed it up or remove it
    //   - memoization
    //   - do not propagate if self.is_initial_square()
    //   - analyze algorithm runtime
    //     - any data structure improvements?
    //       - keep track of last clean bitmap? (i.e. values eliminated)
    //     - any algorithm improvements?
    //
    // strategy, trials: avg time, avg guesses, avg max depth
    //
    // w/o POE
    // 2,3,4,... w/o POE, 100x: 170ms, 263 guesses, 13 depth
    // 2,4,3,... w/o POE, 100x: 184ms, 288 guesses, 13 depth
    // 2,3,4,... w/o POE, 100x: 171ms, 268 guesses, 13 depth
    // 2,4,3,... w/o POE, 100x: 163ms, 257 guesses, 13 depth
    //
    // w/ POE
    // 2,3,4,... w/ POE, 100x: 517ms, 101 guesses, 10 depth
    // 2,4,3,... w/ POE, 100x: 530ms, 108 guesses, 10 depth
    // 2,3,4,... w/ POE, 100x: 565ms, 112 guesses, 10 depth
    // 2,4,3,... w/ POE, 100x: 491ms, 96 guesses, 9 depth
    //
    // implemented memoization (POE+)
    // 2,3,4,... w/ POE+, 100x: 402ms, 102 guesses, 10 depth
    // 2,4,3,... w/ POE+, 100x: 402ms, 101 guesses, 10 depth
    // 2,3,4,... w/ POE+, 100x: 408ms, 103 guesses, 10 depth
    // 2,4,3,... w/ POE+, 100x: 398ms, 101 guesses, 10 depth
    //
    // profiled POE and filter out values set exactly once
    // "only because a square has it already selected"
    // 2,3,4,... w/ POE++, 100x: 234ms, 103 guesses, 10 depth
    // 2,4,3,... w/ POE++, 100x: 234ms, 103 guesses, 10 depth
    // 2,3,4,... w/ POE++, 100x: 245ms, 93 guesses, 9 depth
    // 2,4,3,... w/ POE++, 100x: 229ms, 102 guesses, 10 depth
    //
    // optimize POE filtering, avoid mutable vectors
    // 2,3,4,... w/ POE+++, 100x: 191ms, 95 guesses, 10 depth
    // 2,4,3,... w/ POE+++, 100x: 209ms, 95 guesses, 10 depth
    // 2,3,4,... w/ POE+++, 100x: 199ms, 101 guesses, 10 depth
    // 2,4,3,... w/ POE+++, 100x: 195ms, 101 guesses, 10 depth
    // 2,3,4,... w/ POE+++, 100x: 263ms, 94 guesses, 10 depth
    // 2,4,3,... w/ POE+++, 100x: 182ms, 91 guesses, 10 depth
    //
    debug::debug("entering profile_strategy".to_string());

    let number_of_trials: usize = 100;
    let ordering_strategies = vec![
        guess_ordering::SimpleGuessOrderingByTuple::new((2,3,4,5,6,7,8,9)),
        guess_ordering::SimpleGuessOrderingByTuple::new((2,4,3,5,6,7,8,9)),
        //guess_ordering::SimpleGuessOrderingByTuple::new((3,2,4,5,6,7,8,9)),
        //guess_ordering::SimpleGuessOrderingByTuple::new((4,3,2,5,6,7,8,9)),
        //guess_ordering::SimpleGuessOrderingByTuple::new((5,4,3,2,6,7,8,9)),
    ];

    for ordering_strategy in ordering_strategies {
        // let ordering_strategy = guess_ordering::SimpleGuessOrderingByTuple::new((4,3,2,5,6,7,8,9));
        // let mut elapsed_time_subtotal = 0;
        let mut guesses_per_trial: Vec<usize> = vec![];
        let mut elapsed_time_per_trial: Vec<u128> = vec![];
        let mut max_depth_per_trial: Vec<usize> = vec![];
        let mut constraint_times_map: HashMap<String, u128> = HashMap::new();

        let p = match sample_puzzles::get_by_id("worldshardest") {
            Some(puzzle) => puzzle,
            None => panic!("invalid id!"),
        };

        for trial in 0..number_of_trials {
            let sudoku_puzzle = sudoku_puzzle::SudokuPuzzle::new(p.puzzle.clone());
            let mut s = solver::Solver::new(sudoku_puzzle, ordering_strategy.clone());
            // let time_start = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards");
            let mut stopwatch = Stopwatch::new();
            stopwatch.start();
            match s.solve() {
                Ok(_) => {
                    // println!("max depth: {}", s.get_max_depth_reached());
                    guesses_per_trial.push(s.get_total_guesses());
                    max_depth_per_trial.push(s.get_max_depth_reached());
                    if constraint_times_map.is_empty() {
                        constraint_times_map = s.get_constraint_times_map();
                    } else {
                        let solver_map = s.get_constraint_times_map();
                        for key in solver_map.keys() {
                            let old_value = constraint_times_map.get(key).unwrap();
                            let new_value = solver_map.get(key).unwrap();
                            let new_value_sum = old_value + new_value;
                            if key == "POESubMeasurement" {
                                //println!("poe-sub-measurement: {}", new_value);
                            }
                            if key == "ProcessOfEliminationWithMemoization" {
                                //println!("poe: {}", new_value);
                            }
                            constraint_times_map.insert(key.to_string(), new_value_sum);
                        }
                    }
                    ()
                }
                Err(e) => panic!("puzzle should not have errored: {}", e.msg),
            }
            if trial == number_of_trials-1 {
                println!("");
                std::io::stdout().flush().unwrap();
            } else {
                print!(".");
                std::io::stdout().flush().unwrap();
            }
            stopwatch.stop();
            let elapsed_time_millis = stopwatch.elapsed_millis();
            elapsed_time_per_trial.push(elapsed_time_millis);
        }

        println!("ordering strategy: {}", ordering_strategy.name());
        let elapsed_time_total: u128 = elapsed_time_per_trial.iter().fold(0u128, |sum, val| sum + val);
        let elapsed_time_avg: u128 = elapsed_time_total / number_of_trials as u128;
        let guesses_avg: usize = guesses_per_trial.iter().fold(0usize, |sum, val| sum + val) / number_of_trials;
        let max_depth_reached_avg: usize =
            max_depth_per_trial.iter().fold(0usize, |sum, val| sum + val) / number_of_trials;
        let mut elapsed_time_total_constraints = 0u128;
        for (k,v) in constraint_times_map {
            elapsed_time_total_constraints += v;
            println!("{}: {}ms", k, v);
        }
        println!("total elapsed times: {} (per clock), {} (per constraints)",
                 elapsed_time_total,
                 elapsed_time_total_constraints);
    
        println!("avg time: {}ms, avg guesses: {}, avg max depth: {} ({} trials)", elapsed_time_avg, guesses_avg, max_depth_reached_avg, number_of_trials);
        // println!("number of trials: {}", number_of_trials);
        // println!("total elapsed time: {}", elapsed_time_subtotal);

    }
}

fn run_one_puzzle_with_search(puzzle: &Puzzle) {
    debug::debug("entering run_one_puzzle_with_search".to_string());

    let sudoku_puzzle = sudoku_puzzle::SudokuPuzzle::new(puzzle.puzzle.clone());

    println!("{}", puzzle.name);

    sudoku_puzzle.debug_print_puzzle();

    let ordering_strategy = guess_ordering::SimpleGuessOrderingByTuple::new((4,3,2,5,6,7,8,9));
    let mut s = solver::Solver::new(sudoku_puzzle, ordering_strategy);
    // s.with_progress(vec![]).with_progress(vec![]);

    //let time_reference = SystemTime::now();
    // let time_start = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();

    let result = s.solve();

    match result {
        Ok(_) => println!("puzzle returned OK"),
        Err(e) => println!("puzzle returned with error: {}", e.msg),
    }

    // let time_end = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards again");
    stopwatch.stop();
    // let elapsed_time = (time_end - time_start).as_millis();
    let elapsed_time = stopwatch.elapsed_millis();
    println!("Profiling info: total elapsed time={}ms", elapsed_time);

    let solved_puzzle = s.get_puzzle();
    solved_puzzle.print_puzzle();
}

fn main() {
    println!("Welcome to SudokuSolver! Please make mode selection");

    let sample_puzzle = match sample_puzzles::get_by_id(config::SAMPLE_PUZZLE_ID) {
        Some(puzzle) => puzzle,
        None => panic!("invalid id supplied, exiting..."),
    };

    let puzzle_name = &sample_puzzle.name;

    println!("");
    println!("selected sudoku: {}", puzzle_name);
    println!("  1) solve sudoku");
    println!("  2) run solver with search disabled (under construction)");
    println!(r##"  3) run profiler against World's hardest Sudoku (under construction)"##);

    loop {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Failed to read line");

        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //test_function_pointer(choice);

        let choice_map: HashMap<usize, Mode> =
            [
                (1, Mode::Search),
                (2, Mode::RunWithoutSearch),
                (3, Mode::ProfileWorldsHardest),
            ]
            .iter()
            .cloned()
            .collect();

        match choice_map.get(&choice) {
            Some(mode) => {
                match mode {
                    Mode::Search => run_one_puzzle_with_search(&sample_puzzle),
                    Mode::RunWithoutSearch => println!("run without search not implemented"),
                    Mode::ProfileWorldsHardest => profile_strategy(),
                }
            },
            None => continue,
        }

        break;
    }
}
