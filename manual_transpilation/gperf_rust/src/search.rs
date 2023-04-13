#[path = "bool_array.rs"]
mod Bool_Array;
#[path = "hash_table.rs"]
mod Hash_Table;
#[path = "keyword_list.rs"]
mod KeywordExt_List;
// #[path = "keyword_list.rs"]
mod Keyword_list;
#[path = "positions1.rs"]
mod positions;
use std::io::{self, Write};
struct EquivalenceClass;
use std::process;

struct Search {
    pub _head: *mut Keyword_list,
    pub _total_keys: i32,
    pub _max_key_len: i32,
    pub _min_key_len: i32,
    pub _hash_includes_len: bool,
    pub _key_positions: Positions,
    pub _alpha_inc: &mut u32,
    pub _alpha_size: u32,
    pub _alpha_unify: &mut u32,
    pub _max_selchars_length: u32,
    pub _total_duplicates: i32,
    pub _occurrences: &mut i32,
    pub _asso_values: &mut i32,

    _list_len: i32,
    _asso_value_max: u32,
    _initial_asso_value: i32,
    _jump: i32,
    _max_hash_value: i32,
    _collision_detector: *mut Bool_Array,
}

impl Default for Search {
    fn default() -> Search {
        Search {
            _head: todo!(),
            _total_keys: todo!(),
            _max_key_len: todo!(),
            _min_key_len: todo!(),
            _hash_includes_len: todo!(),
            _key_positions: todo!(),
            _alpha_inc: todo!(),
            _alpha_size: todo!(),
            _alpha_unify: todo!(),
            _max_selchars_length: todo!(),
            _total_duplicates: todo!(),
            _occurrences: todo!(),
            _asso_values: todo!(),
            _list_len: todo!(),
            _asso_value_max: todo!(),
            _initial_asso_value: todo!(),
            _jump: todo!(),
            _max_hash_value: todo!(),
            _collision_detector: todo!(),
        }
    }
}

impl Search {
    pub fn prepare() {
        _total_keys = 0;
        let temp: *mut Keyword_list = _head;

        while temp != ptr::null() {
            _total_keys = _total_keys + 1;
            temp = temp.rest();
        }

        let _max_key_len = std::i32::MIN;
        let _min_key_len = std::i32::MAX;
        temp = _head;
        while temp != ptr::null() {
            let keyword: *mut Keyword_list = temp.first();

            if (_max_key_len < keyword._allchars_length) {
                _max_key_len = keyword._allchars_length;
            }
            if (_min_key_len > keyword._allchars_length) {
                _min_key_len = keyword._allchars_length;
            }
            temp = temp.rest();
        }

        if (_min_key_len == 0) {
            eprintln!(
                "{}{}{}",
                "Empty input keyword is not allowed.\n",
                "To recognize an empty input keyword, your code should check for\n",
                "len == 0 before calling the gperf generated lookup function.\n"
            );

            process::exit(1);
        }
        if (option[SEVENBIT]) {
            temp = _head;
            while temp != ptr::null() {
                let keyword: *mut Keyword_list = temp.first();
                let k: Vec<char> = keyword._allchars.chars().collect();
                let i: i32 = keyword._allchars_length;
                let ind = 0;
                while i > 0 {
                    if ((k[ind] as u32) > 127) {
                        eprintln!("Option --seven-bit has been specified,\nbut keyword \"{}{}\" contains non-ASCII characters.\nTry removing option --seven-bit.\n",
                        keyword._allchars_length, keyword._allchars);
                        process::exit(1);
                    }
                    ind = ind + 1;
                    i = i - 1;
                }
                temp = temp.rest();
            }
        }
        _hash_includes_len = !(option[NOLENGTH] || (_min_key_len == _max_key_len));
    }

    pub fn compute_alpha_size() -> u32 {
        if (option[SEVENBIT]) {
            return 128;
        }
        return 256;
    }

    pub fn compute_alpha_unify() -> Vec<u32> {
        if (option[UPPERLOWER]) {
            let alpha_size: u32 = compute_alpha_size();
            let mut alpha_unify: Vec<u32> = vec![0; alpha_size];

            let mut c = 0;
            while c < alpha_size {
                alpha_unify[c] = c;
                c = c + 1;
            }
            c = 'A';
            while c < 'Z' {
                alpha_unify[c] = c + ('a' - 'A');
                c = c + 1;
            }

            return alpha_unify;
        } else {
            return ptr::null();
        }
    }

