#[path = "positions.rs"]
pub mod positions;
use positions::*;

use std::time::{SystemTime, UNIX_EPOCH};

fn test_set_useall_and_is_useall() {

    let mut positions = Positions::default();
    let useall = true;
    positions.set_useall(useall);
    assert!(positions.is_useall() == true)

}

fn test_set_size_and_get_size() {

    let mut positions = Positions::default();
    let size = 200;
    positions.set_size(size);
    assert!(positions.get_size() == 200);
}

fn test_pointer_and_sort() {

    let mut temp: [i32; 256] = [1;256];
    temp[1] = 100;
    temp[32] = 0;
    temp[163] = 27;
    let mut positions = Positions{_useall: false, _size: 256, _positions: temp};
    assert!(temp == positions.pointer());
    let duplicate_free: bool = positions.sort();
    assert!(duplicate_free == false)
}


fn test_iterator_iterator_maxlen_reviterator_reviterator_maxlen() {

    let mut positions = Positions{_useall: false, _size: 256, _positions: [10; 256]};
    let iterator1 = positions.iterator();
    let iterator2 = positions.iterator_maxlen(50);
    let reviterator1 = positions.reviterator();
    let reviterator2 = positions.reviterator_maxlen(50);

}

fn test_contains() {
    let mut temp: [i32; 256] = [1;256];
    temp[0] = 100;
    temp[2] = 15;
    temp[1] = 27;
    let mut positions = Positions{_useall: false, _size: 256, _positions: temp};
    
    positions.sort();
    assert!(positions.contains(27) == true);
    assert!(positions.contains(500) == false);
}

fn test_add_remove() {

    let mut temp: [i32; 256] = [1;256];
    temp[0] = 100;
    temp[2] = 15;
    temp[1] = 27;
    let mut positions = Positions{_useall: false, _size: 200, _positions: temp};
    positions.sort();
    positions.add(7);
    // positions.add(7); //Performing this again will give duplicate error as expected
    positions.remove(7);
    // positions.remove(7); //Performing this again will give not found error as expected
}

fn test_iterator_reviterator_next_remaining() {

    let mut temp: [i32; 256] = [1;256];
    temp[0] = 100;
    temp[2] = 15;
    temp[1] = 27;
    let mut positions = Positions{_useall: false, _size: 200, _positions: temp};
    

    let mut iterator = PositionIterator{_set: &positions, _index: 1};
    assert!(iterator.next() == 15);
    assert!(iterator.remaining() == 198);
    
   let mut reviterator = PositionReverseIterator{_set: &positions, _index: 1, _minindex: 0};
   assert!(reviterator.next() == 100);
   assert!(reviterator.remaining() == 0);
}



pub fn test_positions() {

    let timestamp1 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get timestamp");
    // let seconds1 = timestamp1.as_secs();
    let milliseconds1 = timestamp1.subsec_millis();

    println!("Testing positions.rs file...\n");

    test_set_useall_and_is_useall();
    test_set_size_and_get_size();
    test_pointer_and_sort();
    test_iterator_iterator_maxlen_reviterator_reviterator_maxlen();    
    test_contains();
    test_add_remove();
    test_iterator_reviterator_next_remaining();

    println!("Finished testing positions.rs file!\n");

    let timestamp2 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get timestamp");
    // let seconds2 = timestamp2.as_secs();
    let milliseconds2 = timestamp2.subsec_millis();

    println!("Runtime positions Rust file: {} milliseconds\n\n", milliseconds2 - milliseconds1);

}

fn main() {
    test_positions();
}