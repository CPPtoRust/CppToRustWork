#[path = "keyword.rs"] mod keyword;
use keyword::{*};
struct Hash_Table<'a> {
    _table: Vec<Option<KeywordExt<'a>>>,
    _size: u32,
    _log_size: u32,
    _ignore_length: bool,
    _collisions: u32
}

const size_factor: i32 = 10;

impl Hash_Table<'_> {

    pub fn new(mut size: u32, mut ignore_length: bool) -> Hash_Table<'static> {
        
        size = size * size_factor as u32;

        let mut shift: u32 = 0;

        if (size >> 16) > 0 {
            size = size >> 16;
            shift += 16;
        }

        if (size >> 8) > 0 {
            size = size >> 8;
            shift += 8;
        }

        if (size >> 4) > 0 {
            size = size >> 4;
            shift += 4;
        }

        if (size >> 2) > 0 {
            size = size >> 2;
            shift += 2;
        }

        if (size >> 1) > 0 {
            size = size >> 1;
            shift += 1;
        }
        return Hash_Table{
            _log_size : shift,
            _size : 1<<shift,
            _table: Hash_Table::initVec(1<<shift),
            _ignore_length: false,
            _collisions: 0,
        };
        
    }
    fn initVec(size : u32) -> Vec<Option<KeywordExt<'static>>>
    {
        let temp : Vec<Option<KeywordExt>>;

        for i in 0..size
        {
            temp.push(None);
        }
        return temp;
    }
    pub fn insert(&mut self,mut item: &mut KeywordExt) -> Option<Keyword>{
        
        //Know what is hashpjw function
        let mut hash_val: u32 = hashpjw(item._selchars, item._selchars_length * (std::mem::size_of::<u32>() as i32));
        let mut probe: u32 = hash_val & (self._size - 1);
        let mut increment: u32 = (((hash_val >> self._log_size) ^ (if self._ignore_length == true {0} else {item.keyword._allchars_length}) as u32) << 1) + 1;

        while (self._table.add(probe)) != None {
            if self.equal(*(self._table.add(probe)), item) {
                return *(self._table.add(probe));
            }

            self._collisions += 1;
            probe = (probe + increment) & (self._size - 1);
        }

        *(self._table.add(probe)) = item;
        return None;
    }

    pub const fn dump(&mut self) {

        let mut field_width: i32;
        field_width = 0;
        
        {
            let mut i: i32 = (self._size - 1 ) as i32;
            while i >= 0 {
                if *(self._table.add(i)) != None {
                    if field_width < *(*(self._table.add(i)))._selchars_length {
                        field_width = *(*(self._table.add(i)))._selchars_length;
                    }
                }

                i -= 1;
            }
        }

        
        eprint!("\ndumping the hash table\ntotal available table slots = {}, total bytes = {}, total collisions = {}\nlocation, {:field_width$}, keyword\n", 
                        self._size, self._size * (std::mem::size_of_val(&self._table) as u32), 
                        self._collisions, "keysig");

        let mut i: i32 = (self._size - 1) as i32;
        while i >= 0 {
            if (self._table.add(i)) != None {
                eprint!("{:>8}, ", i);
                if field_width > *(*(self._table.add(i)))._selchars_length {
                    eprint!("{:>a$}", "", a = (field_width as usize) - ((self._table.add(i)))._selchars_length);
                }
                let mut j: i32 = 0;
                while j < *(*(self._table.add(i)))._selchars_length {
                    eprint!("{}", *(*(*(self._table.add(i))).selchars.add(j)));
                    j += 1;
                }

                eprint!(", {:.a$}\n", ((self._table.add(i)))._allchars, a = ((self._table.add(i)))._allchars_length);
            } 
        }        

        eprint!("\nend dumping hash table\n\n");
    }

    #[inline]
    fn equal(&mut self,mut item1: &mut KeywordExt, mut item2: &mut KeywordExt) -> bool {

        if item1._selchars_length != item2._selchars_length {
            return false;
        }

        for i in 0..item2._selchars_length {
            if (item1._selchars.add(i)) != (item2._selchars.add(i)) {
                return false;
            }
        }

        if self._ignore_length == false && item1.keyword._allchars_length != item2.keyword._allchars_length {
            return false;
        }

        return true;

    }

}


// impl Drop for Hash_Table {
//     fn drop(&mut self) {
//         std::mem::drop(self._table);
//     }
// }