    pub fn init_selchars_tuple(positions: &mut Positions, alpha_unify: Vec<u32>) {
        let temp: *mut Keyword_list = _head;

        while temp != ptr::null() {
            temp.first().init_selchars_tuple(positions, alpha_unify);
            temp = temp.rest();
        }
    }

    pub fn delete_selchars() {
        let temp: *mut Keyword_list = _head;

        while temp != ptr::null() {
            temp.first().delete_selchars();
            temp = temp.rest();
        }
    }

    pub fn count_duplicates_tuple(positions: &mut Positions, alpha_unify: Vec<u32>) -> u32 {
        init_selchars_tuple(positions, alpha_unify);
        let count: u32 = 0;
        let mut representatives: Hash_Table = Hash_Table.new(_total_keys, !_hash_includes_len);

        let temp: *mut Keyword_list = _head;

        while temp != ptr::null() {
            let keyword: *mut Keyword_list = temp.first();

            if (representatives.insert(keyword)) {
                count = count + 1;
            }

            temp = temp.rest();
        }
        delete_selchars();
        return count;
    }

    pub fn find_positions() {
        if (option[POSITIONS]) {
            _key_positions = option.get_key_positions();
            return;
        }

        let mut alpha_unify: Vec<u32> = compute_alpha_unify();

        let mut mandatory: Positions;
        if (!option[DUP]) {
            let l1: *mut KeywordExt_List = _head;

            while l1 != ptr::null() && l1.rest() {
                let keyword1: *mut KeywordExt = l1.first();
                let l2: *mut KeywordExt_List = l1.rest();

                while l2 != ptr::null() {
                    let keyword2: *mut KeywordExt = l2.first();
                    if (keyword1._allchars_length == keyword2._allchars_length) {
                        let n: i32 = keyword1._allchars_length;
                        let i: i32 = 0;
                        while i < n - 1 {
                            let mut c1: char = keyword1.keyword._allchars.chars().collect()[0];
                            let mut c2: char = keyword2.keyword._allchars.chars().collect()[0];

                            if (option[UPPERLOWER]) {
                                if (c1 >= 'A' && c1 <= 'Z') {
                                    c1 = c1 + 'a' - 'A';
                                }
                                if (c2 >= 'A' && c2 <= 'Z') {
                                    c2 = c2 + 'a' - 'A';
                                }
                            }
                            if (c1 != c2) {
                                break;
                            }
                            i = i + 1;
                        }
                        if (i < n - 1) {
                            let j = i + 1;
                            while j < n {
                                let mut c1: char = keyword1.keyword._allchars.chars().collect()[0];
                                let mut c2: char = keyword2.keyword._allchars.chars().collect()[0];

                                if (option[UPPERLOWER]) {
                                    if (c1 >= 'A' && c1 <= 'Z') {
                                        c1 = c1 + 'a' - 'A';
                                    }
                                    if (c2 >= 'A' && c2 <= 'Z') {
                                        c2 = c2 + 'a' - 'A';
                                    }
                                }
                                if (c1 != c2) {
                                    break;
                                }
                                j = j + 1;
                            }
                            if (j >= n) {
                                /* Position i is mandatory.  */
                                if (!mandatory.contains(i)) {
                                    mandatory.add(i);
                                }
                            }
                        }
                    }

                    l2 = l2.rest()
                }

                l1 = l1.rest()
            }
        }

        let mut imax: i32;
        if (_key_len - 1 < Positions.MAX_KEY_POS - 1) {
            imax = _max_key_len - 1;
        } else {
            imax = Positions.MAX_KEY_POS - 1;
        }

        let mut current: Positions;
        let current_duplicates_count: u32 = count_duplicates_tuple(current, alpha_unify);

        loop {
            let mut best: Positions;
            let best_duplicates_count: i32 = UINT_MAX;
            let mut i = imax;

            while i >= -1 {
                if (!current.contains(i)) {
                    let mut tryal: Positions = current;
                    tryal.add(i);
                    let try_duplicates_count: u32 = count_duplicates_tuple(tryal, alpha_unify);

                    /* We prefer 'try' to 'best' if it produces less duplicates,
                    or if it produces the same number of duplicates but with
                    a more efficient hash function.  */
                    if (try_duplicates_count < best_duplicates_count
                        || (try_duplicates_count == best_duplicates_count && i >= 0))
                    {
                        best = tryal;
                        best_duplicates_count = try_duplicates_count;
                    }
                }

                i = i - 1;
            }

            if (best_duplicates_count >= current_duplicates_count) {
                break;
            }

            current = best;
            current_duplicates_count = best_duplicates_count;
        }

        loop {
            let mut best: Positions;
            let best_duplicates_count: i32 = UINT_MAX;
            let mut i = imax;

            while i >= -1 {
                if (current.contains(i) && !mandatory.contains(i)) {
                    let mut tryal: Positions = current;
                    tryal.remove(i);
                    let try_duplicates_count = count_duplicates_tuple(tryal, alpha_unify);

                    /* We prefer 'try' to 'best' if it produces less duplicates,
                    or if it produces the same number of duplicates but with
                    a more efficient hash function.  */
                    if (try_duplicates_count < best_duplicates_count
                        || (try_duplicates_count == best_duplicates_count && i == -1))
                    {
                        best = tryal;
                        best_duplicates_count = try_duplicates_count;
                    }
                }

                i = i - 1;
            }

            /* Stop removing positions when it gives no improvement.  */
            if (best_duplicates_count > current_duplicates_count) {
                break;
            }

            current = best;
            current_duplicates_count = best_duplicates_count;
        }

        loop {
            let mut best: Positions;
            let best_duplicates_count: i32 = UINT_MAX;
            let mut i1 = imax;

            while i1 >= -1 {
                if (current.contains(i1) && !mandatory.contains(i1)) {
                    let mut i2 = imax;
                    while i2 >= -1 {
                        if (current.contains(i2) && !mandatory.contains(i2) && i2 != i1) {
                            let mut i3 = imax;
                            while i3 >= -1 {
                                if (!current.contains(i3)) {
                                    let tryal: Positions = current;
                                    tryal.remove(i1);
                                    tryal.remove(i2);
                                    tryal.add(i3);
                                    let try_duplicates_count =
                                        count_duplicates_tuple(tryal, alpha_unify);

                                    /* We prefer 'try' to 'best' if it produces less duplicates,
                                    or if it produces the same number of duplicates but with
                                    a more efficient hash function.  */
                                    if (try_duplicates_count < best_duplicates_count
                                        || (try_duplicates_count == best_duplicates_count
                                            && (i1 == -1 || i2 == -1 || i3 >= 0)))
                                    {
                                        best = tryal;
                                        best_duplicates_count = try_duplicates_count;
                                    }
                                }
                                i3 = i3 - 1;
                            }
                        }
                        i2 = i2 - 1;
                    }
                }
                i1 = i1 - 1;
            }

            /* Stop removing positions when it gives no improvement.  */
            if (best_duplicates_count > current_duplicates_count) {
                break;
            }

            current = best;
            current_duplicates_count = best_duplicates_count;
        }

        _key_positions = current;

        if (option[DEBUG]) {
            /* Print the result.  */
            eprintln!("\nComputed positions: ");
            let mut iter: PositionReverseIterator = _key_positions.reviterator();
            let mut seen_lastchar = false;
            let mut first = true;
            let mut i = iter.next();
            while i != PositionReverseIterator.EOS {
                if (!first) {
                    eprintln!(", ");
                }
                if (i == Positions.LASTCHAR) {
                    seen_lastchar = true;
                } else {
                    eprintln!("{}", i + 1);
                    first = false;
                }
                i = iter.next();
            }
            if (seen_lastchar) {
                if (!first) {
                    eprintln!(", ");
                }
                eprintln!("$");
            }
            eprintln!("\n");
        }

        drop(alpha_unify);
    }

