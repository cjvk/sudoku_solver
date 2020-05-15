pub fn get_bitmap_values() -> Vec<usize> {
    return [511, 1, 2, 4, 8, 16, 32, 64, 128, 256]
        .iter().cloned().collect();
}

pub trait Square {
    fn count_remaining(&self) -> usize;
    fn has_single_value(&self) -> bool;
    fn get_single_value(&mut self) -> usize;
    fn get_single_value_nomut(&self) -> usize;
    fn eliminate(&mut self, arg: usize) -> ();
    fn eliminate_list(&mut self, arg: &Vec<usize>) -> ();
    fn bitmap_elimination(&mut self, bitmap: usize) -> ();
    fn select(&mut self, arg: usize) -> ();
    fn is_dirty(&self) -> bool;
    fn clean(&mut self) -> ();
    fn is_possible(&self, value: usize) -> bool;
    fn values_remaining(&self) -> Vec<usize>;
    fn get_bitmap(&self) -> usize;
    fn clone(&self) -> Box<dyn Square>;
}

#[derive(Default)]
pub struct DerivedSquare {
    bitmap: usize,
    single_value: Option<usize>,
    dirty: bool,
}
impl DerivedSquare {
    pub fn copy(initial_state_bitmap: usize) -> Self {
        DerivedSquare {
            bitmap: initial_state_bitmap,
            ..Default::default()
        }
    }
    pub fn new() -> Self {
        DerivedSquare {
            bitmap: get_bitmap_values()[0],
            ..Default::default()
        }
    }
    pub fn number_of_bits(bitmap: usize) -> usize {
        match bitmap {
            0 => 0,
            _ => 1 + DerivedSquare::number_of_bits(bitmap & bitmap-1),
        }
    }
    fn calculate_single_value(&self) -> usize {
        let mut ret_value = 0;
        [1,2,3,4,5,6,7,8,9]
            .iter()
            .for_each(|i| if self.is_possible(*i) { ret_value = *i });
        match ret_value {
            0 => panic!("error finding single value!"),
            _ => ret_value,
        }
    }
}
impl Square for DerivedSquare {
    fn clone(&self) -> Box<dyn Square> {
        Box::new(DerivedSquare::copy(self.get_bitmap()))
    }
    fn count_remaining(&self) -> usize {
        DerivedSquare::number_of_bits(self.bitmap)
    }
    fn has_single_value(&self) -> bool {
        self.count_remaining() == 1
    }
    fn get_single_value_nomut(&self) -> usize {
        if !self.has_single_value() {
            panic!("do not call get_single_value if no single value!");
        }
        self.calculate_single_value()
    }
    // this is mutable because of memoization on derived squares
    fn get_single_value(&mut self) -> usize {
        if !self.has_single_value() {
            panic!("do not call get_single_value if no single value!");
        }
        match self.single_value {
            Some(i) => i,
            None => {
                let single_value = self.calculate_single_value();
                self.single_value = Some(single_value);
                single_value
            }
        }
    }
    fn eliminate(&mut self, arg: usize) -> () {
        // not sure why the "mut" is not required
        if arg < 1 || arg > 9 {
            panic!("Sudoku only works 1-9: {}", arg);
        }
        self.bitmap_elimination(get_bitmap_values()[arg])
    }
    fn eliminate_list(&mut self, arg: &Vec<usize>) -> () {
        let mut cumulative_bitmap: usize = 0;
        for el in arg {
            cumulative_bitmap = cumulative_bitmap ^ get_bitmap_values()[*el];
        }
        self.bitmap_elimination(cumulative_bitmap)
        // for loop also did not work (borrow checker)
        // for el in arg {
        //     self.eliminate(el);
        // }

        // iterators did not work
        // arg
        //     .iter()
        //     .for_each(|i| self.eliminate(*i))
    }
    fn bitmap_elimination(&mut self, bitmap: usize) -> () {
        let previous_bitmap = self.bitmap;
        self.bitmap = self.bitmap & (get_bitmap_values()[0] ^ bitmap);
        if previous_bitmap != self.bitmap {
            self.dirty = true;
        }
        ()
    }
    fn select(&mut self, arg: usize) -> () {
        match self.is_possible(arg) {
            false => panic!("cannot select"),
            true => {
                let is_nop = get_bitmap_values()[arg] == self.bitmap;
                match is_nop {
                    true => (),
                    false => {
                        self.dirty = true;
                        self.bitmap = get_bitmap_values()[arg];
                        ()
                    }
                }
            }
        }
    }
    fn is_dirty(&self) -> bool { self.dirty }
    fn clean(&mut self) -> () {
        self.dirty = false;
    }
    fn is_possible(&self, value: usize) -> bool {
        self.bitmap & get_bitmap_values()[value] != 0
    }
    fn values_remaining(&self) -> Vec<usize> {
        [1,2,3,4,5,6,7,8,9]
            .iter()
            .filter(|i| self.is_possible(**i as usize))
            .map(|i| *i as usize)
            .collect()
    }
    fn get_bitmap(&self) -> usize { self.bitmap }
}

