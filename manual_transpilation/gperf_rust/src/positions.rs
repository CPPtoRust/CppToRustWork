use std::ops::Index;


#[derive(Clone, Copy)]
pub struct Positions{
    pub _useall: bool,
    pub _size: u32,
    pub _positions: [i32; 256]
}


impl Default for Positions {

    fn default() -> Positions {
        Positions {
            _useall: false,
            _size: 0,
            _positions: [-1; 256]
        }
    }

}


impl Index<usize> for Positions {
    type Output = i32;
    fn index<'a>(&'a self, idx: usize) -> &'a i32 {
        return &self._positions[idx];
    }
}


//Assignment operator overloading is not possible in Rust


impl Positions {

    pub const LASTCHAR: i32 = -1;
    pub const MAX_KEY_POS: i32 = 255;
    pub const MAX_SIZE: i32 = Positions::MAX_KEY_POS + 1;

    #[inline]
    pub const fn is_useall(&self) -> bool {
        return self._useall;
    }

    #[inline]
    pub const fn get_size(&self) -> u32 {
        return self._size;
    }

    #[inline]
    pub fn set_useall(&mut self, useall: bool) {
        self._useall = useall;
        if useall {
            /* The positions are 0, 1, ..., MAX_KEY_POS-1, in descending order.  */
            self._size = Positions::MAX_KEY_POS as u32;
            let mut i: i32 = Positions::MAX_KEY_POS - 1;
            while i >= 0  {
                self._positions[(self._positions.len() as i32 - 1 - i) as usize] += 1;
                i -= 1;
            }
        }
    }

    #[inline]
    pub fn pointer(&self) -> [i32; 256] {
        return self._positions;
    }

    #[inline]
    pub fn set_size(&mut self, size: u32) {
        self._size = size;
    }

    /* Sorts the array in reverse order.
       Returns true if there are no duplicates, false otherwise.  */
    #[inline]
    pub fn sort(&mut self) -> bool {
        if self._useall {
            return true;
        }

        let mut duplicate_free: bool = true;
        let mut len: u32 = self._size;

        for i in 1..len {

            let mut j: usize = i as usize;
            let mut tmp: i32 = self._positions[j];
    
            while j > 0 && tmp >= self._positions[j - 1] {
                self._positions[j] = self._positions[j - 1];
                if self._positions[j] == tmp {
                    duplicate_free = false;
                }
                j -= 1;
            }

            self._positions[j] = tmp;
        }

        return duplicate_free;
    }
    
    /* Creates an iterator, returning the positions in descending order.  */
    pub fn iterator(&self) -> PositionIterator {
        return PositionIterator{_set: self, _index: 0};
    }
    
    pub fn iterator_maxlen(&self, maxlen: i32) -> PositionIterator {
        let mut index: usize;

        if self._useall {
            index = if maxlen <= Positions::MAX_KEY_POS {(Positions::MAX_KEY_POS - maxlen) as usize} else {0};
        } else {
            index = 0;
            while index < self._size as usize && self._positions[index] >= maxlen {
                index += 1;
            }
        }


        return PositionIterator{_set: self, _index: index as u32};
    }

    /* Creates an iterator, returning the positions in ascending order.  */
    pub fn reviterator(&self) -> PositionReverseIterator {
        return PositionReverseIterator{_set: self,  _index: 0, _minindex: 0};
    }

    pub fn reviterator_maxlen(&self, maxlen: i32) -> PositionReverseIterator {
        let mut index: usize;

        if self._useall {
            index = if maxlen <= Positions::MAX_KEY_POS {(Positions::MAX_KEY_POS - maxlen) as usize} else {0};
        } else {
            index = 0;
            while index < self._size as usize && self._positions[index] >= maxlen {
                index += 1;
            }
        }

        return PositionReverseIterator{_set: self, _index: self._size, _minindex: index as u32};
    }

    
    pub const fn contains(&self, pos: i32) -> bool {
        
        let mut count: u32 = self._size;
        let mut p: i32 = self._size as i32 - 1;

        while count > 0 {
            if self._positions[p as usize] == pos {
                return true;
            }

            if self._positions[p as usize] > pos {
                break;
            }

            p -= 1;
            count -= 1;
        }

        return false;
    }


