#include "stdio.h"
#include "stdlib.h"
#include<cstring>
#include<chrono>
#include<cassert>

using namespace std;


//-----------------------------------------------------------------------------------------

class PositionIterator;
class PositionReverseIterator;

/* This class denotes a set of byte positions, used to access a keyword.  */

class Positions
{
  friend class PositionIterator;
  friend class PositionReverseIterator;
public:
  /* Denotes the last char of a keyword, depending on the keyword's length.  */
  enum {                LASTCHAR = -1 };

  /* Maximum key position specifiable by the user, 1-based.
     Note that MAX_KEY_POS-1 must fit into the element type of _positions[],
     below.  */
  enum {                MAX_KEY_POS = 255 };

  /* Maximum possible size.  Since duplicates are eliminated and the possible
     0-based positions are -1 .. MAX_KEY_POS-1, this is:  */
  enum {                MAX_SIZE = MAX_KEY_POS + 1 };

  /* Constructors.  */
                        Positions ();
                        Positions (int pos1);
                        Positions (int pos1, int pos2);

  /* Copy constructor.  */
                        Positions (const Positions& src);

  /* Assignment operator.  */
  Positions&            operator= (const Positions& src);

  /* Accessors.  */
  bool                  is_useall () const;
  int                   operator[] (unsigned int index) const;
  unsigned int          get_size () const;

  /* Write access.  */
  void                  set_useall (bool useall);
  int *                 pointer ();
  void                  set_size (unsigned int size);

  /* Sorts the array in reverse order.
     Returns true if there are no duplicates, false otherwise.  */
  bool                  sort ();

  /* Creates an iterator, returning the positions in descending order.  */
  PositionIterator      iterator () const;
  /* Creates an iterator, returning the positions in descending order,
     that apply to strings of length <= maxlen.  */
  PositionIterator      iterator (int maxlen) const;
  /* Creates an iterator, returning the positions in ascending order.  */
  PositionReverseIterator reviterator () const;
  /* Creates an iterator, returning the positions in ascending order,
     that apply to strings of length <= maxlen.  */
  PositionReverseIterator reviterator (int maxlen) const;

  /* Set operations.  Assumes the array is in reverse order.  */
  bool                  contains (int pos) const;
  void                  add (int pos);
  void                  remove (int pos);

  /* Output in external syntax.  */
  void                  print () const;

// public:
  /* The special case denoted by '*'.  */
  bool                  _useall;
  /* Number of positions.  */
  unsigned int          _size;
  /* Array of positions.  0 for the first char, 1 for the second char etc.,
     LASTCHAR for the last char.  */
  int                   _positions[MAX_SIZE];
};

/* This class denotes an iterator through a set of byte positions.  */

class PositionIterator
{
  friend class Positions;
public:
  /* Copy constructor.  */
                        PositionIterator (const PositionIterator &src);

  /* End of iteration marker.  */
  enum {                EOS = -2 };

  /* Retrieves the next position, or EOS past the end.  */
  int                   next ();

  /* Returns the number of remaining positions, i.e. how often next() will
     return a value != EOS.  */
  unsigned int          remaining () const;

public:
  /* Initializes an iterator through POSITIONS.  */
                        PositionIterator (Positions positions);
  /* Initializes an iterator through POSITIONS, ignoring positions >= maxlen.  */
                        PositionIterator (Positions positions, int maxlen);

  Positions      _set;
  unsigned int          _index;
};

/* This class denotes an iterator in reverse direction through a set of
   byte positions.  */

class PositionReverseIterator
{
  friend class Positions;
public:
  /* Copy constructor.  */
                        PositionReverseIterator (const PositionReverseIterator &src);

  /* End of iteration marker.  */
  enum {                EOS = -2 };

  /* Retrieves the next position, or EOS past the end.  */
  int                   next ();

