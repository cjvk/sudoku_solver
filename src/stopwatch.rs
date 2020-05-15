use std::time::{SystemTime, UNIX_EPOCH};

pub struct Stopwatch {
    time_start: Option<std::time::Duration>,
    time_end: Option<std::time::Duration>,
    elapsed_millis: u128,
    elapsed_micros: u128,
}

impl Stopwatch {
    pub fn new() -> Self {
        Stopwatch {
            time_start: None,
            time_end: None,
            elapsed_millis: 0u128,
            elapsed_micros: 0u128,
        }
    }
    pub fn _is_running(&self) -> bool {
        self.time_start.is_some()
    }
    pub fn start(&mut self) {
        if self.time_start.is_some() || self.time_end.is_some() {
            panic!("illegal stopwatch configuration");
        }
        self.time_start = Some(SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards"));
    }
    pub fn stop(&mut self) {
        self.time_end = Some(SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards again"));
        // print!("time_start={:?}, time_end={:?}", self.time_start, self.time_end);
        let new_elapsed_millis = self.elapsed_millis +
            (self.time_end.unwrap() - self.time_start.unwrap()).as_millis();
        self.elapsed_millis = new_elapsed_millis;
        let new_elapsed_micros = self.elapsed_micros +
            (self.time_end.unwrap() - self.time_start.unwrap()).as_micros();
        self.elapsed_micros = new_elapsed_micros;
        self.reset();
    }
    pub fn reset(&mut self) {
        self.time_start = None;
        self.time_end = None;
    }
    pub fn elapsed_millis(&self) -> u128 {
        self.elapsed_micros / 1000
        // // if never started, return 0
        // // otherwise, consider an error (for now) if elapsed_millis() is called with running stopwatch
        // if !self.time_start.is_some() && !self.time_end.is_some() {
        //     return 0u128;
        // }
        // (self.time_end.unwrap() - self.time_start.unwrap()).as_millis()
    }
}
