#include <bits/stdc++.h>
#include<chrono>
#include<cassert>

using namespace std;

class Bool_Array
{
public:
  /* Initializes the bit array with room for SIZE bits, numbered from
     0 to SIZE-1. */
                        Bool_Array (unsigned int size);

  /* Frees this object.  */
                        ~Bool_Array ();

  /* Resets all bits to zero.  */
  void                  clear ();

  /* Sets the specified bit to true.
     Returns its previous value (false or true).  */
  bool                  set_bit (unsigned int index);

public:
  /* Size of array.  */
  unsigned int const    _size;

  /* Current iteration number.  Always nonzero.  Starts out as 1, and is
     incremented each time clear() is called.  */
  unsigned int          _iteration_number;

  /* For each index, we store in storage_array[index] the iteration_number at
     the time set_bit(index) was last called.  */
  unsigned int * const  _storage_array;
};

Bool_Array::Bool_Array (unsigned int size)
  : _size (size),
    _iteration_number (1),
    _storage_array (new unsigned int [size])
{
  memset (_storage_array, 0, size * sizeof (_storage_array[0]));
  if (true)
    fprintf (stderr, "\nbool array size = %d, total bytes = %d\n",
             0,
             static_cast<unsigned int> (0 * sizeof (_storage_array[0])));
}

bool Bool_Array::set_bit (unsigned int index)
{
  if (_storage_array[index] == _iteration_number)
    /* The bit was set since the last clear() call.  */
    return true;
  else
    {
      /* The last operation on this bit was clear().  Set it now.  */
      _storage_array[index] = _iteration_number;
      return false;
    }
}

void
Bool_Array::clear ()
{
  /* If we wrap around it's time to zero things out again!  However, this only
     occurs once about every 2^32 iterations, so it will not happen more
     frequently than once per second.  */

  if (++_iteration_number == 0)
    {
      _iteration_number = 1;
      memset (_storage_array, 0, _size * sizeof (_storage_array[0]));
      if (true)
        {
          fprintf (stderr, "(re-initialized bool_array)\n");
          fflush (stderr);
        }
    }
}

Bool_Array::~Bool_Array ()
{
  /* Print out debugging diagnostics. */
  if (true)
    fprintf (stderr, "\ndumping boolean array information\n"
             "size = %d\niteration number = %d\nend of array dump\n",
             0, 0);
  delete[] const_cast<unsigned int *>(_storage_array);
}

void test_bool_array_set_bit() 
{
    Bool_Array bool_array = Bool_Array(5);
    bool_array._iteration_number = 1;
    assert(bool_array.set_bit(2) == false);
    assert(bool_array.set_bit(2) == true);
}

void test_bool_array_clear() {
    Bool_Array bool_array = Bool_Array(3); 
    bool_array._iteration_number = 1;
    bool_array.set_bit(0); 
    bool_array.set_bit(2);

    bool_array.clear(); 

    assert(bool_array.set_bit(0) == false); 
    assert(bool_array.set_bit(2) == false); 
}   

void test_bool_array() {
    using namespace std::chrono;
    uint64_t ms1 = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    
    printf("Testing bool_array C++ file...\n");
    test_bool_array_set_bit();
    test_bool_array_clear();
    printf("Finished testing bool_array C++ file!\n");

    uint64_t ms2 = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    printf("Runtime bool_array file: %u milliseconds\n\n", ms2 - ms1);

}

int main()
{
    test_bool_array();
    return 0;
}