    pub fn count_duplicates_tuple2() -> u32 {
        let mut alpha_unify: Vec<u32> = compute_alpha_unify();
        let count = count_duplicates_tuple(_key_positions, alpha_unify);

        drop(alpha_unify);

        return count as u32;
    }

    pub fn compute_alpha_size2(alpha_inc: Vec<u32>) -> u32 {
        let mut max_alpha_inc: u32 = 0;
        let i = 0;
        while i < _max_key_len {
            if (max_alpha_inc < alpha_inc[i]) {
                max_alpha_inc = alpha_inc[i];
            }
            i = i + 1;
        }
        if (option[SEVENBIT]) {
            return 128 + max_alpha_inc;
        } else {
            return 256 + max_alpha_inc;
        }
    }

    pub fn compute_alpha_unify(positions: &mut Positions, alpha_inc: Vec<u32>) -> Vec<u32> {
        if (option[UPPERLOWER]) {
            /* Without alpha increments, we would simply unify
            'A' -> 'a', ..., 'Z' -> 'z'.
            But when a keyword contains at position i a character c,
            we have the constraint
                asso_values[tolower(c) + alpha_inc[i]] ==
                asso_values[toupper(c) + alpha_inc[i]].
            This introduces a unification
            toupper(c) + alpha_inc[i] -> tolower(c) + alpha_inc[i].
            Note that this unification can extend outside the range of
            ASCII letters!  But still every unified character pair is at
            a distance of 'a'-'A' = 32, or (after chained unification)
            at a multiple of 32.  So in the end the alpha_unify vector has
            the form    c -> c + 32 * f(c)   where f(c) is a nonnegative
            integer.  */
            let alpha_size = compute_alpha_size2(alpha_inc);

            let mut alpha_unify = Vec::with_capacity(alpha_size as usize);
            let mut c = 0;
            while c < alpha_size {
                alpha_unify[c] = c;
                c = c + 1;
            }
            let mut temp: *mut KeywordExt_List = _head;
            while temp != ptr::null() {
                let keyword: *mut KeywordExt = temp.first();

                let iter = positions.iterator(keyword._allchars_length);
                let mut i = iter.next();
                while i != PositionIterator::EOS {
                    let c;
                    if (i == Positions::LASTCHAR) {
                        c = (keyword._allchars[keyword._allchars_length - 1]) as c_uchar;
                    } else if (i < keyword._allchars_length) {
                        c = (keyword._allchars[i]) as c_uchar;
                    } else {
                        abort();
                    }
                    if (c >= 'A' && c <= 'Z') {
                        c = c + 'a' - 'A';
                    }
                    if (c >= 'a' && c <= 'z') {
                        if (i != Positions::LASTCHAR) {
                            c = c + alpha_inc[i];
                        }
                        /* Unify c with c - ('a'-'A').  */
                        let d = alpha_unify[c];
                        let b = c - ('a' - 'A') as usize;
                        let a = b;
                        while a >= 0 && alpha_unify[a] == b {
                            alpha_unify[a] = d;
                            a = a - ('a' - 'A') as usize;
                        }
                    }
                    i = iter.next();
                }
                temp = temp.rest();
            }
            return alpha_unify;
        } else {
            return ptr::null();
        }
    }

