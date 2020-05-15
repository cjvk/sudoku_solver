use crate::config;

pub fn debug(s: String) {
    if config::DEBUG_PRINT_ENABLED {
        println!("{}", s);
    }
}

