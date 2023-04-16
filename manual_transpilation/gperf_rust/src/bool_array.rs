pub struct Bool_Array
{
    pub _size: u32,
    pub _iteration_number: u32,
    pub _storage_array: Vec<u32>,
}

impl Bool_Array {
    pub fn new(size: u32) -> Bool_Array {
        Bool_Array{
            _size : size,
            _iteration_number : 0,
            _storage_array : vec![0; size as usize],
        }

    }

    #[inline]
    pub fn set_bit(&mut self, index: u32) -> bool {
        if self._storage_array[index as usize] == self._iteration_number
        {
            return true;
        }
        else
        {
            self._storage_array[index as usize] = self._iteration_number;
            return false;
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self._iteration_number += 1;

        if self._iteration_number == 0
        {
            self._iteration_number = 1;
            self._storage_array = vec![0; self._size as usize];
        }
    }
}

fn main() {
    
}