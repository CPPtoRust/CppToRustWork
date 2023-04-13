// mod bool_array;
pub mod keyword;
use keyword::{*, positions::Positions};
use std::time::{SystemTime, UNIX_EPOCH};

// pub mod positions;
// use positions::*;
// mod keyword_list;
// use keyword_list::*;
// mod output;
// use output::*;
// mod search;
// use search::*;
// use bool_array::Bool_Array;
// mod hash_table;
// use hash_table::*;
fn main() {
    let timestamp1 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get timestamp");

    let milliseconds1 = timestamp1.subsec_nanos();

    // let mut ba  = Bool_Array::new(10);
    // ba.set_bit(1);
    // ba.clear();
    // ba.set_bit(1);
    // ba.clear();
    // ba.set_bit(1);
    let mut kw = Keyword {
        _allchars: "",
        _allchars_length: 0,
        _rest: "",
        _lineno: 0,
    };

    let mut kwext = KeywordExt{
        keyword: kw,
        _selchars: vec![0;0],
        _selchars_length: 0,
        _duplicate_link: None,
        _hash_value: 0,
        _final_index: 0,
    };

    let mut po = Positions{..Default::default()};
    
    let mut au:Vec<u32> = vec![0;0];
    let mut ai:Vec<u32> = vec![0;0];
    kwext.init_selchars_low(&po, &au, &ai);

    kwext.init_selchars_multiset(&po, &au, &ai);
    kwext.init_selchars_tuple(&po, &au);

    // po.is_useall();
    // po.get_size();
    // po.set_useall(true);
    // po.pointer();
    // po.set_size(0);
    // po.sort();
    let timestamp2 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get timestamp");
    // let seconds2 = timestamp2.as_secs();
    let milliseconds2 = timestamp2.subsec_nanos();

    println!("Runtime bool_array Rust file: {} milliseconds\n\n", (milliseconds2 - milliseconds1));


}