  /* Returns the number of remaining positions, i.e. how often next() will
     return a value != EOS.  */
  unsigned int          remaining () const;

public:
  /* Initializes an iterator through POSITIONS.  */
                        PositionReverseIterator (Positions positions);
  /* Initializes an iterator through POSITIONS, ignoring positions >= maxlen.  */
                        PositionReverseIterator (Positions positions, int maxlen);

  Positions      _set;
  unsigned int          _index;
  unsigned int          _minindex;
};


inline
Positions::Positions ()
  : _useall (false),
    _size (0)
{
}

inline
Positions::Positions (int pos1)
  : _useall (false),
    _size (1)
{
  _positions[0] = pos1;
}

inline
Positions::Positions (int pos1, int pos2)
  : _useall (false),
    _size (2)
{
  _positions[0] = pos1;
  _positions[1] = pos2;
}

/* Copy constructor.  */

inline
Positions::Positions (const Positions& src)
  : _useall (src._useall),
    _size (src._size)
{
  memcpy (_positions, src._positions, _size * sizeof (_positions[0]));
}

/* Assignment operator.  */

inline Positions&
Positions::operator= (const Positions& src)
{
  _useall = src._useall;
  _size = src._size;
  memcpy (_positions, src._positions, _size * sizeof (_positions[0]));
  return *this;
}

/* Accessors.  */

inline bool
Positions::is_useall () const
{
  return _useall;
}

inline int
Positions::operator[] (unsigned int index) const
{
  return _positions[index];
}

inline unsigned int
Positions::get_size () const
{
  return _size;
}

/* Write access.  */

inline void
Positions::set_useall (bool useall)
{
  _useall = useall;
  if (useall)
    {
      /* The positions are 0, 1, ..., MAX_KEY_POS-1, in descending order.  */
      _size = MAX_KEY_POS;
      int *ptr = _positions;
      for (int i = MAX_KEY_POS - 1; i >= 0; i--)
        *ptr++ = i;
    }
}

inline int *
Positions::pointer ()
{
  return _positions;
}

inline void
Positions::set_size (unsigned int size)
{
  _size = size;
}

/* Sorts the array in reverse order.
   Returns true if there are no duplicates, false otherwise.  */
inline bool
Positions::sort ()
{
  if (_useall)
    return true;

  /* Bubble sort.  */
  bool duplicate_free = true;
  int *base = _positions;
  unsigned int len = _size;

  for (unsigned int i = 1; i < len; i++)
    {
      unsigned int j;
      int tmp;

      for (j = i, tmp = base[j]; j > 0 && tmp >= base[j - 1]; j--)
        if ((base[j] = base[j - 1]) == tmp) /* oh no, a duplicate!!! */
          duplicate_free = false;

      base[j] = tmp;
    }

  return duplicate_free;
}

/* Creates an iterator, returning the positions in descending order.  */
inline PositionIterator
Positions::iterator () const
{
  return PositionIterator (*this);
}

/* Creates an iterator, returning the positions in descending order,
   that apply to strings of length <= maxlen.  */
inline PositionIterator
Positions::iterator (int maxlen) const
{
  return PositionIterator (*this, maxlen);
}

/* Creates an iterator, returning the positions in ascending order.  */
inline PositionReverseIterator
Positions::reviterator () const
{
  return PositionReverseIterator (*this);
}

/* Creates an iterator, returning the positions in ascending order,
   that apply to strings of length <= maxlen.  */
inline PositionReverseIterator
Positions::reviterator (int maxlen) const
{
  return PositionReverseIterator (*this, maxlen);
}

/* ------------------------- Class PositionIterator ------------------------ */

/* Initializes an iterator through POSITIONS.  */
inline
PositionIterator::PositionIterator (Positions positions)
  : _set (positions),
    _index (0)
{
}

/* Initializes an iterator through POSITIONS, ignoring positions >= maxlen.  */
inline
PositionIterator::PositionIterator (Positions positions, int maxlen)
  : _set (positions)
{
  if (positions._useall)
    _index = (maxlen <= Positions::MAX_KEY_POS ? Positions::MAX_KEY_POS - maxlen : 0);
  else
    {
      unsigned int index;
      for (index = 0;
           index < positions._size && positions._positions[index] >= maxlen;
           index++)
        ;
      _index = index;
    }
}

