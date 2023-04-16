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
struct Keyword
{

  /* Constructor.  */
                        Keyword (const char *allchars, int allchars_length,
                                 const char *rest);

  /* Data members defined immediately by the input file.  */
  /* The keyword as a string, possibly containing NUL bytes.  */
  const char *const     _allchars;
  int const             _allchars_length;
  /* Additional stuff seen on the same line of the input file.  */
  const char *const     _rest;
  /* Line number of this keyword in the input file.  */
  unsigned int          _lineno;
};

/* A keyword, in the context of a given keyposition list.  */

struct KeywordExt : public Keyword
{
  /* Constructor.  */
                        KeywordExt (const char *allchars, int allchars_length,
                                    const char *rest);

  /* Data members depending on the keyposition list.  */
  /* The selected characters that participate for the hash function,
     selected according to the keyposition list, as a canonically reordered
     multiset.  */
  const unsigned int *  _selchars;
  int                   _selchars_length;
  /* Chained list of keywords having the same _selchars and
     - if !option[NOLENGTH] - also the same _allchars_length.
     Note that these duplicates are not members of the main keyword list.  */
  KeywordExt *          _duplicate_link;

  /* Methods depending on the keyposition list.  */
  /* Initializes selchars and selchars_length, without reordering.  */
  void                  init_selchars_tuple (Positions& positions, const unsigned int *alpha_unify);
  /* Initializes selchars and selchars_length, with reordering.  */
  void                  init_selchars_multiset (Positions& positions, const unsigned int *alpha_unify, const unsigned int *alpha_inc);
  /* Deletes selchars.  */
  void                  delete_selchars ();

  /* Data members used by the algorithm.  */
  int                   _hash_value; /* Hash value for the keyword.  */

  /* Data members used by the output routines.  */
  int                   _final_index;

public:
  unsigned int *        init_selchars_low (Positions& positions, const unsigned int *alpha_unify, const unsigned int *alpha_inc);
};

/* An abstract factory for creating Keyword instances.
   This factory is used to make the Input class independent of the concrete
   class KeywordExt.  */

class Keyword_Factory
{
public:
  /* Constructor.  */
                        Keyword_Factory ();
  /* Destructor.  */
  virtual               ~Keyword_Factory ();

  /* Creates a new Keyword.  */
  virtual /*abstract*/ Keyword *
                        create_keyword (const char *allchars, int allchars_length,
                                        const char *rest) = 0;
};

/* A statically allocated empty string.  */
extern char empty_string[1];

/* Constructor.  */
inline
Keyword::Keyword (const char *allchars, int allchars_length, const char *rest)
  : _allchars (allchars), _allchars_length (allchars_length), _rest (rest)
{
}


/* --------------------------- KeywordExt class --------------------------- */

/* Constructor.  */
inline
KeywordExt::KeywordExt (const char *allchars, int allchars_length, const char *rest)
  : Keyword (allchars, allchars_length, rest),
    _final_index (-1)
{
}



static inline void sort_char_set (unsigned int *base, int len)
{
  /* Bubble sort is sufficient here.  */
  for (int i = 1; i < len; i++)
    {
      int j;
      unsigned int tmp;

      for (j = i, tmp = base[j]; j > 0 && tmp < base[j - 1]; j--)
        base[j] = base[j - 1];

      base[j] = tmp;
    }
}

unsigned int *
KeywordExt::init_selchars_low (Positions &positions, const unsigned int *alpha_unify, const unsigned int *alpha_inc)
{
  /* Iterate through the list of positions, initializing selchars
     (via ptr).  */
  PositionIterator iter = positions.iterator(_allchars_length);

  unsigned int *key_set = new unsigned int[iter.remaining()];
  unsigned int *ptr = key_set;

  for (int i; (i = iter.next ()) != PositionIterator::EOS; )
    {
      unsigned int c;
      if (i == Positions::LASTCHAR)
        /* Special notation for last KEY position, i.e. '$'.  */
        c = static_cast<unsigned char>(_allchars[_allchars_length - 1]);
      else if (i < _allchars_length)
        {
          /* Within range of KEY length, so we'll keep it.  */
          c = static_cast<unsigned char>(_allchars[i]);
          if (alpha_inc)
            c += alpha_inc[i];
        }
      else
        /* Out of range of KEY length, the iterator should not have
           produced this.  */
        abort ();
      if (alpha_unify)
        c = alpha_unify[c];
      *ptr = c;
      ptr++;
    }

  _selchars = key_set;
  _selchars_length = ptr - key_set;

  return key_set;
}

