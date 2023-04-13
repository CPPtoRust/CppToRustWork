#include <bits/stdc++.h>
#include<chrono>
#include<cassert>

// #include "keyword.h"
// #include "positions.h"

using namespace std;

class Positions;


class PositionIterator;
class PositionReverseIterator;

/* This class denotes a set of byte positions, used to access a keyword.  */

class Positions
{
   friend class PositionIterator;
   friend class PositionReverseIterator;

public:
   /* Denotes the last char of a keyword, depending on the keyword's length.  */
   enum
   {
      LASTCHAR = -1
   };

   /* Maximum key position specifiable by the user, 1-based.
      Note that MAX_KEY_POS-1 must fit into the element type of _positions[],
      below.  */
   enum
   {
      MAX_KEY_POS = 255
   };

   /* Maximum possible size.  Since duplicates are eliminated and the possible
      0-based positions are -1 .. MAX_KEY_POS-1, this is:  */
   enum
   {
      MAX_SIZE = MAX_KEY_POS + 1
   };

   /* Constructors.  */
   Positions();
   Positions(int pos1);
   Positions(int pos1, int pos2);

   /* Copy constructor.  */
   Positions(const Positions &src);

   /* Assignment operator.  */
   Positions &operator=(const Positions &src);

   /* Accessors.  */
   bool is_useall() const;
   int operator[](unsigned int index) const;
   unsigned int get_size() const;

   /* Write access.  */
   void set_useall(bool useall);
   int *pointer();
   void set_size(unsigned int size);

   /* Sorts the array in reverse order.
      Returns true if there are no duplicates, false otherwise.  */
   bool sort();

   /* Creates an iterator, returning the positions in descending order.  */
   PositionIterator iterator() const;
   /* Creates an iterator, returning the positions in descending order,
      that apply to strings of length <= maxlen.  */
   PositionIterator iterator(int maxlen) const;
   /* Creates an iterator, returning the positions in ascending order.  */
   PositionReverseIterator reviterator() const;
   /* Creates an iterator, returning the positions in ascending order,
      that apply to strings of length <= maxlen.  */
   PositionReverseIterator reviterator(int maxlen) const;

   /* Set operations.  Assumes the array is in reverse order.  */
   bool contains(int pos) const;
   void add(int pos);
   void remove(int pos);

   /* Output in external syntax.  */
   void print() const;

private:
   /* The special case denoted by '*'.  */
   bool _useall;
   /* Number of positions.  */
   unsigned int _size;
   /* Array of positions.  0 for the first char, 1 for the second char etc.,
      LASTCHAR for the last char.  */
   int _positions[MAX_SIZE];
};

/* This class denotes an iterator through a set of byte positions.  */

class PositionIterator
{
   friend class Positions;

public:
   /* Copy constructor.  */
   PositionIterator(const PositionIterator &src);

   /* End of iteration marker.  */
   enum
   {
      EOS = -2
   };

   /* Retrieves the next position, or EOS past the end.  */
   int next();

   /* Returns the number of remaining positions, i.e. how often next() will
      return a value != EOS.  */
   unsigned int remaining() const;

private:
   /* Initializes an iterator through POSITIONS.  */
   PositionIterator(Positions const &positions);
   /* Initializes an iterator through POSITIONS, ignoring positions >= maxlen.  */
   PositionIterator(Positions const &positions, int maxlen);

   const Positions &_set;
   unsigned int _index;
};

/* This class denotes an iterator in reverse direction through a set of
   byte positions.  */

class PositionReverseIterator
{
   friend class Positions;

public:
   /* Copy constructor.  */
   PositionReverseIterator(const PositionReverseIterator &src);

   /* End of iteration marker.  */
   enum
   {
      EOS = -2
   };

   /* Retrieves the next position, or EOS past the end.  */
   int next();

   /* Returns the number of remaining positions, i.e. how often next() will
      return a value != EOS.  */
   unsigned int remaining() const;

private:
   /* Initializes an iterator through POSITIONS.  */
   PositionReverseIterator(Positions const &positions);
   /* Initializes an iterator through POSITIONS, ignoring positions >= maxlen.  */
   PositionReverseIterator(Positions const &positions, int maxlen);

   const Positions &_set;
   unsigned int _index;
   unsigned int _minindex;
};


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
     (false);

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
Positions::Positions ()
  : _useall (false),
    _size (0)
{
}

Positions::Positions (int pos1)
  : _useall (false),
    _size (1)
{
  _positions[0] = pos1;
}

Positions::Positions (int pos1, int pos2)
  : _useall (false),
    _size (2)
{
  _positions[0] = pos1;
  _positions[1] = pos2;
}

Positions::Positions (const Positions& src)
  : _useall (src._useall),
    _size (src._size)
{
  memcpy (_positions, src._positions, _size * sizeof (_positions[0]));
}

Positions&
Positions::operator= (const Positions& src)
{
  _useall = src._useall;
  _size = src._size;
  memcpy (_positions, src._positions, _size * sizeof (_positions[0]));
  return *this;
}

bool
Positions::is_useall () const
{
  return _useall;
}   

int
Positions::operator[] (unsigned int index) const
{
  return _positions[index];
}

