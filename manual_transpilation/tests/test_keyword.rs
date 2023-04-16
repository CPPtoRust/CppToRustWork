#[path = "keyword.rs"]
mod keyword;
use keyword::*;

#[path = "positions.rs"]
mod positions;
use positions::*;

use std::time::{SystemTime, UNIX_EPOCH};


fn test_sort_char_set() {
    let mut input = [4, 2, 1, 3];
    let expected = [1, 2, 3, 4];
    sort_char_set(&mut input, 4);
    assert!(input == expected);
}

fn test_init_selchars_low() {
    let mut temp: [i32; 256] = [1;256];
    temp[0] = 100;
    temp[2] = 15;
    temp[1] = 27;
    let mut positions: Positions = Positions{_useall: false, _size: 200, _positions: temp};
    let keyword = Keyword { _allchars: "abc", _allchars_length: 3, _rest: "d", _lineno: 0 };
    let mut ext = KeywordExt::default();
    let selchars = ext.init_selchars_low(&positions, &[1, 3, 2], &[2,3,4]);
}


fn test_init_selchars_tuple() {
    let mut temp: [i32; 256] = [1;256];
    temp[0] = 100;
    temp[2] = 15;
    temp[1] = 27;
    let mut positions = Positions{_useall: false, _size: 200, _positions: temp};
    let keyword = Keyword { _allchars: "ab", _allchars_length: 2, _rest: "c", _lineno: 0 };
    let mut ext = KeywordExt::default();
    ext.init_selchars_tuple(&positions, &[2,1,6,4]);
}


fn test_init_selchars_multiset() {
    let mut temp: [i32; 256] = [1;256];
    temp[0] = 100;
    temp[2] = 15;
    temp[1] = 27;
    let mut positions = Positions{_useall: false, _size: 200, _positions: temp};
    let keyword = Keyword { _allchars: "abc", _allchars_length: 3, _rest: "", _lineno: 0 };
    let mut ext = KeywordExt::default();
    ext.init_selchars_multiset(&positions, &[], &[]);

}


pub fn test_keyword() {
 
    let timestamp1 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get timestamp");
    
    let milliseconds1 = timestamp1.subsec_millis();
    println!("Testing keyword.rs file...\n");

    test_sort_char_set();
    test_init_selchars_low();
    test_init_selchars_tuple();
    test_init_selchars_multiset();
    
    println!("Finished testing keyword.rs file!\n");

    let timestamp2 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get timestamp");
    
    let milliseconds2 = timestamp2.subsec_millis();

    println!("Runtime keyword Rust file: {} milliseconds\n\n", milliseconds2 - milliseconds1);
    


}

fn main() {
    test_keyword();
}