    pub fn init_selchars_multiset(
        positions: &mut Positions,
        alpha_unify: Vec<u32>,
        alpha_inc: Vec<u32>,
    ) {
        let mut temp: *mut KeywordExt_List = _head;
        while temp != ptr::null() {
            temp.first()
                .init_selchars_multiset(positions, alpha_unify, alpha_inc);
            temp = temp.rest();
        }
    }

    pub fn count_duplicates_multiset(alpha_inc: Vec<u32>) -> u32 {
        let alpha_unify: Vec<u32> = compute_alpha_unify(_key_positions, alpha_inc);
        init_selchars_multiset(_key_positions, alpha_unify, alpha_inc);

        let count: u32 = 0;
        let representatives: Hash_Table = Hash_Table.new(_total_keys, !_hash_includes_len);
        let mut temp: *mut KeywordExt_List = _head;
        while temp != ptr::null() {
            let keyword: *mut KeywordExt = temp.first();
            if (representatives.insert(keyword)) {
                count = count + 1;
            }
            temp = temp.rest();
        }
        delete_selchars();
        drop(alpha_unify);

        return count;
    }

    pub fn find_alpha_inc() {
        // TODO
        todo!();
    }

    pub fn prepare_asso_values() {
        let mut temp: *mut KeywordExt_List;

        init_selchars_multiset(_key_positions, _alpha_unify, _alpha_inc);
        _max_selchars_length = _key_positions.iterator(_max_key_len).remaining();

        _list_len = _total_keys;
        _total_duplicates = 0;

        let mut representatives = Hash_Table.new(_list_len, !_hash_includes_len);

        let mut prev: *mut KeywordExt_List;

        temp = _head;
        while temp != ptr::null() {
            let keyword: *mut KeywordExt = temp.first();
            let other_keyword: *mut KeywordExt = representatives.insert(keyword);
            let garbage: *mut KeywordExt_List = ptr::null();

            if (other_keyword) {
                _total_duplicates = _total_duplicates + 1;
                _list_len = _total_len - 1;
                /* Remove keyword from the main list.  */
                prev.rest() = temp.rest();
                garbage = temp;
                /* And insert it on other_keyword's duplicate list.  */
                keyword._duplicate_link = other_keyword._duplicate_link;
                other_keyword._duplicate_link = keyword;

                /* Complain if user hasn't enabled the duplicate option. */
                if (!option[DUP] || option[DEBUG]) {
                    eprintln!(
                        "Key link: \"{}{}\" = \"{}{}\", with key set \"",
                        keyword._allchars_length,
                        keyword._allchars,
                        other_keyword._allchars_length,
                        other_keyword._allchars
                    );
                    // for (int j = 0; j < keyword->_selchars_length; j++)
                    //     putc (keyword->_selchars[j], stderr);
                    eprintln!("\".\n");
                }
            } else {
                keyword._duplicate_link = ptr::null();
                prev = temp;
            }
            temp = temp.rest();
            if (garbage != ptr::null()) {
                drop(garbage);
            }
        }

        if (option[DEBUG]) {
            representatives.dump();
        }

        if (_total_duplicates) {
            if (option[DUP]) {
                eprintln!(
                    "{} input keys have identical hash values, examine output carefully...\n",
                    _total_duplicates,
                );
            } else {
                eprintln!(
                    "{} input keys have identical hash values,\n",
                    _total_duplicates,
                );
                if (option[POSITIONS]) {
                    eprintln!("try different key positions or use option -D.\n");
                } else {
                    eprintln!("use option -D.\n");
                }
                process::exit(1);
            }
        }

        _occurrences = vec![0; _alpha_size];
        temp = _head;

        while temp != ptr::null() {
            let keyword: *mut KeywordExt = temp.first();
            let ptr: &u32 = keyword._selchars;
            let count = keyword._selchars.length;
            //TODO: check
            while count > 0 {
                _occurrences[*ptr] = _occurrences[*ptr] + 1;
                ptr = &ptr + 1;
                count = count - 1;
            }

            temp = temp.rest();
        }

        _asso_values = vec![0; alpha_size];

        let mut non_linked_length: i32 = _list_len;
        let mut asso_value_max: u32;

        asso_value_max = (non_linked_length * option.get_size_multiple()) as u32;

        if (asso_value_max == 0) {
            asso_value_max = 1;
        }
        asso_value_max = asso_value_max | asso_value_max >> 1;
        asso_value_max = asso_value_max | asso_value_max >> 2;
        asso_value_max = asso_value_max | asso_value_max >> 4;
        asso_value_max = asso_value_max | asso_value_max >> 8;
        asso_value_max = asso_value_max | asso_value_max >> 16;
        asso_value_max = asso_value_max + 1;
        _asso_value_max = asso_value_max;

        if (_hash_includes_len) {
            _max_hash_value = _max_key_len + (_asso_value_max - 1) * _max_selchars_length;
        } else {
            _max_hash_value = (_asso_value_max - 1) * _max_selchars_length;
        }

        _collision_detector = Bool_Array.new(_max_hash_value + 1);

        if (option[DEBUG]) {
            eprintln! ("total non-linked keys = {}\nmaximum associated value is {}"
                    "\nmaximum size of generated hash table is {}\n",
                    non_linked_length, asso_value_max, _max_hash_value);

            let mut field_width;

            field_width = 0;
            let mut temp: *mut KeywordExt_List = _head;
            while temp != ptr::null() {
                let keyword: *mut KeywordExt = temp.first();
                if (field_width < keyword._selchars_length) {
                    field_width = keyword._selchars_length;
                }

                temp = temp.rest();
            }

            eprintln!("\ndumping the keyword list without duplicates\n");
            eprintln!("keyword #, {}{}, keyword\n", field_width, "keysig");
            let i = 0;
            temp = _head;
            while temp != ptr::null() {
                let keyword: *mut KeywordExt = temp.first();
                i = i + 1;
                eprintln!( "{}, ", ++i);
                if (field_width > keyword._selchars_length) {
                    eprintln!("{}{}", field_width - keyword._selchars_length, "");
                }

                // for (int j = 0; j < keyword->_selchars_length; j++)
                // putc (keyword->_selchars[j], stderr);

                eprintln!(", {}{}\n", keyword._allchars_length, keyword._allchars);
                temp = temp.rest();
            }
            eprintln!("\nend of keyword list\n\n");
        }

        if (option[RANDOM] || option.get_jump() == 0) {
            // TODO : check
            /* We will use rand(), so initialize the random number generator.  */
            //srand (static_cast<long>(time (0)));
        }
        if (option[RANDOM]) {
            _initial_asso_value = -1;
        } else {
            _initial_asso_value = option.get_initial_asso_value();
        }
        _jump = option.get_jump();
    }