unsigned int
Positions::get_size () const
{
  return _size;
}

void
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
int *
Positions::pointer ()
{ 
  return _positions;
}
void
Positions::set_size (unsigned int size)
{
  _size = size;
}
bool
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
PositionIterator
Positions::iterator () const
{
  return PositionIterator (*this);
}
PositionIterator
Positions::iterator (int maxlen) const
{
  return PositionIterator (*this, maxlen);
}

PositionReverseIterator
Positions::reviterator () const
{
  return PositionReverseIterator (*this);
}

PositionReverseIterator
Positions::reviterator (int maxlen) const
{
  return PositionReverseIterator (*this, maxlen);
}

PositionIterator::PositionIterator (Positions const& positions)
  : _set (positions),
    _index (0)
{
}
PositionIterator::PositionIterator (Positions const& positions, int maxlen)
  : _set (positions)
{
  if (positions._useall)
    _index = (maxlen <= Positions::MAX_KEY_POS ? Positions::MAX_KEY_POS - maxlen : 0);
  else
    {
      unsigned int index;
      for (index = 0;index < positions._size && positions._positions[index] >= maxlen;index++)  ;
      _index = index;
    }
}
 int
PositionIterator::next ()
{
  return (_index < _set._size ? _set._positions[_index++] : EOS);
}

/* Returns the number of remaining positions, i.e. how often next() will
   return a value != EOS.  */
 unsigned int
PositionIterator::remaining () const
{
  return _set._size - _index;
}

/* Copy constructor.  */

PositionIterator::PositionIterator (const PositionIterator& src)
  : _set (src._set),
    _index (src._index)
{
}

/* --------------------- Class PositionReverseIterator --------------------- */

/* Initializes an iterator through POSITIONS.  */

PositionReverseIterator::PositionReverseIterator (Positions const& positions)
  : _set (positions),
    _index (_set._size),
    _minindex (0)
{
}

/* Initializes an iterator through POSITIONS, ignoring positions >= maxlen.  */

PositionReverseIterator::PositionReverseIterator (Positions const& positions, int maxlen)
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
 int
PositionReverseIterator::next ()
{
  return (_index > _minindex ? _set._positions[--_index] : EOS);
}

/* Returns the number of remaining positions, i.e. how often next() will
   return a value != EOS.  */
 unsigned int
PositionReverseIterator::remaining () const
{
  return _index - _minindex;
}

/* Copy constructor.  */

PositionReverseIterator::PositionReverseIterator (const PositionReverseIterator& src)
  : _set (src._set),
    _index (src._index),
    _minindex (src._minindex)
{
}

/* An instance of this class is a keyword, as specified in the input file.  */

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
  void                  init_selchars_tuple (const Positions& positions, const unsigned int *alpha_unify);
  /* Initializes selchars and selchars_length, with reordering.  */
  void                  init_selchars_multiset (const Positions& positions, const unsigned int *alpha_unify, const unsigned int *alpha_inc);
  /* Deletes selchars.  */
  void                  delete_selchars ();

  /* Data members used by the algorithm.  */
  int                   _hash_value; /* Hash value for the keyword.  */

  /* Data members used by the output routines.  */
  int                   _final_index;

public:
  unsigned int *        init_selchars_low (const Positions& positions, const unsigned int *alpha_unify, const unsigned int *alpha_inc);
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


static  void sort_char_set (unsigned int *base, int len)
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
KeywordExt::init_selchars_low (const Positions& positions, const unsigned int *alpha_unify, const unsigned int *alpha_inc)
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
KeywordExt::init_selchars_tuple (const Positions& positions, const unsigned int *alpha_unify)
{
  init_selchars_low (positions, alpha_unify, NULL);
}

void
KeywordExt::init_selchars_multiset (const Positions& positions, const unsigned int *alpha_unify, const unsigned int *alpha_inc)
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

Keyword::Keyword (const char *allchars, int allchars_length, const char *rest)
  : _allchars (allchars), _allchars_length (allchars_length), _rest (rest)
{
}

KeywordExt::KeywordExt (const char *allchars, int allchars_length, const char *rest)
  : Keyword (allchars, allchars_length, rest),
    _final_index (-1)
{
}

class Keyword_List
{
public:
  /* Constructor.  */
                        Keyword_List (Keyword *car);

  /* Access to first element of list.  */
  Keyword *             first () const;
  /* Access to next element of list.  */
  Keyword_List *&       rest ();

protected:
  Keyword_List *        _cdr;
  Keyword * const       _car;
};

/* List node of a linear list of KeywordExt.  */
class KeywordExt_List : public Keyword_List
{
public:
  /* Constructor.  */
                        KeywordExt_List (KeywordExt *car);

  /* Access to first element of list.  */
  KeywordExt *          first () const;
  /* Access to next element of list.  */
  KeywordExt_List *&    rest ();
};

/* Copies a linear list, sharing the list elements.  */
extern Keyword_List * copy_list (Keyword_List *list);
extern KeywordExt_List * copy_list (KeywordExt_List *list);

/* Deletes a linear list, keeping the list elements in memory.  */
extern void delete_list (Keyword_List *list);

