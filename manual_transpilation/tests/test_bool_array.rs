#[path = "bool_array.rs"]
pub mod bool_array;
use bool_array::*;

use std::time::{SystemTime, UNIX_EPOCH};

fn test_bool_array_set_bit() {
    let mut bool_array = Bool_Array::new(5); 
    bool_array._iteration_number = 1;
    assert_eq!(bool_array.set_bit(2), false); 
    assert_eq!(bool_array.set_bit(2), true); 
}

fn test_bool_array_clear() {
    let mut bool_array = Bool_Array::new(3); 
    bool_array._iteration_number = 1;
    bool_array.set_bit(0); 
    bool_array.set_bit(2);

    bool_array.clear(); 

    assert_eq!(bool_array.set_bit(0), false); 
    assert_eq!(bool_array.set_bit(2), false); 
}    


pub fn test_bool_array() {

    let timestamp1 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get timestamp");
    // let seconds1 = timestamp1.as_secs();
    let milliseconds1 = timestamp1.subsec_millis();

    println!("Testing bool_array.rs file...\n");
    test_bool_array_set_bit();
    test_bool_array_clear();
    println!("Finished testing bool_array.rs file!\n");

    let timestamp2 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get timestamp");
    // let seconds2 = timestamp2.as_secs();
    let milliseconds2 = timestamp2.subsec_millis();

    println!("Runtime bool_array Rust file: {} milliseconds\n\n", (milliseconds2 - milliseconds1));

}

fn main() {
    test_bool_array();
}