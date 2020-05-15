use crate::sudoku_puzzle::SudokuPuzzle;
use crate::config;
use rand::Rng;

pub struct SimpleGuessOrderingByTuple {
    // ordering_tuple: (usize, usize, usize, usize, usize, usize, usize, usize),
    // ordering_array: [usize],
    ordering_vector: Vec<usize>,
}

impl SimpleGuessOrderingByTuple {
    pub fn new(ordering_tuple: (usize, usize, usize, usize, usize, usize, usize, usize)) -> Self {
        let ordering_vector = vec![
            ordering_tuple.0,
            ordering_tuple.1,
            ordering_tuple.2,
            ordering_tuple.3,
            ordering_tuple.4,
            ordering_tuple.5,
            ordering_tuple.6,
            ordering_tuple.7,
        ];
        SimpleGuessOrderingByTuple {
            ordering_vector
        }
    }
    pub fn clone(&self) -> Self {
        let mut ordering_vector = vec![];
        self.ordering_vector
            .iter().for_each(|el| {
                ordering_vector.push(*el);
            });
        
        SimpleGuessOrderingByTuple {
            ordering_vector
        }
    }
    pub fn name(&self) -> String {
        let v = &self.ordering_vector;
        format!("Simple ({},{},{},{},{},{},{},{})", v[0],v[1],v[2],v[3],v[4],v[5],v[6],v[7])
    }
    pub fn guess_list(&self, puzzle: &mut SudokuPuzzle) -> Vec<(usize, usize, usize)> {
        // returns guesses for a single node
        let mut guess_list = vec![];

        // find qualifying nodes
        let mut qualifying_nodes: Vec<(usize, usize)> = vec![];
        for degree in &self.ordering_vector {
            for i in 1..=9 {
                for j in 1..=9 {
                    let square = puzzle.get_square(i,j);
                    if square.count_remaining() == *degree {
                        qualifying_nodes.push((i,j));
                    }
                }
            }

            if qualifying_nodes.len() != 0 {
                break;
            }
        }

        if qualifying_nodes.len() > 0 {
            let mut chosen_node = &qualifying_nodes[0];
            let mut rng = rand::thread_rng();
            
            if config::RANDOMIZE_GUESSLIST {
                let chosen_node_index = rng.gen_range(0, qualifying_nodes.len());
                chosen_node = &qualifying_nodes[chosen_node_index];
            }

            let row = chosen_node.0;
            let col = chosen_node.1;
            let square = puzzle.get_square(row,col);

            if config::RANDOMIZE_GUESSLIST {
                for value in SimpleGuessOrderingByTuple::randomize_values(&square.values_remaining()) {
                    guess_list.push((row, col, value));
                }
                // let mut values_remaining_randomized: Vec<usize> = 
                // let mut values_remaining: Vec<usize> = square.values_remaining();
                // while values_remaining.len() > 0 {
                //     let index = rng.gen_range(0, values_remaining.len());
                //     let value = values_remaining.remove(index);
                //     guess_list.push((row, col, value));
                // }
            } else {
                for value in square.values_remaining() {
                    guess_list.push((row, col, value));
                }
            }
            
        }

        guess_list
    }

    pub fn randomize_values(number_list: &Vec<usize>) -> Vec<usize> {
        let mut rng = rand::thread_rng();
        let mut randomized: Vec<usize> = vec![];

        let mut number_list_copy = number_list.clone();

        while number_list_copy.len() > 0 {
            let index = rng.gen_range(0, number_list_copy.len());
            let element = number_list_copy.remove(index);
            randomized.push(element);
        }

        randomized
    }

    fn _guess_list_deprecated(&self, puzzle: &mut SudokuPuzzle) -> Vec<(usize, usize, usize)> {
        // returns guesses for a single node
        let mut guess_list = vec![];

        for degree in &self.ordering_vector {
            let mut single_degree_guess_list: Vec<(usize, usize, usize)> = vec![];
            for i in 1..=9 {
                for j in 1..=9 {
                    let square = puzzle.get_square(i,j);
                    if square.count_remaining() == *degree {
                        for value in square.values_remaining() {
                            single_degree_guess_list.push((i,j,value));
                        }
                    }
                }
            }
            if config::RANDOMIZE_GUESSLIST {
                panic!("not implemented");
            }
            guess_list.append(&mut single_degree_guess_list);
        }

        guess_list
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn always_pass() {
        assert!(true);
    }

    #[test]
    fn test_randomize_values() {
        let num_list = vec![0,1,2,3,4,5];

        let num_list_randomized = SimpleGuessOrderingByTuple::randomize_values(&num_list);

        (0..=5).for_each(|value| {
            assert!(num_list.contains(&value));
            assert!(num_list_randomized.contains(&value));
        });
    }
}