pub struct InitialSquare {
    value: usize,
}
impl InitialSquare {
    pub fn new(value: usize) -> Self {
        assert!(value>=1 && value<=9);
        InitialSquare {
            value
        }
    }
}
impl Square for InitialSquare {
    fn clone(&self) -> Box<dyn Square> {
        Box::new(InitialSquare::new(self.get_single_value_nomut()))
    }
    fn count_remaining(&self) -> usize { 1 }
    fn has_single_value(&self) -> bool { true }
    fn get_single_value(&mut self) -> usize { self.value }
    fn get_single_value_nomut(&self) -> usize { self.value }
    fn eliminate(&mut self, _arg: usize) -> () {}
    fn eliminate_list(&mut self, _arg: &Vec<usize>) -> () {}
    fn bitmap_elimination(&mut self, _bitmap: usize) -> () {}
    fn select(&mut self, _arg: usize) -> () {}
    fn is_dirty(&self) -> bool { false }
    fn clean(&mut self) -> () {}
    fn is_possible(&self, value: usize) -> bool { value == self.value }
    fn values_remaining(&self) -> Vec<usize> {
        [self.value].iter().cloned().collect()
    }
    fn get_bitmap(&self) -> usize { get_bitmap_values()[self.value] }
}

#[cfg(test)]
mod tests {

    // use super::InitialSquare;

    #[test]
    fn always_pass() {
        assert!(true);
    }

    mod derived_square {
        use super::super::DerivedSquare;
        use super::super::Square;

        #[test]
        fn get_bitmap() {
            let mut s = DerivedSquare::new();
            assert!(s.get_bitmap() == 511);
            [3,4,5,6,7,8,9]
                .iter()
                .for_each(|i| s.eliminate(*i));
            assert!(s.get_bitmap() == 3);
        }

        #[test]
        fn values_remaining() {
            let mut s = DerivedSquare::new();
            [1,2,3,4,5,6,7,8,9]
                .iter()
                .zip(&s.values_remaining())
                .for_each(|(e1,e2)| assert!(*e1 == *e2));
            [1,7,8,9]
                .iter()
                .for_each(|i| s.eliminate(*i));
            [2,3,4,5,6]
                .iter()
                .zip(&s.values_remaining())
                .for_each(|(e1,e2)| assert!(*e1 == *e2));
        }

        #[test]
        fn is_possible() {
            let mut s = DerivedSquare::new();
            [1,2,3,4,5,6,7,8,9]
                .iter()
                .for_each(|i| assert!(s.is_possible(*i)));
            [2,3,4,5,6,7,8,9]
                .iter()
                .for_each(|i| s.eliminate(*i));
            [1,2,3,4,5,6,7,8,9]
                .iter()
                .for_each(|i| {
                    match i {
                        1 => assert!(s.is_possible(*i)),
                        _ => assert!(!s.is_possible(*i)),
                    }
                });
            
        }

        #[test]
        fn clean() {
            let mut s = DerivedSquare::new();
            assert!(!s.is_dirty());
            s.select(3);
            assert!(s.is_dirty());
            s.clean();
            s.clean();
            assert!(!s.is_dirty());
        }

        #[test]
        fn is_dirty() {
            let mut s = DerivedSquare::new();
            assert!(!s.is_dirty());
            assert!(!s.is_dirty());
            s.select(3);
            s.select(3);
            assert!(s.is_dirty());
            assert!(s.is_dirty());
        }