    pub fn compute_partition(undetermined: Vec<bool>) -> *mut EquivalenceClass {
        let partition: *mut EquivalenceClass = ptr::null();
        let partition_last: *mut EquivalenceClass = ptr::null();

        let mut temp: *mut KeywordExt_List = _head;

        while temp != ptr::null() {
            let keyword: *mut KeywordExt = temp.first();

            let undetermined_chars: Vec<u32> = vec![0, keyword._selchars_length];
            let undetermined_chars_length: u32 = 0;
            let i: u32 = 0;
            while i < keyword._selchars_length {
                if (undetermined[keyword._selchars[i]]) {
                    undetermined_chars[undetermined_chars_length] = keyword._selchars[i];
                    undetermined_chars_length = undetermined_chars_length + 1;
                }
                i = i + 1;
            }

            let mut equclass: *mut EquivalenceClass = partition;
            while equclass != ptr::null() {
                if (equclass._undetermined_chars_length == undetermined_chars_length
                    && equals(
                        equclass._undetermined_chars,
                        undetermined_chars,
                        undetermined_chars_length,
                    ))
                {
                    break;
                }
                equclass = equclass._next;
            }

            if (equclass == ptr::null()) {
                equclass = &mut EquivalenceClass;
                equclass._keywords = ptr::null();
                equclass._keywords_last = ptr::null();
                equclass._cardinality = 0;
                equclass._undetermined_chars = undetermined_chars;
                equclass._undetermined_chars_length = undetermined_chars_length;
                equclass._next = ptr::null();
                if (partition) {
                    partition_last._next = equclass;
                } else {
                    partition = equclass;
                }
                partition_last = equclass;
            } else {
                drop(undetermined_chars);
            }

            /* Add the keyword to the equivalence class.  */
            let cons: *mut KeywordExt_List = &mut KeywordExt_List(keyword);
            if (equclass._keywords) {
                equclass._keywords_last.rest() = cons;
            } else {
                equclass._keywords = cons;
            }
            equclass._keywords_last = cons;
            equclass._cardinality = equclass._cardinality + 1;

            temp = temp.rest();
        }

        let cls: *mut EquivalenceClass = partition;
        while cls != ptr::null() {
            drop(cls._undetermined_chars);
            cls = cls._next;
        }

        return partition;
    }