/* Retrieves the next position, or EOS past the end.  */
inline int
PositionIterator::next ()
{
  return (_index < _set._size ? _set._positions[_index++] : EOS);
}

/* Returns the number of remaining positions, i.e. how often next() will
   return a value != EOS.  */
inline unsigned int
PositionIterator::remaining () const
{
  return _set._size - _index;
}

/* Copy constructor.  */
inline
PositionIterator::PositionIterator (const PositionIterator &src)
  : _set (src._set),
    _index (src._index)
{
}

/* --------------------- Class PositionReverseIterator --------------------- */

/* Initializes an iterator through POSITIONS.  */
inline
PositionReverseIterator::PositionReverseIterator (Positions positions)
  : _set (positions),
    _index (_set._size),
    _minindex (0)
{
}

/* Initializes an iterator through POSITIONS, ignoring positions >= maxlen.  */
inline
PositionReverseIterator::PositionReverseIterator (Positions positions, int maxlen)
  : _set (positions),
    _index (_set._size)
{
  if (positions._useall)
    _minindex = (maxlen <= Positions::MAX_KEY_POS ? Positions::MAX_KEY_POS - maxlen : 0);
  else
    {
      unsigned int index;
      for (index = 0;
           index < positions._size && positions._positions[index] >= maxlen;
           index++)
        ;
      _minindex = index;
    }
}

/* Retrieves the next position, or EOS past the end.  */
inline int
PositionReverseIterator::next ()
{
  return (_index > _minindex ? _set._positions[--_index] : EOS);
}

/* Returns the number of remaining positions, i.e. how often next() will
   return a value != EOS.  */
inline unsigned int
PositionReverseIterator::remaining () const
{
  return _index - _minindex;
}

/* Copy constructor.  */
inline
PositionReverseIterator::PositionReverseIterator (const PositionReverseIterator &src)
  : _set (src._set),
    _index (src._index),
    _minindex (src._minindex)
{
}


bool
Positions::contains (int pos) const
{
  unsigned int count = _size;
  const int *p = _positions + _size - 1;

  for (; count > 0; p--, count--)
    {
      if (*p == pos)
        return true;
      if (*p > pos)
        break;
    }
  return false;
}

void
Positions::add (int pos)
{
  set_useall (false);

  unsigned int count = _size;

  if (count == MAX_SIZE)
    {
      fprintf (stderr, "Positions::add internal error: overflow\n");
      exit (1);
    }

  int *p = _positions + _size - 1;

  for (; count > 0; p--, count--)
    {
      if (*p == pos)
        {
          fprintf (stderr, "Positions::add internal error: duplicate\n");
          exit (1);
        }
      if (*p > pos)
        break;
      p[1] = p[0];
    }
  p[1] = pos;
  _size++;
}

void
Positions::remove (int pos)
{
  set_useall (false);

  unsigned int count = _size;
  if (count > 0)
    {
      int *p = _positions + _size - 1;

      if (*p == pos)
        {
          _size--;
          return;
        }
      if (*p < pos)
        {
          int prev = *p;

          for (;;)
            {
              p--;
              count--;
              if (count == 0)
                break;
              if (*p == pos)
                {
                  *p = prev;
                  _size--;
                  return;
                }
              if (*p > pos)
                break;
              int curr = *p;
              *p = prev;
              prev = curr;
            }
        }
    }
  fprintf (stderr, "Positions::remove internal error: not found\n");
  exit (1);
}