    pub fn add(&mut self, pos: i32) {

        self.set_useall(false);

        let mut count: u32 = self._size;

        if count == Positions::MAX_SIZE as u32  {
            eprintln!("Positions::add internal error: overflow");
            std::process::exit(1);
        }

        let mut p: usize = self._size as usize - 1;

        while count > 0 {
            if self._positions[p] == pos {
                eprintln!("Positions:add internal error: duplicate");
                std::process::exit(1);
            }

            if self._positions[p] > pos {
                break;
            }

            self._positions[p + 1] = self._positions[p];
            
            p -= 1;
            count -= 1;
        }

        self._positions[p + 1] = pos;
        self._size += 1;
    }


    pub fn remove(&mut self, pos: i32) {
        
        self.set_useall(false);

        let mut count: u32 = self._size;

        if count > 0 {
            let mut p: usize = self._size as usize - 1;

            if self._positions[p] == pos {
                self._size -= 1;
                return;
            }

            if self._positions[p] < pos {
                let mut prev: i32 = self._positions[p];

                while true {
                    p -= 1;
                    count -= 1;
                    if count == 0 {
                        break;
                    }
                    if self._positions[p] == pos {
                        self._positions[p] = prev;
                        self._size -= 1;
                        return;
                    }
                    if self._positions[p] > pos {
                        break;
                    }
                    let mut curr: i32 = self._positions[p];
                    self._positions[p] = prev;
                    prev = curr;
                }

            }
        }
        eprintln!("Positions::remove internal error: not found");
        std::process::exit(1);
    }


    pub fn print(&self) {
        if self._useall {
            print!("*");
        } else {
            let mut first: bool = true;
            let mut seen_LASTCHAR: bool = false;
            let mut count: u32 = self._size;
            let mut p: usize = self._size as usize - 1;

            while count > 0 {
                count -= 1;
                if self._positions[p] == Positions::LASTCHAR {
                    seen_LASTCHAR = true;
                } else {
                    if !first {
                        print!(",");
                    }
                    print!("{}", self._positions[p] + 1);
                    if count > 0 && self._positions[p - 1] == self._positions[p] + 1 {
                        print!("-");
                        loop {
                            p -= 1;
                            count -= 1;
                            if !(count > 0 && self._positions[p - 1] == self._positions[p] + 1) {
                                break;
                            }
                        }
                        print!("{}", self._positions[p] + 1);
                    }
                    first = false;
                }

            }
            if seen_LASTCHAR {
                if !first {
                    print!(",");
                }
                print!("$");
            }
        }

    }

}


/* ------------------------- Class PositionIterator ------------------------ */


//#[derive(Default)]
pub struct PositionIterator<'a> {
    pub _set: &'a Positions,
    pub _index: u32
}

//Keep note of various constructors of PositionIterator

impl<'a> PositionIterator<'a> {

    pub const EOS: i32 = -2;

    /* Retrieves the next position, or EOS past the end.  */
    pub fn next(&mut self) -> i32 {
        
        if self._index < self._set._size {
            self._index += 1;
            return self._set._positions[self._index as usize];
        } else {
            return PositionIterator::EOS;
        }

    }

    pub const fn remaining(&self) -> u32 {
        return self._set._size - self._index;
    }
    
}

/* --------------------- Class PositionReverseIterator --------------------- */

pub struct PositionReverseIterator<'a> {

    pub _set: &'a Positions,
    pub _index: u32,
    pub _minindex: u32

}


//Keep note of various constructors of PositionReverseIterator

impl<'a> PositionReverseIterator<'a> {

    pub const EOS: i32 = -2;

    pub fn next(&mut self) -> i32 {

        if self._index > self._minindex {
            self._index -= 1;
            return self._set._positions[self._index as usize];
        } else {
            return PositionReverseIterator::EOS;
        }

    }

    pub const fn remaining(&self) -> u32 {
        return self._index - self._minindex;
    }

}