    pub fn count_possible_collisions(partition: *mut EquivalenceClass, c: u32) -> u32 {
        let sum: u32 = 0;
        let m: u32 = _max_selchars_length;

        let mut split_cardinalities: Vec<u32> = vec![0, m + 1];

        let cls: *mut EquivalenceClass = partition;
        while cls != ptr::null() {
            split_cardinalities = vec![0, m + 1];
            let mut temp: *mut KeywordExt_List = cls._keywords;

            while temp != ptr::null() {
                let mut keyword: *mut KeywordExt = temp.first();

                let mut count = 0;
                let i = 0;

                while i < keyword._selchars_length {
                    if (keyword._selchars[i] == c) {
                        count = count + 1;
                    }
                    i = i + 1;
                }
                split_cardinalities[count] = split_cardinalities[count] + 1;
                temp = temp.rest();
            }
            sum = sum + (cls._cardinality * cls._cardinality);
            let i = 0;

            while i < m {
                sum = sum - (split_cardinalities[i] * split_cardinalities[i]);
            }
            cls = cls.next;
        }

        drop(split_cardinalities);

        return sum;
    }

    pub fn unchanged_partition(mut partition: *mut EquivalenceClass, mut c: u32) -> bool {
        let cls: *mut EquivalenceClass = partition;
        while cls != ptr::null() {
            let first_count = u32::MAX;

            let mut temp: *mut KeywordExt_List = cls._keywords;

            while temp != ptr::null() {
                let mut keyword: *mut KeywordExt = temp.first();

                let mut count = 0;
                let i = 0;

                while i < keyword._selchars_length {
                    if (keyword._selchars[i] == c) {
                        count = count + 1;
                    }
                    i = i + 1;
                }

                if (temp == cls._keywords) {
                    first_count = count;
                } else if (count != first_count)
                /* c would split this equivalence class.  */
                {
                    return false;
                }
                temp = temp.rest()
            }

            cls = cls.next;
        }
        return true;
    }