/* Output in external syntax.  */
void
Positions::print () const
{
  if (_useall)
    printf ("*");
  else
    {
      bool first = true;
      bool seen_LASTCHAR = false;
      unsigned int count = _size;
      const int *p = _positions + _size - 1;

      for (; count > 0; p--)
        {
          count--;
          if (*p == LASTCHAR)
            seen_LASTCHAR = true;
          else
            {
              if (!first)
                printf (",");
              printf ("%d", *p + 1);
              if (count > 0 && p[-1] == *p + 1)
                {
                  printf ("-");
                  do
                    {
                      p--;
                      count--;
                    }
                  while (count > 0 && p[-1] == *p + 1);
                  printf ("%d", *p + 1);
                }
              first = false;
            }
        }
      if (seen_LASTCHAR)
        {
          if (!first)
            printf (",");
          printf ("$");
        }
    }
}

//----------------------------------------------------------------------------------------------------------------


void test_set_useall_and_is_useall() 
{
    Positions positions = Positions();
    bool useall = true;
    positions.set_useall(useall);
    assert(positions.is_useall() == true);

}

void test_set_size_and_get_size() 
{

    Positions positions = Positions();
    unsigned int size = 200;
    positions.set_size(size);
    assert(positions.get_size() == 200);
}

void test_pointer_and_sort() 
{
    Positions positions = Positions();
    positions._useall = false;
    positions._size = 256;
    memset(positions._positions, 1, 256 * sizeof(int));
    positions._positions[1] = 100;
    positions._positions[32] = 0;
    positions._positions[163] = 27;
    assert(positions._positions == positions.pointer());
    bool duplicate_free = positions.sort();
    assert(duplicate_free == false);
}


void test_iterator_iterator_maxlen_reviterator_reviterator_maxlen() 
{
    Positions positions = Positions();
    positions._useall = false;
    positions._size = 256;
    memset(positions._positions, 1, 256 * sizeof(int));

    PositionIterator iterator1 = positions.iterator();
    PositionIterator iterator2 = positions.iterator(50);
    PositionReverseIterator reviterator1 = positions.reviterator();
    PositionReverseIterator reviterator2 = positions.reviterator(50);

}

void test_contains() 
{
    Positions positions = Positions();
    positions._useall = false;
    positions._size = 256;
    memset(positions._positions, 1, 256 * sizeof(int));
    positions._positions[0] = 100;
    positions._positions[2] = 15;
    positions._positions[1] = 27;
    positions.sort();
    assert(positions.contains(27) == true);
    assert(positions.contains(500) == false);
}

void test_add_remove()
{
    Positions positions = Positions();
    positions._useall = false;
    positions._size = 200;
    memset(positions._positions, 1, 200 * sizeof(int));

    positions._positions[0] = 100;
    positions._positions[2] = 15;
    positions._positions[1] = 27;

    positions.sort();
    positions.add(7);
    // positions.add(7); //Performing this again will give duplicate error as expected
    positions.remove(7);
    // positions.remove(7); //Performing this again will give not found error as expected
}

void test_iterator_reviterator_next_remaining() 
{
    Positions positions;
    positions._useall = false;
    positions._size = 200;
    memset(positions._positions, 1, 200 * sizeof(int));

    positions._positions[0] = 100;
    positions._positions[2] = 15;
    positions._positions[1] = 27;

    PositionIterator iterator = PositionIterator(positions, 1);  
    iterator._index = 2;

    PositionReverseIterator reviterator = PositionReverseIterator(positions, 0);
    reviterator._set = positions;
    reviterator._index = 1; 
    reviterator._minindex = 0;
    assert(reviterator.next() == 100);
    assert(reviterator.remaining() == 0);
}


void test_positions() {
    using namespace std::chrono;
    uint64_t ms1 = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    
    printf("Testing positions C++ file...\n");
    test_set_useall_and_is_useall();
    test_set_size_and_get_size();
    test_pointer_and_sort();
    test_iterator_iterator_maxlen_reviterator_reviterator_maxlen();    
    test_contains();
    test_add_remove();
    test_iterator_reviterator_next_remaining();

    printf("Finished positions C++ file!\n");

    uint64_t ms2 = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    printf("Runtime positions C++ file: %u milliseconds\n\n", ms2 - ms1);

}

int main()
{
    test_positions();
    return 0;
}