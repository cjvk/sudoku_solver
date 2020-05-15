use std::collections::HashMap;

pub struct Puzzle {
    pub id: String,
    pub name: String,
    pub puzzle: Vec<String>,
}

#[derive(Clone)]
struct PuzzleTuple(String, String, Vec<String>);

pub fn get_by_id(id: &str) -> Option<Puzzle> {
    let sample_puzzles: Vec<PuzzleTuple> =
        [
            PuzzleTuple(
                "sjm20200425".to_string(),
                "Mercury News, 4/25/2020, difficulty: 2/4".to_string(),
                [
                    "7   12 89",
                    "  8 57   ",
                    "  1 6    ",
                    "8      91",
                    "      6  ",
                    "12      4",
                    "    2 7  ",
                    "   64 2  ",
                    "43 17   6",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "from_a_20200126_testing".to_string(),
                "Same but remove the 6 in row 8 col 7 - see if no search deduces".to_string(),
                [
                    "69 14    ",
                    " 156 3429",
                    " 34    61",
                    " 46     2",
                    "128936745",
                    " 73  4  6",
                    "46 3 12  ",
                    " 8 4    3",
                    "35 76 9 4",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "from_a_20200126".to_string(),
                "Sudoku.com text from A, 1/26/2020, difficulty: Hard".to_string(),
                [
                    "69 14    ",
                    " 156 3429",
                    " 34    61",
                    " 46     2",
                    "128936745",
                    " 73  4  6",
                    "46 3 12  ",
                    " 8 4  6 3",
                    "35 76 9 4",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "sjm20190118".to_string(),
                "Mercury News, 1/18/2019, difficulty: 4/4".to_string(),
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
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "sjm20160310".to_string(),
                "Mercury News, 3/10/2016, difficulty: 3/4".to_string(),
                [
                    "   164   ",
                    "43  7  1 ",
                    "1    9   ",
                    " 2     57",
                    "  54 73  ",
                    "31     8 ",
                    "   6    2",
                    " 8     45",
                    "   945   ",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "worldshardest".to_string(),
                "http://www.telegraph.co.uk/news/science/science-news/9359579/Worlds-hardest-sudoku-can-you-crack-it.html".to_string(),
                [
                    "8        ",
                    "  36     ",
                    " 7  9 2  ",
                    " 5   7   ",
                    "    457  ",
                    "   1   3 ",
                    "  1    68",
                    "  85   1 ",
                    " 9    4  ",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "sjm20160304".to_string(),
                "Mercury News, 3/4/2016, difficulty: 4/4".to_string(),
                [
                    "27   1 46",
                    " 5 2   3 ",
                    "   3 6   ",
                    "6       7",
                    " 4     2 ",
                    "9       3",
                    "   19    ",
                    " 2   5 7 ",
                    "7  6   8 ",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "7sudokuvd1".to_string(),
                "http://www.7sudoku.com/very-difficult, topleft".to_string(),
                [
                    "2     63 ",
                    " 1   3   ",
                    "  5  7  9",
                    " 4 7   6 ",
                    "  38 91  ",
                    " 2   5 4 ",
                    "1  9  4  ",
                    "   5   9 ",
                    " 56     7",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "ss20160302h".to_string(),
                "sudokusaviour, 3/2/2016, hard".to_string(),
                [
                    "  92    5",
                    "  4 8   7",
                    "7    34  ",
                    " 3  9 12 ",
                    "   5 1   ",
                    " 48 6  5 ",
                    "  14    3",
                    "4   5 9  ",
                    "6    85  ",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "ss20160303h".to_string(),
                "sudokusaviour, 3/3/2016, hard".to_string(),
                [
                    " 9   5 27",
                    "         ",
                    " 439  8  ",
                    "5 84  26 ",
                    "   5 3   ",
                    " 24  65 9",
                    "  5  194 ",
                    "         ",
                    "41 2   5 ",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "ss20160301e".to_string(),
                "sudokusaviour, 3/1/2016, easy".to_string(),
                [
                    "     6258",
                    "    7 1  ",
                    "  5 4   9",
                    "   4    3",
                    "5 6 8 7 1",
                    "9    7   ",
                    "8   9 4  ",
                    "  2 5    ",
                    "7692     ",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "ss20160302e".to_string(),
                "sudokusaviour, 3/2/2016, easy".to_string(),
                [
                    "1 26 7   ",
                    " 739 4   ",
                    "4   3    ",
                    " 19      ",
                    "3 4   6 9",
                    "      28 ",
                    "    9   1",
                    "   2 637 ",
                    "   1 39 6",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "sjm20160302".to_string(),
                "Mercury News, 3/2/2016, difficulty: 2/4".to_string(),
                [
                    "    3    ",
                    "  6  58 2",
                    "9    7   ",
                    " 2   1  9",
                    " 3  5  2 ",
                    "7  6   3 ",
                    "   9    1",
                    "  251 6 8",
                    "    4    ",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "sjm20160229".to_string(),
                "Mercury News, 2/29/2016, difficulty: 1/4".to_string(),
                [
                    "3  2    4",
                    "   5 37  ",
                    "    6 5  ",
                    " 12  93  ",
                    "   182   ",
                    "  67  18 ",
                    "  8 9    ",
                    "  93 1   ",
                    "5    7  6",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
        ]
        .iter().cloned().collect();

    let puzzle_tuples_by_id: HashMap<String, PuzzleTuple> = sample_puzzles
        .iter()
        .cloned()
        .map(|pt| (pt.0.clone(), pt))
        .collect();
        

    /*
    sample_puzzle_map.insert(
        "".to_string(),
        [
        ].iter().cloned().map(|s| s.to_string()).collect());

    let mut sample_puzzle_map: HashMap<String, PuzzleTuple> = HashMap::new();

    let puzzle_tuples_by_id: HashMap<String, PuzzleTuple> =
        [
            (
                String::from("id"),
                PuzzleTuple("id".to_string(), "name".to_string(), vec![]),
            ),
        ].iter().cloned()//.map(|(k,v)| (*k,v)).collect();
        .collect();
*/
    match puzzle_tuples_by_id.get(id) {
        Some(pt) => {
            Some(Puzzle {
                id: pt.0.clone(),
                name: pt.1.clone(),
                puzzle: pt.2.clone(),
            })
        }
        None => None
    }

}

pub fn _get_by_id_test(id: &str) -> Option<Puzzle> {
    let test_puzzles: Vec<PuzzleTuple> =
        [
            PuzzleTuple(
                "completely_solved".to_string(),
                "completely solved puzzle (corner case)".to_string(),
                [
                    "946287351",
                    "583619742",
                    "217543698",
                    "865432179",
                    "721965483",
                    "439178526",
                    "678324915",
                    "394851267",
                    "152796834",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "nearly_solved".to_string(),
                "nearly solved puzzle (1 away in all rows)".to_string(),
                [
                    "94628735 ",
                    "5836 9742",
                    "2 7543698",
                    "865432 79",
                    "72 965483",
                    "439 78526",
                    "6783249 5",
                    "39485 267",
                    " 52796834",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "poe_test".to_string(),
                "testing POE - but not sure how well this works".to_string(),
                [
                    "1        ",
                    "   1     ",
                    "      1  ",
                    " 1       ",
                    "    1    ",
                    "       1 ",
                    "  1      ",
                    "     1   ",
                    "         ",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
            PuzzleTuple(
                "error_testing".to_string(),
                "to test errors".to_string(),
                [
                    "1       1",
                    "   1     ",
                    "      1  ",
                    " 1       ",
                    "    1    ",
                    "       1 ",
                    "  1      ",
                    "     1   ",
                    "         ",
                ].iter().cloned().map(|s| s.to_string()).collect(),
            ),
        ]
        .iter().cloned().collect();

    let puzzle_tuples_by_id: HashMap<String, PuzzleTuple> = test_puzzles
        .iter()
        .cloned()
        .map(|pt| (pt.0.clone(), pt))
        .collect();
        

    /*
    sample_puzzle_map.insert(
        "".to_string(),
        [
        ].iter().cloned().map(|s| s.to_string()).collect());

    let mut sample_puzzle_map: HashMap<String, PuzzleTuple> = HashMap::new();

    let puzzle_tuples_by_id: HashMap<String, PuzzleTuple> =
        [
            (
                String::from("id"),
                PuzzleTuple("id".to_string(), "name".to_string(), vec![]),
            ),
        ].iter().cloned()//.map(|(k,v)| (*k,v)).collect();
        .collect();
*/
    match puzzle_tuples_by_id.get(id) {
        Some(pt) => {
            Some(Puzzle {
                id: pt.0.clone(),
                name: pt.1.clone(),
                puzzle: pt.2.clone(),
            })
        }
        None => None
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
    fn test_map() {
        match get_by_id("sjm20190118") {
            Some(_) => (),
            None => panic!(),
        }
        match get_by_id("sjm20160229") {
            Some(_) => (),
            None => panic!(),
        }
        match get_by_id("should_not_be_there") {
            Some(_) => panic!(),
            None => (),
        }
    }
}