    pub fn find_asso_values() {
        //TODO:
        todo!()
    }

    #[inline]
    pub fn compute_hash(mut keyword: *mut KeywordExt) -> i32 {
        let mut sum = 0;
        if (_hash_includes_len) {
            sum = _allchars_length;
        }

        let mut p: Vec<u32> = keyword._selchars;
        let i = keyword._selchars_length;
        let j = 0;

        while i > 0 {
            sum = sum + _asso_values[p[j]];
            j = j + 1;
            i = i - 1;
        }
        keyword._hash_value = sum;
        return sum;
    }

    pub fn find_good_asso_values() {
        prepare_asso_values();

        /* Search for good _asso_values[].  */
        let asso_iteration: i32;
        if ((asso_iteration = option.get_asso_iterations()) == 0)
        /* Try only the given _initial_asso_value and _jump.  */
        {
            find_asso_values();
        } else {
            /* Try different pairs of _initial_asso_value and _jump, in the
            following order:
            (0, 1)
            (1, 1)
            (2, 1) (0, 3)
            (3, 1) (1, 3)
            (4, 1) (2, 3) (0, 5)
            (5, 1) (3, 3) (1, 5)
            ..... */
            let mut saved_head: *mut KeywordExt_List = _head;
            let mut best_initial_asso_value = 0;
            let mut best_jump = 1;
            let mut best_asso_values: Vec<i32> = vec![0, _alpha_size];
            let mut best_collisions = i32::MAX;
            let mut best_max_hash_value = i32::MAX;

            _initial_asso_value = 0;
            _jump = 1;
            loop {
                /* Restore the keyword list in its original order.  */
                _head = copy_list(saved_head);
                /* Find good _asso_values[].  */
                find_asso_values();
                /* Test whether it is the best solution so far.  */
                let mut collisions = 0;
                let mut max_hash_value = i32::MIN;
                _collision_detector.clear();
                let mut ptr: *mut KeywordExt_List = _head;
                while ptr != ptr::null() {
                    let mut keyword: *mut KeywordExt = ptr.first();
                    let hashcode = compute_hash(keyword);
                    if (max_hash_value < hashcode) {
                        max_hash_value = hashcode;
                    }
                    if (_collision_detector.set_bit(hashcode)) {
                        collisions = collisions + 1;
                    }
                }

                if (collisions < best_collisions
                    || (collisions == best_collisions && max_hash_value < best_max_hash_value))
                {
                    best_asso_values = best_asso_values;
                    // memcpy(
                    //     best_asso_values,
                    //     best_asso_values,
                    //     _alpha_size * sizeof(_asso_values[0]),
                    // );
                    //TODO check: memcpy
                    best_collisions = collisions;
                    best_max_hash_value = max_hash_value;
                }
                /* Delete the copied keyword list.  */
                delete_list(_head);
                asso_iteration = asso_iteration - 1;
                if (asso_iteration == 0) {
                    break;
                }
                /* Prepare for next iteration.  */
                if (_initial_asso_value >= 2) {
                    _initial_asso_value = _initial_asso_value - 2;
                    _jump = _jump + 2;
                } else {
                    _initial_asso_value = _initial_asso_value + _jump;
                    _jump = 1;
                }
            }
            _head = saved_head;
            /* Install the best found asso_values.  */
            //TODO check: memcpy
            _initial_asso_value = best_initial_asso_value;
            _jump = best_jump;
            best_asso_values = _asso_values;
            // _asso_values = vec![best_asso_values, _alpha_size];

            drop(best_asso_values);
            /* The keywords' _hash_value fields are recomputed below.  */
        }
    }

