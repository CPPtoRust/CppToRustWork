#[path = "positions.rs"]
pub mod positions;
use positions::{Positions, PositionIterator, PositionReverseIterator};

use std::process;


#[inline]
fn sort_char_set(base: &mut [u32], len: i32) {

    for i in 1..len + 1 {
        
        let mut j: usize = i as usize;
        let tmp: u32 = base[j];

        while j > 0 && tmp < base[j - 1] {
            base[j] = base[j - 1];
            j -= 1;
        }

        base[j] = tmp;
    }

}

/* Declaring Keyword */

#[derive(Default, Copy, Clone)]
pub struct Keyword<'a>{
    pub(crate) _allchars: &'a str,
    pub(crate) _allchars_length: i32,
    pub _rest: &'a str,
    pub _lineno: u32
}


#[derive(Default)]
pub struct KeywordExt<'a>{
    pub keyword: Keyword<'a>,
    pub _selchars: Vec<i32>,
    pub _selchars_length: i32,
    pub _duplicate_link: Option<&'a mut KeywordExt<'a>>,
    pub _hash_value: i32,
    pub _final_index: i32 // Default value = -1
}


impl KeywordExt<'_>{

    pub fn init_selchars_low(&mut self, positions: &Positions, alpha_unify: &[u32], alpha_inc: &[u32]) -> Vec<u32> {

        let mut iter: PositionIterator = positions.iterator_maxlen(self.keyword._allchars_length);

        let mut key_set: Vec<u32> = vec![0; iter.remaining() as usize];
        let mut ptr: usize = 0;
        let mut i: i32;

        while true {
            
            i = iter.next();
            if i == -2 {
                break;
            }

            let mut c: u32;
            
            if i == -1 {
                c = self.keyword._allchars.chars().nth(self.keyword._allchars_length as usize - 1).unwrap() as u32;
            } else if i < self.keyword._allchars_length {
                c = self.keyword._allchars.chars().nth(i as usize).unwrap() as u32;
                if alpha_inc.len() > 0 {
                    c += alpha_inc[i as usize] as u32;
                }

            } else {
                process::abort();
                if alpha_unify.len() > 0 {
                    c = alpha_unify[c as usize];
                }

                key_set[ptr] = c;
                ptr += 1;
            
            }

        }

        self._selchars = Vec::with_capacity(key_set.len());

        for i in 0..key_set.len() {
            self._selchars[i] = key_set[i] as i32;
        }

        self._selchars_length = ptr as i32;

        return key_set;

    }


    pub fn init_selchars_tuple(&mut self, positions: &Positions, alpha_unify: &[u32]){

        self.init_selchars_low (positions, alpha_unify, &vec![]);
    }


    pub fn init_selchars_multiset(&mut self, positions: &Positions, alpha_unify: &[u32], alpha_inc: &[u32]){

        let mut selchars: Vec<u32> = self.init_selchars_low (positions, alpha_unify, alpha_inc);
        
        sort_char_set(&mut selchars, self._selchars_length);
    }


    pub fn delete_selchars(&self){
        //std::mem::drop(self._selchars);
        
    }

}


struct Keyword_Factory {

}

static empty_string: [char; 1] = ['\0'];
