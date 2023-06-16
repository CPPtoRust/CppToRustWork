#include <bits/stdc++.h>
using namespace std;

class Bool_Array
{
public:
    Bool_Array(unsigned int size) : _size(size),
                                    _iteration_number(1),
                                    _storage_array(new unsigned int[size])
    {
        memset(_storage_array, 0, size * sizeof(_storage_array[0]));
        if (option[DEBUG])
            fprintf(stderr, "\nbool array size = %d, total bytes = %d\n",
                    _size,
                    static_cast<unsigned int>(_size * sizeof(_storage_array[0])));
    }

    void clear()
    {

        if (++_iteration_number == 0)
        {
            _iteration_number = 1;
            memset(_storage_array, 0, _size * sizeof(_storage_array[0]));
            if (option[DEBUG])
            {
                fprintf(stderr, "(re-initialized bool_array)\n");
                fflush(stderr);
            }
        }
    }

    bool set_bit(unsigned int index)
    {
        if (_storage_array[index] == _iteration_number)

            return true;
        else
        {

            _storage_array[index] = _iteration_number;
            return false;
        }
    }

private:
    unsigned int const _size;
    unsigned int _iteration_number;
    unsigned int *const _storage_array;
};