    pub fn sort() {
        _head = mergesort_list(_head, less_by_hash_value);
    }

    pub fn optimize() {
        prepare();

        /* Step 1: Finding good byte positions.  */
        find_positions();

        /* Step 2: Finding good alpha increments.  */
        find_alpha_inc();

        /* Step 3: Finding good asso_values.  */
        find_good_asso_values();

        /* Make one final check, just to make sure nothing weird happened.... */
        _collision_detector.clear();
        let mut curr_ptr: *mut KeywordExt_List = _head;
        while (curr_ptr != ptr::null()) {
            let mut curr: *mut KeywordExt = curr_ptr.first();
            let hashcode: i32 = compute_hash(curr);

            if (_collision_detector.set_bit(hashcode)) {
                /* This shouldn't happen.  proj1, proj2, proj3 must have been
                computed to be injective on the given keyword set.  */
                eprintln!("\nInternal error, unexpected duplicate hash code\n");
                if (option[POSITIONS]) {
                    eprintln!("try options -m or -r, or use new key positions.\n\n",);
                } else {
                    eprintln!("try options -m or -r.\n\n");
                }
                process::exit(1);
            }
            curr_ptr = curr_ptr.rest()
        }

        /* Sorts the keyword list by hash value.  */
        sort();

        /* Set unused asso_values[c] to max_hash_value + 1.  This is not absolutely
        necessary, but speeds up the lookup function in many cases of lookup
        failure: no string comparison is needed once the hash value of a string
        is larger than the hash value of any keyword.  */
        let mut max_hash_value: i32;
        {
            let mut temp: *mut KeywordExt_List = _head;

            while temp.rest() != ptr::null() {
                temp = temp.rest()
            }

            max_hash_value = temp.first()._hash_value;
        }
        let c:u32 = 0;
        while c < _alpha_size {
            if (_occurrences[c] == 0) {
                _asso_values[c] = max_hash_value + 1;
            }
            c = c + 1;
        }

        /* Propagate unified asso_values.  */
        if (_alpha_unify) {
            c = 0;
            while c < _alpha_size {
                if (_alpha_unify[c] != c) {
                    _asso_values[c] = _asso_values[_alpha_unify[c]];
                }
                c = c + 1;
            }
        }
    }

    //TODO: destructor for search
}

struct EquivalenceClass {
    _keywords: *mut KeywordExt_List,
    _keywords_last: *mut KeywordExt_List,
    _cardinality: u32,
    _undetermined_chars: Vec<u32>,
    _undetermined_chars_length: u32,
    next: *mut EquivalenceClass,
}

struct Step {
    _changing_count: u32,
    _changing: Vec<u32>,
    _asso_value_max: u32,
    _undetermined: Vec<bool>,
    _partition: *mut EquivalenceClass,
    _expected_lower: f32,
    _expected_upper: f32,
    _next: *mut Step,
}

#[inline]
pub fn equals(ptr1: Vec<u32>, ptr2: Vec<u32>, len: usize) -> bool {
    let j = 0;
    while (len > 0) {
        if (ptr1[j] != ptr2[j]) {
            return false;
        }
        j = j + 1;
        len = len - 1;
    }
    return true;
}

pub fn delete_partition(partition: *mut EquivalenceClass) {
    while partition != ptr::null() {
        let equclass = partition;
        partition = equclass._next;
        delete_list(equclass._keywords);
        //delete[] equclass->_undetermined_chars; // already freed above
        drop(equclass);
    }
}

pub fn less_by_hash_value(mut keyword1: *mut KeywordExt, mut keyword2: *mut KeywordExt) -> bool {
    return keyword1._hash_value < keyword2._hash_value;
}
