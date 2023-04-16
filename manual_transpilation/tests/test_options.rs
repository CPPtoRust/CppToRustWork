#[path = "options.rs"]
pub mod options;
use options::*;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn test_parse_print_options() {

    let mut options = Options1::new();
    options.parse_options();
    
    assert!(options._slot_name == "slot_dummy"); //Giving -K slot_dummy as command line argument
    assert!(options._language == "C++"); // Giving -L C++ as command line argument

    options.print_options();
}

pub fn test_set_get_total_switches() {

    let mut options = Options1::new();
    options.set_total_switches(3);

    assert!(options.get_total_switches() == 3); 
}


pub fn test_options() {

    let timestamp1 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get timestamp");
    // let seconds1 = timestamp1.as_secs();
    let milliseconds1 = timestamp1.subsec_millis();

    println!("Testing options.rs file...\n");

    test_parse_print_options();
    test_set_get_total_switches();
    
    println!("\n\nFinished testing options.rs file!\n");

    let timestamp2 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get timestamp");
    // let seconds2 = timestamp2.as_secs();
    let milliseconds2 = timestamp2.subsec_millis();

    println!("Runtime options Rust file: {} milliseconds\n\n", milliseconds2 - milliseconds1);
}