/* Sorts a linear list, given a comparison function.
   Note: This uses a variant of mergesort that is *not* a stable sorting
   algorithm.  */
extern Keyword_List * mergesort_list (Keyword_List *list,
                                      bool (*less) (Keyword *keyword1,
                                                    Keyword *keyword2));
extern KeywordExt_List * mergesort_list (KeywordExt_List *list,
                                         bool (*less) (KeywordExt *keyword1,
                                                       KeywordExt *keyword2));

Keyword_List::Keyword_List (Keyword *car)
  : _cdr (NULL), _car (car)
{
}
KeywordExt_List::KeywordExt_List (KeywordExt *car)
  : Keyword_List (car)
{
}

Keyword_List *
copy_list (Keyword_List *list)
{
  Keyword_List *result;
  Keyword_List **lastp = &result;
  while (list != NULL)
    {
      Keyword_List *new_cons = new Keyword_List (list->first());
      *lastp = new_cons;
      lastp = &new_cons->rest();
      list = list->rest();
    }
  *lastp = NULL;
  return result;
}

/* Copies a linear list, sharing the list elements.  */
KeywordExt_List *
copy_list (KeywordExt_List *list)
{
  return static_cast<KeywordExt_List *> (copy_list (static_cast<Keyword_List *> (list)));
}

/* Deletes a linear list, keeping the list elements in memory.  */
void
delete_list (Keyword_List *list)
{
  while (list != NULL)
    {
      Keyword_List *rest = list->rest();
      delete list;
      list = rest;
    }
}

/* Type of a comparison function.  */
typedef bool (*Keyword_Comparison) (Keyword *keyword1, Keyword *keyword2);

/* Merges two sorted lists together to form one sorted list.  */
static Keyword_List *
merge (Keyword_List *list1, Keyword_List *list2, Keyword_Comparison less)
{
  Keyword_List *result;
  Keyword_List **resultp = &result;
  for (;;)
    {
      if (!list1)
        {
          *resultp = list2;
          break;
        }
      if (!list2)
        {
          *resultp = list1;
          break;
        }
      if (less (list2->first(), list1->first()))
        {
          *resultp = list2;
          resultp = &list2->rest();
          /* We would have a stable sorting if the next line would read:
             list2 = *resultp;  */
          list2 = list1; list1 = *resultp;
        }
      else
        {
          *resultp = list1;
          resultp = &list1->rest();
          list1 = *resultp;
        }
    }
  return result;
}

/* Sorts a linear list, given a comparison function.
   Note: This uses a variant of mergesort that is *not* a stable sorting
   algorithm.  */
Keyword_List *
mergesort_list (Keyword_List *list, Keyword_Comparison less)
{
  if (list == NULL || list->rest() == NULL)
    /* List of length 0 or 1.  Nothing to do.  */
    return list;
  else
    {
      /* Determine a list node in the middle.  */
      Keyword_List *middle = list;
      for (Keyword_List *temp = list->rest();;)
        {
          temp = temp->rest();
          if (temp == NULL)
            break;
          temp = temp->rest();
          middle = middle->rest();
          if (temp == NULL)
            break;
        }

      /* Cut the list into two halves.
         If the list has n elements, the left half has ceiling(n/2) elements
         and the right half has floor(n/2) elements.  */
      Keyword_List *right_half = middle->rest();
      middle->rest() = NULL;

      /* Sort the two halves, then merge them.  */
      return merge (mergesort_list (list, less),
                    mergesort_list (right_half, less),
                    less);
    }
}

KeywordExt_List *
mergesort_list (KeywordExt_List *list,
                bool (*less) (KeywordExt *keyword1, KeywordExt *keyword2))
{
  return
    static_cast<KeywordExt_List *>
      (mergesort_list (static_cast<Keyword_List *> (list),
                       reinterpret_cast<Keyword_Comparison> (less)));
}

Keyword *
Keyword_List::first () const
{
  return _car;
}

Keyword_List *&
Keyword_List::rest ()
{
  return _cdr;
}

KeywordExt *
KeywordExt_List::first () const
{
  return static_cast<KeywordExt*>(_car);
}

KeywordExt_List *&
KeywordExt_List::rest ()
{
  return *reinterpret_cast<KeywordExt_List**>(&_cdr);
}

void test_keyword_list() {
    using namespace std::chrono;
    uint64_t ms1 = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    
    printf("Testing keyword_list C++ file...\n");
    
    Keyword *kw = new Keyword("",0,"");
    KeywordExt *kwext = new KeywordExt("",0,"");
    Positions *po = new Positions();
    unsigned int au[0];
    unsigned int ai[0];

    kwext->init_selchars_low(*po,au,ai);
    kwext->init_selchars_multiset(*po,au,ai);
    kwext->init_selchars_tuple(*po,au);

    printf("Finished testing keyword_list C++ file!\n");

    uint64_t ms2 = duration_cast<milliseconds>(system_clock::now().time_since_epoch()).count();
    printf("Runtime keyword_list file: %u milliseconds\n\n", ms2 - ms1);

}


int main()
{
    test_keyword_list();
    return 0;
}