void
KeywordExt::init_selchars_tuple (Positions& positions, const unsigned int *alpha_unify)
{
  init_selchars_low (positions, alpha_unify, NULL);
}

void
KeywordExt::init_selchars_multiset (Positions& positions, const unsigned int *alpha_unify, const unsigned int *alpha_inc)
{
  unsigned int *selchars =
    init_selchars_low (positions, alpha_unify, alpha_inc);

  /* Sort the selchars elements alphabetically.  */
  sort_char_set (selchars, _selchars_length);
}

/* Deletes selchars.  */
void
KeywordExt::delete_selchars ()
{
  delete[] const_cast<unsigned int *>(_selchars);
}


/* ------------------------- Keyword_Factory class ------------------------- */

Keyword_Factory::Keyword_Factory ()
{
}

Keyword_Factory::~Keyword_Factory ()
{
}


/* ------------------------------------------------------------------------- */

char empty_string[1] = "";

//----------------------------------------------------------------------------------------------------------------------

void test_sort_char_set() 
{
    unsigned int input[] = {4, 2, 1, 3};
    unsigned int expected[] = {1, 2, 3, 4};
    sort_char_set(input, 4);
    for(int i = 0; i < 4; i++)
        if(input[i] != expected[i])
            assert(false);
}

void test_init_selchars_low() 
{
    Positions positions = Positions();
    positions._useall = false;
    positions._size = 200;
    memset(positions._positions, 1, 256 * sizeof(int));
    positions._positions[0] = 100;
    positions._positions[2] = 15;
    positions._positions[1] = 27;

    Keyword keyword = Keyword("abc", 3, "d");
    keyword._lineno = 0;
    KeywordExt ext = KeywordExt("ab", 2, "c");

    const unsigned int alpha_unify[3] = {1, 3, 2};
    const unsigned int alpha_inc[3] = {2, 3, 4};
    unsigned int* selchars = ext.init_selchars_low(positions, alpha_unify, alpha_inc);
}


void test_init_selchars_tuple() 
{
    Positions positions = Positions();
    positions._useall = false;
    positions._size = 200;
    memset(positions._positions, 1, 256 * sizeof(int));
    positions._positions[0] = 100;
    positions._positions[2] = 15;
    positions._positions[1] = 27;

    Keyword keyword = Keyword("ab", 2, "c");
    keyword._lineno = 0;
    KeywordExt ext = KeywordExt("ab", 2, "c");
    const unsigned int alpha_unify[4] = {2, 1, 6, 4};
    ext.init_selchars_tuple(positions, alpha_unify);
}


void test_init_selchars_multiset() 
{Positions positions = Positions();
    positions._useall = false;
    positions._size = 200;
    memset(positions._positions, 1, 256 * sizeof(int));
    positions._positions[0] = 100;
    positions._positions[2] = 15;
    positions._positions[1] = 27;

    Keyword keyword = Keyword("abc", 3, "d");
    keyword._lineno = 0;
    KeywordExt ext = KeywordExt("ab", 2, "c");
    const unsigned int alpha_unify[] = {};
    const unsigned int alpha_inc[] = {};
    ext.init_selchars_multiset(positions, alpha_unify, alpha_inc);
}



void test_keyword() {
    using namespace std::chrono;
    uint64_t ms1 = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    
    printf("Testing keyword C++ file...\n");
    test_sort_char_set();
    test_init_selchars_low();
    test_init_selchars_tuple();
    test_init_selchars_multiset();
    printf("Finished testing keyword C++ file!\n");

    uint64_t ms2 = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    printf("Runtime keyword file: %u milliseconds\n\n", ms2 - ms1);

}

int main()
{
    test_keyword();
    return 0;
}