        #[test]
        fn select() {
            let mut s = DerivedSquare::new();
            s.select(1);
            s.select(1);
        }

        #[test]
        fn bitmap_elimination() {
            let mut s = DerivedSquare::new();
            s.bitmap_elimination(8);
            s.bitmap_elimination(8);
        }

        #[test]
        fn eliminate_list() {
            let mut s = DerivedSquare::new();
            s.eliminate_list(&vec![1, 2]);
        }

        #[test]
        fn eliminate() {
            let mut s = DerivedSquare::new();
            s.eliminate(1);
            s.eliminate(2);
        }

        #[test]
        #[should_panic]
        fn get_single_value_panic() {
            let mut s = DerivedSquare::new();
            s.get_single_value();
        }

        #[test]
        fn get_single_value() {
            let mut s = DerivedSquare::new();
            s.select(1);
            assert!(s.get_single_value() == 1);
            assert!(s.get_single_value() == 1);
        }

        #[test]
        fn has_single_value() {
            let s = DerivedSquare::new();
            assert!(!s.has_single_value());
            assert!(!s.has_single_value());
        }

        #[test]
        fn count_remaining() {
            let s = DerivedSquare::new();
            assert!(s.count_remaining() == 9);
            assert!(s.count_remaining() == 9);
        }
    }

    mod initial_square {

        use super::super::InitialSquare;
        use super::super::Square;

        #[test]
        fn get_bitmap() {
            let s = InitialSquare::new(5);
            // this is how bitmap values is structured (5th element is 2^^4)
            assert!(s.get_bitmap() == 16);
            assert!(s.get_bitmap() == 16);
        }

        #[test]
        fn values_remaining() {
            let s = InitialSquare::new(5);

            let remaining = s.values_remaining();
            let remaining = s.values_remaining();

            assert!(remaining.len() == 1);
            assert!(remaining[0] == 5);
        }

        #[test]
        fn is_possible() {
            let s = InitialSquare::new(5);
            [1,2,3,4,6,7,8,9]
                .iter()
                .for_each(|val| assert!(!s.is_possible(*val)));
            assert!(s.is_possible(5));
        }

        #[test]
        fn clean() {
            let mut nine = InitialSquare::new(9);
            nine.clean();
            nine.clean();
        }

        #[test]
        fn is_dirty() {
            let eight = InitialSquare::new(8);
            assert!(!eight.is_dirty());
            assert!(!eight.is_dirty());
        }

        #[test]
        fn select() {
            let mut seven = InitialSquare::new(7);
            seven.select(0);
            seven.select(1);
            seven.select(7);
            seven.select(9);
            seven.select(10);
        }

        #[test]
        fn bitmap_elimination() {
            let mut six = InitialSquare::new(6);
            six.bitmap_elimination(8);
            six.bitmap_elimination(8);
        }

        #[test]
        fn eliminate_list() {
            let mut five = InitialSquare::new(5);
            let some_values = vec![1, 2];
            five.eliminate_list(&some_values);
            five.eliminate_list(&some_values);
        }

        #[test]
        fn eliminate() {
            let mut four = InitialSquare::new(4);
            four.eliminate(0);
            four.eliminate(1);
            four.eliminate(4);
            four.eliminate(10);
        }

        #[test]
        fn get_single_value() {
            let mut three = InitialSquare::new(3);
            assert!(three.get_single_value() == 3);
            assert!(three.get_single_value() == 3);
        }

        #[test]
        fn new() {
            [1,2,3,4,5,6,7,8,9]
                .iter()
                .for_each(|i| { InitialSquare::new(*i); () })
        }

        #[test]
        #[should_panic]
        fn new_toohigh() {
            InitialSquare::new(10);
        }

        #[test]
        #[should_panic(expected="assert")]
        fn new_toolow() {
            InitialSquare::new(0);
        }

        #[test]
        fn has_single_value() {
            let one = InitialSquare::new(1);
            assert!(one.has_single_value());
            assert!(one.has_single_value());
        }

        #[test]
        fn count_remaining() {
            let two = InitialSquare::new(2);
            assert!(two.count_remaining() == 1);
            assert!(two.count_remaining() == 1);
        }
        
    }
    

}
