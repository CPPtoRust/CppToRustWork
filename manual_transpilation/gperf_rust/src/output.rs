// use std::{f32::consts::E, io};
use std::io::{self, Write};
// #[path = "keyword.rs"]
// mod keyword;
#[path = "keyword_list.rs"]
mod keyword_list;
#[path = "options.rs"]
mod options;
#[path = "positions.rs"]
mod positions;
// use keyword::*;
use keyword_list::*;
use options::*;
use positions::*;

use crate::output::keyword_list::keyword::KeywordExt;
struct Output_Constants;
// struct Output_Compare;
struct Output<'a> {
    pub _head: &'a mut KeywordExt_List<'a>,
    pub _struct_decl: str,
    pub _struct_decl_lineno: u32,
    pub _return_type: str,
    pub _struct_tag: str,
    pub _wordlist_eltype: str,
    pub _verbatim_declarations: str,
    pub _verbatim_declarations_end: str,
    pub _verbatim_declarations_lineno: u32,

    pub _verbatim_code: str,
    pub _verbatim_code_end: str,
    pub _verbatim_code_lineno: u32,

    pub _charset_dependent: bool,
    pub _total_keys: i32,
    pub _max_key_len: i32,
    pub _min_key_len: i32,
    pub _hash_includes_len: bool,

    pub _key_positions: Positions,
    pub _alpha_inc: Vec<u32>,
    pub _total_duplicates: i32,
    pub _min_hash_value: i32,
    pub _max_hash_value: i32,
    pub _alpha_size: u32,
    pub _asso_values: Vec<i32>,
}

/* The "const " qualifier.  */
static const_always: &str = "";

/* The "const " qualifier, for read-only arrays.  */
static const_readonly_array: &str = "";

/* The "const " qualifier, for the array type.  */
static const_for_struct: &str = "";

// struct Output_Constants;

//MACROS
const UCHAR_MAX: u32 = 255;
const USHRT_MAX: u32 = 65535;
const SCHAR_MIN: i32 = -128;
const SCHAR_MAX: u32 = 127;
const SHRT_MIN: i32 = -32768;
const SHRT_MAX: i32 = 32767;

/* Returns the smallest unsigned C type capable of holding integers
up to N.  */
pub fn smallest_integral_type(n: u32) -> &'static str {
    if n <= UCHAR_MAX {
        return "unsigned char";
    }
    if n <= USHRT_MAX {
        return "unsigned short";
    }
    return "unsigned int";
}

//change the overloaded function names
pub fn smallest_integral_type2(min: i32, max: i32) -> &'static str {
    if (option[ANSIC as usize] | option[CPLUSPLUS as usize]) {
        if (min >= SCHAR_MIN && max <= SCHAR_MAX) {
            return "signed char";
        }
    }
    if (min >= SHRT_MIN && max <= SHRT_MAX) {
        return "short";
    }
    return "int";
}

impl<'a> Output<'a> {
    pub fn compute_min_max(&mut self) {
        self._min_hash_value = self._head.first().unwrap()._hash_value;
        let temp: Option<&mut KeywordExt_List> = Some(self._head);

        while !temp.unwrap().rest().is_none() {
            temp = *temp.unwrap().rest();
        }
        self._max_hash_value = temp.unwrap().first().unwrap()._hash_value;
    }

    pub fn num_hash_values(&mut self) -> i32 {
        let count: i32 = 0;
        let temp: Option<&mut KeywordExt_List> = Some(self._head);

        while !temp.unwrap().rest().is_none() {
            count += 1;
            temp = *temp.unwrap().rest();
        }

        return count;
    }

    pub fn output_constants(&mut self, &style: &Output_Constants) {
        style.output_start();
        style.output_item("TOTAL_KEYWORDS", self._total_keys);
        style.output_item("MIN_WORD_LENGTH", self._min_key_len);
        style.output_item("MAX_WORD_LENGTH", self._max_key_len);
        style.output_item("MIN_HASH_VALUE", self._min_hash_value);
        style.output_item("MAX_HASH_VALUE", self._max_hash_value);
        style.output_end();
    }

    pub fn output_asso_values_ref(&mut self, pos: i32) {
        print!("asso_values[");
        /* Always cast to unsigned char.  This is necessary when the alpha_inc
        is nonzero, and also avoids a gcc warning "subscript has type 'char'".  */
        print!("(unsigned char)");
        if (pos == Positions::LASTCHAR) {
            print!("str[len - 1]");
        } else {
            print!("str[{}]", pos);
            if self._alpha_inc[pos as usize] != 0 {
                print!("+{}", self._alpha_inc[pos as usize]);
            }
        }
        print!("]");
    }

    pub fn output_hash_function(&mut self) {
        /* Output the function's head.  */
        if (option[CPLUSPLUS]) {
            print!("inline ");
        } else if (option[KRC] | option[C] | option[ANSIC]) {
            print!(
                "{}{}{}{}{}{}{}",
                "#ifdef __GNUC__\n",
                "__inline\n",
                "#else\n",
                "#ifdef __cplusplus\n",
                "inline\n",
                "#endif\n",
                "#endif\n"
            );
        }
        if (self._key_positions.get_size() == 0
            || (!self._hash_includes_len
                && self._key_positions[0] < self._min_key_len
                && self._key_positions[(self._key_positions.get_size() - 1) as usize]
                    != Positions::LASTCHAR))
        {
            print!("/*ARGSUSED*/\n");
        }

        if (option[KRC] || option[C] || option[ANSIC]) {
            print!("static ");
        }
        print!("unsigned int\n");
        if (option[CPLUSPLUS]) {
            print!("{}::", option.get_class_name());
        }
        print!("{} ", option.get_hash_name());

        if (option[KRC]) {
            print!(
                "{}{}{}",
                "(str, len)\n", "     register char *str;\n", "     register unsigned int len;\n"
            );
        } else if (option[C]) {
            print!(
                "{}{}{}",
                "(str, len)\n",
                "     register const char *str;\n",
                "     register unsigned int len;\n"
            );
        } else if (option[ANSIC] || option[CPLUSPLUS]) {
            print!("(register const char *str, register unsigned int len)\n");
        } else {
            print!("");
        }

        print!("{{\n");

        if (self._key_positions.get_size() > 0) {
            print!(
                "  static {}{} asso_values[] =\n{}",
                const_readonly_array, //E
                smallest_integral_type((self._max_hash_value + 1) as u32),
                "    {"
            );

            let columns: i32 = 10;
            let mut field_width: i32 = 2;
            let mut trunc: i32 = self._max_hash_value;

            while trunc > 0 {
                trunc = trunc / 10;
                field_width = field_width + 1;
            }

            let count = 0;
            while count < self._alpha_size {
                if (count > 0) {
                    print!(",");
                }
                if (count % (columns as u32)) == 0 {
                    print!("\n     ");
                }
                print!("{}{}", field_width, self._asso_values[count as usize]);
            }
            print!("{}{}", "\n", "    };\n");
        }

        if (self._key_positions.get_size() == 0) {
            if (self._hash_includes_len) {
                print!("  return {{}};\nlen");
            } else {
                print!("  return {{}};\n0");
            }
        } else {
            let mut iter: PositionIterator = self._key_positions.iterator();

            let mut key_pos: i32 = iter.next();
            if (key_pos == Positions::LASTCHAR || key_pos < self._min_key_len) {
                if (self._hash_includes_len) {
                    print!("  return len + ");
                } else {
                    print!("  return ");
                }

                if (self._key_positions.get_size() == 2
                    && self._key_positions[0] == 0
                    && self._key_positions[1] == Positions::LASTCHAR)
                {
                    self.output_asso_values_ref(Positions::LASTCHAR);
                    print!(" + ");
                    self.output_asso_values_ref(0);
                } else {
                    while key_pos != Positions::LASTCHAR {
                        self.output_asso_values_ref(key_pos);
                        key_pos = iter.next();
                        if (key_pos != PositionIterator::EOS) {
                            print!(" + ");
                        } else {
                            break;
                        }
                    }
                    if (key_pos == Positions::LASTCHAR) {
                        self.output_asso_values_ref(Positions::LASTCHAR);
                    }
                }
                print!(";\n");
            } else {
                if (self._hash_includes_len) {
                    print!(
                        "{}{}{}{}",
                        "  register int hval = len;\n\n",
                        "  switch (hval)\n",
                        "    {\n",
                        "      default:\n"
                    );
                } else {
                    print!(
                        "{}{}{}{}",
                        "  register int hval = 0;\n\n",
                        "  switch (len)\n",
                        "    {\n",
                        "      default:\n"
                    );
                }

                while key_pos != Positions::LASTCHAR && key_pos >= self._max_key_len {
                    key_pos = iter.next();
                    if (key_pos == PositionIterator::EOS) {
                        break;
                    }
                }
                if (key_pos != PositionIterator::EOS && key_pos != Positions::LASTCHAR) {
                    let mut i = key_pos;
                    loop {
                        if (i > key_pos) {
                            print!("      /*FALLTHROUGH*/\n"); /* Pacify lint.  */
                        }
                        while i > key_pos {
                            print!("      case {}:\n", i);
                            i = i - 1;
                        }

                        print!("        hval += ");
                        self.output_asso_values_ref(key_pos);
                        print!(";\n");

                        key_pos = iter.next();
                        if !(key_pos != PositionIterator::EOS && key_pos != Positions::LASTCHAR) {
                            break;
                        }
                    }

                    if (i >= self._min_key_len) {
                        print!("      /*FALLTHROUGH*/\n"); /* Pacify lint.  */
                    }
                    while i >= self._min_key_len {
                        print!("      case {}:\n", i);
                        i = i - 1;
                    }
                }

                print!("{}{}{}", "        break;\n", "    }\n", "  return hval");
                if (key_pos == Positions::LASTCHAR) {
                    print!(" + ");
                    self.output_asso_values_ref(Positions::LASTCHAR);
                }
                print!(";\n");
            }
        }
        print!("}\n\n");
    }

    pub fn output_keylength_table(&mut self) {
        let columns = 14;
        let indent: &str;

        if (option[GLOBAL]) {
            indent = "";
        } else {
            indent = " ";
        }

        print!(
            "{}static {}{} {}[] =\n{}  {{",
            indent,
            const_readonly_array,
            smallest_integral_type(self._max_key_len as u32),
            option.get_lengthtable_name(),
            indent
        );

        let mut index: i32 = 0;
        let mut column: i32 = 0;

        let temp: Option<&mut KeywordExt_List> = Some(self._head);

        while !temp.is_none() {
            let keyword: &mut KeywordExt = temp.unwrap().first().unwrap();

            if (option[SWITCH] && !option[TYPE] && !keyword._duplicate_link) {
                continue;
            }
            if (index < keyword._hash_value && !option[SWITCH] && !option[DUP]) {
                /* Some blank entries.  */
                while index < keyword._hash_value {
                    if (index > 0) {
                        print!(",");
                    }
                    if ((column % columns) == 0) {
                        print!("\n{}   ", indent);
                    }
                    column = column + 1;
                    print!("{}", 0);
                    index = index + 1;
                }
            }
            if (index > 0) {
                print!(",");
            }
            if ((column % columns) == 0) {
                print!("\n{}   ", indent);
            }
            column = column + 1;
            print!("{}", keyword.keyword._allchars_length);
            index = index + 1;

            if !keyword._duplicate_link.is_none() {
                let links: Option<&mut KeywordExt> = Some(keyword._duplicate_link.unwrap());
                while !links.is_none() {
                    print!(",");
                    if ((column % columns) == 0) {
                        print!("\n{}   ", indent);
                    }
                    column = column + 1;
                    print!("{}", links.unwrap().keyword._allchars_length);
                    index = index + 1;
                    links = Some(links.unwrap()._duplicate_link.unwrap());
                }
            }
            temp = Some(temp.unwrap().rest().unwrap());
        }

        print!("\n{}  }};\n", indent);
        if (option[GLOBAL]) {
            print!("\n");
        }
    }

    pub fn output_string_pool(&mut self) {
        let mut indent: &str;
        if (option[TYPE] || option[GLOBAL]) {
            indent = "";
        } else {
            indent = " ";
        }
        let mut index: i32 = 0;
        let temp: Option<&mut KeywordExt_List> = Some(self._head);
        print!(
            "{}struct {}_t\n{}  {{\n",
            indent,
            option.get_stringpool_name(),
            indent
        );
        while !temp.is_none() {
            let keyword: &mut KeywordExt = temp.unwrap().first().unwrap(); //E

            if option[SWITCH] && !option[TYPE] && !keyword._duplicate_link.is_none() {
                continue;
            }

            if (!option[SWITCH] && !option[DUP]) {
                index = keyword._hash_value;
            }

            print!(
                "{}    char {}_str{}[sizeof(",
                indent,
                option.get_stringpool_name(),
                index
            );
            output_string(keyword.keyword._allchars, keyword.keyword._allchars_length);
            print!(")];\n");

            if !keyword._duplicate_link.is_none() {
                let links: Option<&mut KeywordExt> = Some(keyword._duplicate_link.unwrap());
                while !links.is_none(){
                    if !(links.unwrap().keyword._allchars_length == keyword.keyword._allchars_length
                        && links.unwrap().keyword._allchars.eq(keyword.keyword._allchars))
                    {
                        index = index + 1;
                        print!(
                            "{}    char {}_str{}[sizeof(",
                            indent,
                            option.get_stringpool_name(),
                            index
                        );
                        output_string(links.unwrap().keyword._allchars, links.unwrap().keyword._allchars_length);
                        print!(")];\n");
                    }

                    links = Some(links.unwrap()._duplicate_link.unwrap());
                }
            }
            index = index + 1;
            temp = Some(temp.unwrap().rest().unwrap());
        }
        print!("{}  }};\n", indent);
        print!(
            "{}static {}struct {}_t {}_contents =\n{}  {{\n",
            indent,
            const_readonly_array,
            option.get_stringpool_name(),
            option.get_stringpool_name(),
            indent
        );
        temp = Some(self._head);
        index = 0;

        while !temp.is_none() {
            let keyword: &mut KeywordExt = temp.unwrap().first();

            if (option[SWITCH] && !option[TYPE] && !keyword._duplicate_link) {
                continue;
            }
            if (index > 0) {
                print!(",\n");
            }

            if (!option[SWITCH] && !option[DUP]) {
                index = keyword._hash_value;
            }
            print!("{}    ", indent);
            output_string(keyword.keyword._allchars, keyword.keyword._allchars_length);

            if !keyword._duplicate_link.is_none() {
                let links: Option<&mut KeywordExt> = Some(keyword._duplicate_link.unwrap());
                while !links.is_none(){
                    if !(links.unwrap().keyword._allchars_length == keyword.keyword._allchars_length
                        && links.unwrap().keyword._allchars.eq(keyword.keyword._allchars))
                    {
                        index = index + 1;
                        print!(",\n");
                        print!("{}    ", indent);
                        output_string(links.unwrap().keyword._allchars, links.unwrap().keyword._allchars_length);
                    }
                }
            }
            index = index + 1;
        }
        if index > 0 {
            print!("\n");
        }
        print!("{}  }};\n", indent);
        print!(
            "{}#define {} (({}char *) &{}_contents)\n",
            indent,
            option.get_stringpool_name(),
            const_always,
            option.get_stringpool_name()
        );
        if (option[GLOBAL]) {
            print!("\n");
        }
    }

    pub fn output_keyword_table(&mut self) {
        //E the KeywordExt struct
        let indent: &str;
        if (option[GLOBAL]) {
            indent = "";
        } else {
            indent = " ";
        }
        let mut index: i32 = 0;
        let temp: Option<&mut KeywordExt_List> = self._head;

        print!("{}static ", indent);
        output_const_type(const_readonly_array, &self._wordlist_eltype);
        print!("{}[] =\n{}  {{\n", option.get_wordlist_name(), indent);

        while !temp.is_none() {
            let keyword: &mut KeywordExt = temp.unwrap().first();

            if (option[SWITCH] && !option[TYPE] && !keyword._duplicate_link) {
                continue;
            }
            if (index > 0) {
                print!(",\n");
            }
            if (index < keyword._hash_value && !option[SWITCH] && !option[DUP]) {
                /* Some blank entries.  */
                output_keyword_blank_entries(keyword._hash_value - index, indent);
                print!(",\n");
                index = keyword._hash_value;
            }
            keyword._final_index = index;
            output_keyword_entry(keyword, index, indent);
            if (keyword._duplicate_link) {
                let links: Option<&mut KeywordExt> = Some(keyword._duplicate_link);
                while !links.is_none() {
                    index = index + 1;
                    links.unwrap()._final_index = index;
                    let mut stringpool_index: i32;
                    if (links.unwrap().keyword._allchars_length == keyword.keyword._allchars_length
                        && links.unwrap().keyword._allchars.eq(keyword.keyword._allchars))
                    {
                        stringpool_index = keyword._final_index;
                    } else {
                        stringpool_index = links.unwrap()._final_index;
                    }
                    output_keyword_entry(links.unwrap(), stringpool_index, indent);
                    links = Some(links.unwrap()._duplicate_link.unwrap());
                }
            }
            index = index + 1;
            temp = Some(temp.unwrap().rest().unwrap());
        }
        if (index > 0) {
            print!("\n");
        }
        print!("{}  }};\n\n", indent);
    }

    pub fn output_lookup_array(&mut self) {
        // TODO 1244 - 13387
    }

    pub fn output_lookup_pools(&mut self) {
        if (option[SWITCH]) {
            if (option[TYPE] || (option[DUP] && self._total_duplicates > 0)) {
                self.output_string_pool();
            }
        } else {
            self.output_string_pool();
        }
    }

    pub fn output_lookup_tables(&mut self) {
        if (option[SWITCH]) {
            /* Use the switch in place of lookup table.  */
            if (option[LENTABLE] && (option[DUP] && self._total_duplicates > 0)) {
                self.output_keylength_table();
            }
            if (option[TYPE] || (option[DUP] && self._total_duplicates > 0)) {
                self.output_keyword_table();
            }
        } else {
            /* Use the lookup table, in place of switch.  */
            if (option[LENTABLE]) {
                self.output_keylength_table();
            }
            self.output_keyword_table();
            self.output_lookup_array();
        }
    }

    pub fn output_lookup_function_body(&mut self, comparison: &Output_Compare) {
        print! ("  if (len <= MAX_WORD_LENGTH && len >= MIN_WORD_LENGTH)\n    {{\n      register int key = {} (str, len);\n\n",
          option.get_hash_name ());

        if (option[SWITCH]) {
            let switch_size = self.num_hash_values();
            let num_switches = option.get_total_switches();
            if (num_switches > switch_size) {
                num_switches = switch_size
            }

            print!("      if (key <= MAX_HASH_VALUE && key >= MIN_HASH_VALUE)\n        {\n");
            if (option[DUP] && self._total_duplicates > 0) {
                if (option[LENTABLE]) {
                    print!(
                        "          register {}{} *lengthptr;\n",
                        self.const_always,
                        smallest_integral_type(self._max_key_len)
                    );
                }
                print!("          register ");
                output_const_type(self.const_readonly_array, self._wordlist_eltype);
                print!("*wordptr;\n");
                print!("          register ");
                output_const_type(self.const_readonly_array, self._wordlist_eltype);
                print!("*wordendptr;\n");
            }
            if (option[TYPE]) {
                print!("          register ");
                output_const_type(self.const_readonly_array, self._struct_tag);
                print!("*resword;\n\n");
            } else {
                print!("          register {}resword;\n\n", self._struct_tag);
            }

            output_switches(
                self._head,
                num_switches,
                switch_size,
                self._min_hash_value,
                self._max_hash_value,
                10,
            );

            print!("          return 0;\n");
            if (option[DUP] && self._total_duplicates > 0) {
                let indent = 8;
                print!(
                    "{}{}multicompare:\n{}{}  while (wordptr < wordendptr)\n{}{}    {{\n",
                    indent, "", indent, "", indent, ""
                );
                if (option[LENTABLE]) {
                    print!(
                        "{}{}      if (len == *lengthptr)\n{}{}        {{\n",
                        indent, "", indent, ""
                    );
                    indent = indent + 4;
                }
                print!(
                    "{}{}      register {}char *s = ",
                    indent, "", self.const_always
                );
                if (option[TYPE]) {
                    print!("wordptr->{}", option.get_slot_name());
                } else {
                    print!("*wordptr");
                }
                if (option[SHAREDLIB]) {
                    print!(" + {}", option.get_stringpool_name());
                }
                print!(";\n\n{}{}      if (", indent, "");
                *comparison.output_comparison(
                    Output_Expr1 {
                        output_Expr: Output_Expr {},
                        _p1: "str",
                    },
                    Output_Expr1 {
                        output_Expr: Output_Expr {},
                        _p1: "s",
                    },
                );
                let op: &str;
                if (option[TYPE]) {
                    op = "wordptr";
                } else {
                    op = "s";
                }

                print!(")\n{}{}        return {};\n", indent, "", op);
                if (option[LENTABLE]) {
                    indent = indent - 4;
                    print!("{}{}        }}\n", indent, "");
                }
                if (option[LENTABLE]) {
                    print!("{}{}      lengthptr++;\n", indent, "");
                }
                print!(
                    "{}{}      wordptr++;\n{}{}    }}\n{}{}  return 0;\n",
                    indent, "", indent, "", indent, ""
                );
            }
            print!("        compare:\n");
            if (option[TYPE]) {
                print!(
                    "          {{\n            register {}char *s = resword->{}",
                    self.const_always,
                    option.get_slot_name()
                );
                if (option[SHAREDLIB]) {
                    print!(" + {}", option.get_stringpool_name());
                }
                print!(";\n\n            if (");
                comparison.output_comparison(
                    Output_Expr1 {
                        output_Expr: Output_Expr {},
                        _p1: "str",
                    },
                    Output_Expr1 {
                        output_Expr: Output_Expr {},
                        _p1: "s",
                    },
                );
                print!(")\n              return resword;\n          }}\n");
            } else {
                print!("          if (");
                comparison.output_comparison(
                    Output_Expr1 {
                        output_Expr: Output_Expr {},
                        _p1: "str",
                    },
                    Output_Expr1 {
                        output_Expr: Output_Expr {},
                        _p1: "resword",
                    },
                );
                print!(")\n            return resword;\n");
            }
            print!("        }}\n");
        } else {
            print!("      if (key <= MAX_HASH_VALUE && key >= 0)\n");

            if (option[DUP]) {
                let mut indent = 8;
                print!(
                    "{}{}{{\n{}{}  register int index = lookup[key];\n\n{}{}  if (index >= 0)\n",
                    indent, "", indent, "", indent, ""
                );
                if (option[LENTABLE]) {
                    print!(
                        "{}{}    {{\n{}{}      if (len == {}[index])\n",
                        indent,
                        "",
                        indent,
                        "",
                        option.get_lengthtable_name()
                    );
                    indent = indent + 4;
                }
                print!(
                    "{}{}    {{\n{}{}      register {}char *s = {}[index]",
                    indent,
                    "",
                    indent,
                    "",
                    const_always,
                    option.get_wordlist_name()
                );
                if (option[TYPE]) {
                    print!(".{}", option.get_slot_name());
                }
                if (option[SHAREDLIB]) {
                    print!(" + {}", option.get_stringpool_name());
                }
                print!(";\n\n{}{}      if (", indent, "");
                comparison.output_comparison(
                    Output_Expr1 {
                        output_Expr: Output_Expr {},
                        _p1: "str",
                    },
                    Output_Expr1 {
                        output_Expr: Output_Expr {},
                        _p1: "s",
                    },
                );
                print!(")\n{}{}        return ", indent, "");
                if (option[TYPE]) {
                    print!("&{}[index]", option.get_wordlist_name());
                } else {
                    print!("s");
                }
                print!(";\n{}{}    }}\n", indent, "");
                if (option[LENTABLE]) {
                    indent = indent - 4;
                    print!("{}{}    }}\n", indent, "");
                }
                if (self._total_duplicates > 0) {
                    print! ("{}{}  else if (index < -TOTAL_KEYWORDS)\n{}{}    {{\n{}{}      register int offset = - 1 - TOTAL_KEYWORDS - index;\n",
                      indent, "", indent, "", indent, "");
                    if (option[LENTABLE]) {
                        print! ("{}{}      register {}{} *lengthptr = &{}[TOTAL_KEYWORDS + lookup[offset]];\n",
                        indent, "", self.const_always, smallest_integral_type (self._max_key_len),
                        option.get_lengthtable_name ());
                    }
                    print!("{}{}      register ", indent, "");
                    output_const_type(self.const_readonly_array, self._wordlist_eltype);
                    print!(
                        "*wordptr = &{}[TOTAL_KEYWORDS + lookup[offset]];\n",
                        option.get_wordlist_name()
                    );
                    print!("{}{}      register ", indent, "");
                    output_const_type(self.const_readonly_array, self._wordlist_eltype);
                    print!("*wordendptr = wordptr + -lookup[offset + 1];\n\n");
                    print!(
                        "{}{}      while (wordptr < wordendptr)\n{}{}        {{\n",
                        indent, "", indent, ""
                    );
                    if (option[LENTABLE]) {
                        print!(
                            "{}{}          if (len == *lengthptr)\n{}{}            {{\n",
                            indent, "", indent, ""
                        );
                        indent = indent + 4;
                    }
                    print!(
                        "{}{}          register {}char *s = ",
                        indent, "", self.const_always
                    );
                    if (option[TYPE]) {
                        print!("wordptr->{}", option.get_slot_name());
                    } else {
                        print!("*wordptr");
                    }
                    if (option[SHAREDLIB]) {
                        print!(" + {}", option.get_stringpool_name());
                    }
                    print!(";\n\n{}{}          if (", indent, "");
                    comparison.output_comparison(
                        Output_Expr1 {
                            output_Expr: Output_Expr {},
                            _p1: "str",
                        },
                        Output_Expr1 {
                            output_Expr: Output_Expr {},
                            _p1: "s",
                        },
                    );
                    print!(
                        ")\n{}{}            return {};\n",
                        indent,
                        "",
                        if option[TYPE] { "wordptr" } else { "s" }
                    );
                    if (option[LENTABLE]) {
                        indent = indent - 4;
                        print!("{}{}            }}\n", indent, "");
                    }
                    if (option[LENTABLE]) {
                        print!("{}{}          lengthptr++;\n", indent, "");
                    }
                    print!(
                        "{}{}          wordptr++;\n{}{}        }}\n{}{}    }\n",
                        indent, "", indent, "", indent, ""
                    );
                }
                print!("{}{}}\n", indent, "");
            } else {
                let mut indent = 8;
                if (option[LENTABLE]) {
                    print!(
                        "{}{}if (len == {}[key])\n",
                        indent,
                        "",
                        option.get_lengthtable_name()
                    );
                    indent = indent + 2;
                }

                if (option[SHAREDLIB]) {
                    if (!option[LENTABLE]) {
                        print!(
                            "{}{}{{\n{}{}  register int o = {}[key]",
                            indent,
                            "",
                            indent,
                            "",
                            option.get_wordlist_name()
                        );
                        if (option[TYPE]) {
                            print!(".{}", option.get_slot_name());
                        }
                        print!(";\n{}{}  if (o >= 0)\n{}{}    {{\n", indent, "", indent, "");
                        indent = indent + 4;
                        print!(
                            "{}{}  register {}char *s = o",
                            indent, "", self.const_always
                        );
                    } else {
                        /* No need for the (o >= 0) test, because the
                        (len == lengthtable[key]) test already guarantees that
                        key points to nonempty table entry.  */
                        print!(
                            "{}{}{{\n{}{}  register {}char *s = {}[key]",
                            indent,
                            "",
                            indent,
                            "",
                            self.const_always,
                            option.get_wordlist_name()
                        );
                        if (option[TYPE]) {
                            print!(".{}", option.get_slot_name());
                        }
                    }
                    print!(" + {}", option.get_stringpool_name());
                } else {
                    print!(
                        "{}{}{{\n{}{}  register {}char *s = {}[key]",
                        indent,
                        "",
                        indent,
                        "",
                        self.const_always,
                        option.get_wordlist_name()
                    );
                    if (option[TYPE]) {
                        print!(".{}", option.get_slot_name());
                    }
                }

                print!(";\n\n{}{}  if (", indent, "");
                if (!option[SHAREDLIB] && option[NULLSTRINGS]) {
                    print!("s && ");
                }
                comparison.output_comparison(
                    Output_Expr1 {
                        output_Expr: Output_Expr {},
                        _p1: "str",
                    },
                    Output_Expr1 {
                        output_Expr: Output_Expr {},
                        _p1: "s",
                    },
                );
                print!(")\n{}{}    return ", indent, "");
                if (option[TYPE]) {
                    print!("&{}[key]", option.get_wordlist_name());
                } else {
                    print!("s");
                }
                print!(";\n");
                if (option[SHAREDLIB] && !option[LENTABLE]) {
                    indent = indent - 4;
                    print!("{}{}    }}\n", indent, "");
                }
                print!("{}{}}}\n", indent, "");
            }
        }
        print!("    }}\n  return 0;\n");
    }

    pub fn output_lookup_function(&mut self) {
        if (option[KRC] | option[C] | option[ANSIC]) {
            print! ("#ifdef __GNUC__\n__inline\n#if defined __GNUC_STDC_INLINE__ || defined __GNUC_GNU_INLINE__\n__attribute__ ((__gnu_inline__))\n#endif\n#endif\n");
        }

        print!("{}{}\n", self.const_for_struct, self._return_type);
        if (option[CPLUSPLUS]) {
            print!("{}::", option.get_class_name());
        }
        print!("{} ", option.get_function_name());
        if (option[KRC]) {
            print!(
                "{}{}{}",
                "(str, len)\n", "     register char *str;\n", "     register unsigned int len;\n"
            );
        } else if (option[C]) {
            print!(
                "{}{}{}",
                "(str, len)\n",
                "     register const char *str;\n",
                "     register unsigned int len;\n"
            );
        } else if (option[ANSIC] | option[CPLUSPLUS]) {
            print!("(register const char *str, register unsigned int len)\n");
        }

        /* Output the function's body.  */
        print!("{{\n");

        if (option[ENUM] && !option[GLOBAL]) {
            let style: Output_Enum = Output_Enum {
                output_Constants: Output_Constants {},
                _indentation: "  ",
                _pending_comma: false,
            };
            self.output_constants(style.output_Constants);
        }

        if (option[SHAREDLIB] && !(option[GLOBAL] || option[TYPE])) {
            self.output_lookup_pools();
        }
        if (!option[GLOBAL]) {
            self.output_lookup_tables();
        }

        if (option[LENTABLE]) {
            self.output_lookup_function_body(Output_Compare_Memcmp {
                output_Compare: Output_Compare {},
            });
        } else {
            if (option[COMP]) {
                self.output_lookup_function_body(Output_Compare_Strncmp {});
            } else {
                self.output_lookup_function_body(Output_Compare_Strcmp {});
            }
        }

        print!("}}\n");
    }

    pub fn output(&mut self) {
        self.compute_min_max();
        if (option[C] | option[ANSIC] | option[CPLUSPLUS]) {
            self.const_always = "const ";
            if (option[CONST]) {
                const_readonly_array = "const ";
            } else {
                const_readonly_array = "";
            }
            if (option[CONST] && option[TYPE]) {
                const_for_struct = "const ";
            } else {
                const_for_struct = "";
            }
        } else {
            const_always = "";
            const_readonly_array = "";
            const_for_struct = "";
        }

        if (!option[TYPE]) {
            if (self.const_always[0]) {
                self._return_type = "const char *";
                self._struct_tag = "const char *";
            } else {
                self._return_type = "char *";
                self._struct_tag = "char *";
            }
        }
        if (option[SHAREDLIB] && !option[TYPE]) {
            self._wordlist_eltype = "int";
        } else {
            self._wordlist_eltype = self._struct_tag;
        }

        print!("/* ");
        if (option[KRC]) {
            print!("KR-C");
        } else if (option[C]) {
            print!("C");
        } else if (option[ANSIC]) {
            print!("ANSI-C");
        } else if (option[CPLUSPLUS]) {
            print!("C++");
        }
        print!(
            " code produced by gperf version {} */\n",
            version_string //what this E
        );
        option.print_options();
        print!("\n");
        if (!option[POSITIONS]) {
            print!("/* Computed positions: -k'");
            self._key_positions.print();
            print!("' */\n");
        }
        print!("\n");

        if (self._charset_dependent && (self._key_positions.get_size() > 0 || option[UPPERLOWER])) {
            //TODO : check this
            // print! ("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            // "#if !((' ' == 32) && ('!' == 33) && ('\"' == 34) && ('#' == 35) \n",
            //   "      && ('%%' == 37) && ('&' == 38) && ('\\'' == 39) && ('(' == 40) \n",
            //   "      && (')' == 41) && ('*' == 42) && ('+' == 43) && (',' == 44) \n",
            //   "      && ('-' == 45) && ('.' == 46) && ('/' == 47) && ('0' == 48) \n",
            //   "      && ('1' == 49) && ('2' == 50) && ('3' == 51) && ('4' == 52) \n",
            //   "      && ('5' == 53) && ('6' == 54) && ('7' == 55) && ('8' == 56) \n",
            //   "      && ('9' == 57) && (':' == 58) && (';' == 59) && ('<' == 60) \n",
            //   "      && ('=' == 61) && ('>' == 62) && ('?' == 63) && ('A' == 65) \n",
            //   "      && ('B' == 66) && ('C' == 67) && ('D' == 68) && ('E' == 69) \n",
            //   "      && ('F' == 70) && ('G' == 71) && ('H' == 72) && ('I' == 73) \n",
            //   "      && ('J' == 74) && ('K' == 75) && ('L' == 76) && ('M' == 77) \n",
            //   "      && ('N' == 78) && ('O' == 79) && ('P' == 80) && ('Q' == 81) \n",
            //   "      && ('R' == 82) && ('S' == 83) && ('T' == 84) && ('U' == 85) \n",,
            //   "      && ('V' == 86) && ('W' == 87) && ('X' == 88) && ('Y' == 89) \n",
            //   "      && ('Z' == 90) && ('[' == 91) && ('\\\\' == 92) && (']' == 93) \n",
            //   "      && ('^' == 94) && ('_' == 95) && ('a' == 97) && ('b' == 98) \n",
            //   "      && ('c' == 99) && ('d' == 100) && ('e' == 101) && ('f' == 102) \n",
            //   "      && ('g' == 103) && ('h' == 104) && ('i' == 105) && ('j' == 106) \n",
            //   "      && ('k' == 107) && ('l' == 108) && ('m' == 109) && ('n' == 110) \n",
            //   "      && ('o' == 111) && ('p' == 112) && ('q' == 113) && ('r' == 114) \n",
            //   "      && ('s' == 115) && ('t' == 116) && ('u' == 117) && ('v' == 118) \n",
            //   "      && ('w' == 119) && ('x' == 120) && ('y' == 121) && ('z' == 122) \n",
            //   "      && ('{' == 123) && ('|' == 124) && ('}' == 125) && ('~' == 126))\n",
            //   "/* The character set is not based on ISO-646.  */\n");

            let st: &str;
            if (option[KRC] || option[C]) {
                st = "error";
            } else {
                st = "#error";
            }
            print! ("{} \"gperf generated tables don't work with this execution character set. Please report a bug to <bug-gnu-gperf@gnu.org>.\"\n", s);
            print!("#endif\n\n");

            if (self._verbatim_declarations < self._verbatim_declarations_end) {
                output_line_directive(self._verbatim_declarations_lineno);
                fwrite(
                    self._verbatim_declarations,
                    1,
                    self._verbatim_declarations_end - self._verbatim_declarations,
                    self.stdout,
                );
            }

            if (option[TYPE] && !option[NOTYPE])
            /* Output type declaration now, reference it later on.... */
            {
                output_line_directive(self._struct_decl_lineno);
                print!("{}\n", self._struct_decl);
            }

            if (option[INCLUDE]) {
                print!("#include <string.h>\n");
            } /* Declare strlen(), strcmp(), strncmp(). */

            if (!option[ENUM]) {
                let style: Output_Defines;
                self.output_constants(style);
            } else if (option[GLOBAL]) {
                let style: Output_Enum = Output_Enum {
                    output_Constants: Output_Constants {},
                    _indentation: "",
                    _pending_comma: false,
                };
                self.output_constants(style);
            }

            print!(
                "/* maximum key range = {}, duplicates = {} */\n\n",
                self._max_hash_value - self._min_hash_value + 1,
                self._total_duplicates
            );

            if (option[UPPERLOWER]) {
                if USE_DOWNCASE_TABLE {
                    output_upperlower_table();
                }

                if (option[LENTABLE]) {
                    output_upperlower_memcmp();
                } else {
                    if (option[COMP]) {
                        output_upperlower_strncmp();
                    } else {
                        output_upperlower_strcmp();
                    }
                }
            }

            if (option[CPLUSPLUS]) {
                print! ("class {}\n{{\nprivate:\n  static inline unsigned int {} (const char *str, unsigned int len);\npublic:\n  static {}{}{} (const char *str, unsigned int len);\n};\n\n",
            option.get_class_name (), option.get_hash_name (),
            const_for_struct, _return_type, option.get_function_name ());
            }

            self.output_hash_function();

            if (option[SHAREDLIB] && (option[GLOBAL] || option[TYPE])) {
                self.output_lookup_pools();
            }
            if (option[GLOBAL]) {
                self.output_lookup_tables();
            }

            self.output_lookup_function();

            if (self._verbatim_code < self._verbatim_code_end) {
                output_line_directive(self._verbatim_code_lineno);
                fwrite(
                    //E
                    self._verbatim_code,
                    1,
                    self._verbatim_code_end - self._verbatim_code,
                    self.stdout,
                );
            }
            print!("{}", self.stdout);
            io::stdout().flush().unwrap();
            // fflush(self.stdout);//E
        }
    }
}

struct Output_Defines {
    pub output_Constants: Option<Output_Constants>,
}

impl Output_Defines {
    pub fn output_start() {
        print!("\n");
    }

    pub fn output_item(name: &str, value: i32) {
        print!("#define {} {}\n", name, value);
    }

    pub fn output_end() {}
}

struct Output_Enum {
    pub output_Constants: Option<Output_Constants>,
    _indentation: str,
    _pending_comma: bool,
}

impl Output_Enum {
    pub fn output_start(&mut self) {
        print!("{}enum\n{}  {{\n", self._indentation, self._indentation);
        self._pending_comma = false;
    }

    pub fn output_item(&mut self, name: &str, value: i32) {
        if (self._pending_comma) {
            print!(",\n");
        }
        print!("{}    {} = {}", self._indentation, name, value);
        self._pending_comma = true;
    }

    pub fn output_end(&mut self) {
        if (self._pending_comma) {
            print!("\n");
        }
        print!("{}  }};\n\n", self._indentation);
    }
}

const USE_DOWNCASE_TABLE: u32 = 1;

pub fn output_upperlower_table() {
    if (!USE_DOWNCASE_TABLE) {
        return;
    }
    let c: u32 = 0;
    print!(
        "{}{}{}{}",
        "#ifndef GPERF_DOWNCASE\n",
        "#define GPERF_DOWNCASE 1\n",
        "static unsigned char gperf_downcase[256] =\n",
        "  {"
    );
    while c < 256 {
        if ((c % 15) == 0) {
            print!("\n   ");
        }
        // print!(" {}", c >= 'A' && c <= 'Z' ? c + 'a' - 'A' : c);
        if (c < 255) {
            print!(",");
        }
    }
    print!("\n  }};\n#endif\n\n");
}

pub fn output_upperlower_strcmp() {
    print!(
        "{}{}{}{}",
        "#ifndef GPERF_CASE_STRCMP\n",
        "#define GPERF_CASE_STRCMP 1\n",
        "static int\n",
        "gperf_case_strcmp "
    );

    if (option[KRC]) {
        print!(
            "{}",
            "(s1, s2)\n     register char *s1;\n     register char *s2;\n"
        );
    } else {
        if (option[C]) {
            print!(
                "{}",
                "(s1, s2)\n     register const char *s1;\n     register const char *s2;\n"
            );
        } else {
            if (option[ANSIC] || option[CPLUSPLUS]) {
                print!("(register const char *s1, register const char *s2)\n");
            } else {
                print!("");
            }
        }
    }

    if (USE_DOWNCASE_TABLE) {
        print!(
            "{}{}{}{}{}{}{}{}{}{}",
            "{\n",
            "  for (;;)\n",
            "    {\n",
            "      unsigned char c1 = gperf_downcase[(unsigned char)*s1++];\n",
            "      unsigned char c2 = gperf_downcase[(unsigned char)*s2++];\n",
            "      if (c1 != 0 && c1 == c2)\n",
            "        continue;\n",
            "      return (int)c1 - (int)c2;\n",
            "    }\n",
            "}\n"
        );
    } else {
        print!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "{{\n",
            "  for (;;)\n",
            "    {\n",
            "      unsigned char c1 = *s1++;\n",
            "      unsigned char c2 = *s2++;\n",
            "      if (c1 >= 'A' && c1 <= 'Z')\n",
            "        c1 += 'a' - 'A';\n",
            "      if (c2 >= 'A' && c2 <= 'Z')\n",
            "        c2 += 'a' - 'A';\n",
            "      if (c1 != 0 && c1 == c2)\n",
            "        continue;\n",
            "      return (int)c1 - (int)c2;\n",
            "    }\n",
            "}\n"
        );
    }
    print!("#endif\n\n");
}

pub fn output_upperlower_strncmp() {
    print!(
        "{}{}{}{}",
        "#ifndef GPERF_CASE_STRNCMP\n",
        "#define GPERF_CASE_STRNCMP 1\n",
        "static int\n",
        "gperf_case_strncmp "
    );

    if (option[KRC]) {
        print!(
            "{}{}{}{}",
            "(s1, s2, n)\n",
            "     register char *s1;\n",
            "     register char *s2;\n",
            "     register unsigned int n;\n"
        );
    } else {
        if (option[C]) {
            print!(
                "{}{}{}{}",
                "(s1, s2, n)\n",
                "     register const char *s1;\n",
                "     register const char *s2;\n",
                "     register unsigned int n;\n"
            );
        } else {
            if (option[ANSIC] || option[CPLUSPLUS]) {
                print!(
                    "(register const char *s1, register const char *s2, register unsigned int n)\n"
                );
            } else {
                print!("");
            }
        }
    }

    if (USE_DOWNCASE_TABLE) {
        print!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "{\n",
            "  for (; n > 0;)\n",
            "    {\n",
            "      unsigned char c1 = gperf_downcase[(unsigned char)*s1++];\n",
            "      unsigned char c2 = gperf_downcase[(unsigned char)*s2++];\n",
            "      if (c1 != 0 && c1 == c2)\n",
            "        {\n",
            "          n--;\n",
            "          continue;\n",
            "        }\n",
            "      return (int)c1 - (int)c2;\n",
            "    }\n",
            "  return 0;\n",
            "}\n",
        );
    } else {
        print!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "{\n",
            "  for (; n > 0;)\n",
            "    {\n",
            "      unsigned char c1 = *s1++;\n",
            "      unsigned char c2 = *s2++;\n",
            "      if (c1 >= 'A' && c1 <= 'Z')\n",
            "        c1 += 'a' - 'A';\n",
            "      if (c2 >= 'A' && c2 <= 'Z')\n",
            "        c2 += 'a' - 'A';\n",
            "      if (c1 != 0 && c1 == c2)\n",
            "        {\n",
            "          n--;\n",
            "          continue;\n",
            "        }\n",
            "      return (int)c1 - (int)c2;\n",
            "    }\n",
            "  return 0;\n",
            "}\n"
        );
    }
    print!("#endif\n\n");
}

pub fn output_upperlower_memcmp() {
    print!(
        "{}{}{}{}",
        "#ifndef GPERF_CASE_MEMCMP\n",
        "#define GPERF_CASE_MEMCMP 1\n",
        "static int\n",
        "gperf_case_memcmp "
    );

    if (option[KRC]) {
        print!(
            "{}{}{}{}",
            "(s1, s2, n)\n",
            "     register char *s1;\n",
            "     register char *s2;\n",
            "     register unsigned int n;\n"
        );
    } else {
        if (option[C]) {
            print!(
                "{}{}{}{}",
                "(s1, s2, n)\n",
                "     register const char *s1;\n",
                "     register const char *s2;\n",
                "     register unsigned int n;\n"
            );
        } else {
            if (option[ANSIC] || option[CPLUSPLUS]) {
                print!(
                    "(register const char *s1, register const char *s2, register unsigned int n)\n"
                );
            } else {
                print!("");
            }
        }
    }

    if (USE_DOWNCASE_TABLE) {
        print!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "{\n",
            "  for (; n > 0;)\n",
            "    {\n",
            "      unsigned char c1 = gperf_downcase[(unsigned char)*s1++];\n",
            "      unsigned char c2 = gperf_downcase[(unsigned char)*s2++];\n",
            "      if (c1 == c2)\n",
            "        {\n",
            "          n--;\n",
            "          continue;\n",
            "        }\n",
            "      return (int)c1 - (int)c2;\n",
            "    }\n",
            "  return 0;\n",
            "}\n"
        );
    } else {
        print!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            "{\n",
            "  for (; n > 0;)\n",
            "    {\n",
            "      unsigned char c1 = *s1++;\n",
            "      unsigned char c2 = *s2++;\n",
            "      if (c1 >= 'A' && c1 <= 'Z')\n",
            "        c1 += 'a' - 'A';\n",
            "      if (c2 >= 'A' && c2 <= 'Z')\n",
            "        c2 += 'a' - 'A';\n",
            "      if (c1 == c2)\n",
            "        {\n",
            "          n--;\n",
            "          continue;\n",
            "        }\n",
            "      return (int)c1 - (int)c2;\n",
            "    }\n",
            "  return 0;\n",
            "}\n"
        );
    }
    print!("#endif\n\n");
}

pub fn output_string(key: &str, mut len: i32) {
    print!("{}", '"');
    let key_vec: Vec<char> = key.chars().collect();
    let i = 0;
    while len > 0 {
        let c = key_vec[i];
        if (c.is_printable()) {
            if (c == '"' || c == '\\') {
                print!("{}", '\\');
                print!("{}", c);
            } else {
                print!("{}", '\\');
                print!(
                    "{}",
                    std::char::from_u32(('0' as u32) + ((c as u32) >> 6 & 7)).unwrap_or(c)
                );
                print!(
                    "{}",
                    std::char::from_u32(('0' as u32) + ((c as u32) >> 3 & 7)).unwrap_or(c)
                );
                print!(
                    "{}",
                    std::char::from_u32(('0' as u32) + ((c as u32) & 7)).unwrap_or(c)
                );
            }
        }
        len = len - 1;
    }
    print!("{}", '"');
}

pub fn output_line_directive(lineno: u32) {
    let file_name: str = option.get_input_file_name();

    if (file_name != "") {
        //E
        print!("#line {} ", lineno);
        output_string(file_name, file_name.len());
        print!("\n");
    }
}

pub fn output_const_type(const_string: &str, type_string: &str) {
    if (type_string.chars().last().unwrap() == '*') {
        print!("{} {}", type_string, const_string);
    } else {
        print!("{}{}", const_string, type_string);
    }
}

struct Output_Expr {}

struct Output_Expr1 {
    output_Expr: Output_Expr,
    _p1: str,
}

impl Output_Expr1 {
    pub fn output_expr(&mut self) {
        println!("{}", self._p1);
    }
}

// unused code for Output_Expr2

struct Output_Compare {}

impl Output_Compare {
    pub fn output_firstchar_comparison(
        &mut self,
        expr1: &Output_Expr,
        expr2: &Output_Expr,
    ) -> bool {
        if (option[UPPERLOWER]) {
            /* Incomplete comparison, just for speedup.  */
            print!("(((unsigned char)*");
            expr1.output_expr();
            print!(" ^ (unsigned char)*");
            expr2.output_expr();
            print!(") & ~32) == 0");
            return false;
        } else {
            /* Complete comparison.  */
            print!("*");
            expr1.output_expr();
            print!(" == *");
            expr2.output_expr();
            return true;
        }
    }
}

struct Output_Compare_Strcmp {
    output_Compare: Output_Compare,
}

impl Output_Compare_Strcmp {
    pub fn output_comparison(&mut self, expr1: &Output_Expr, expr2: &Output_Expr) {
        let firstchar_done: bool = self
            .output_Compare
            .output_firstchar_comparison(expr1, expr2);
        print!(" && !");
        if (option[UPPERLOWER]) {
            print!("gperf_case_");
        }
        print!("strcmp (");
        if (firstchar_done) {
            expr1.output_expr();
            print!(" + 1, ");
            expr2.output_expr();
            print!(" + 1");
        } else {
            expr1.output_expr();
            print!(", ");
            expr2.output_expr();
        }
        print!(")");
    }
}

struct Output_Compare_Strncmp {
    output_Compare: Output_Compare,
}

impl Output_Compare_Strncmp {
    pub fn output_comparison(&mut self, expr1: &Output_Expr, expr2: &Output_Expr) {
        let firstchar_done: bool = self
            .output_Compare
            .output_firstchar_comparison(expr1, expr2);
        print!(" && !");
        if (option[UPPERLOWER]) {
            print!("gperf_case_");
        }
        print!("strncmp (");
        if (firstchar_done) {
            expr1.output_expr();
            print!(" + 1, ");
            expr2.output_expr();
            print!(" + 1, len - 1");
        } else {
            expr1.output_expr();
            print!(", ");
            expr2.output_expr();
            print!(", len");
        }
        print!(") && ");
        expr2.output_expr();
        print!("[len] == '\\0'");
    }
}

struct Output_Compare_Memcmp {
    output_Compare: Output_Compare,
}

impl Output_Compare_Memcmp {
    pub fn output_comparison(&mut self, expr1: &Output_Expr, expr2: &Output_Expr) {
        let firstchar_done: bool = self
            .output_Compare
            .output_firstchar_comparison(expr1, expr2);
        print!(" && !");
        if (option[UPPERLOWER]) {
            print!("gperf_case_");
        }
        print!("memcmp (");
        if (firstchar_done) {
            expr1.output_expr();
            print!(" + 1, ");
            expr2.output_expr();
            print!(" + 1, len - 1");
        } else {
            expr1.output_expr();
            print!(", ");
            expr2.output_expr();
            print!(", len");
        }
        print!(")");
    }
}

pub fn output_keyword_entry(temp: &mut KeywordExt, stringpool_index: i32, indent: &str) {
    if (option[TYPE]) {
        output_line_directive(temp._lineno);
    }
    print!("{}    ", indent);
    if (option[TYPE]) {
        print!("{{");
    }
    if (option[SHAREDLIB]) {
        print!(
            "(int)(long)&((struct {}_t *)0)->{}_str{}",
            option.get_stringpool_name(),
            option.get_stringpool_name(),
            stringpool_index
        );
    } else {
        output_string(temp._allchars, temp._allchars_length);
    }
    if (option[TYPE]) {
        if (temp._rest.len() > 0) {
            print!(",{}", temp._rest);
        }
        print!("}}");
    }
    if (option[DEBUG]) {
        print!(
            " /* hash value = {}, index = {} */",
            temp._hash_value, temp._final_index
        );
    }
}

pub fn output_keyword_blank_entries(count: i32, indent: &str) {
    let mut columns: i32;

    if (option[TYPE]) {
        let mut d: i32;
        if (option[NULLSTRINGS]) {
            d = 8;
        } else {
            d = 2;
        }
        columns = 58 / (4 + d + option.get_initializer_suffix().chars().count());
        if (columns == 0) {
            columns = 1;
        }
    } else {
        if (option[SHAREDLIB]) {
            columns = 9;
        } else if (option[NULLSTRINGS]) {
            columns = 4;
        } else {
            columns = 9;
        }
    }

    let mut column = 0;
    let mut i = 0;
    while i < count {
        if ((column % columns) == 0) {
            if (i > 0) {
                print!(",\n");
            }
            print!("{}    ", indent);
        } else {
            if (i > 0) {
                print!(", ");
            }
        }
        if (option[TYPE]) {
            print!("{{");
        }
        if (option[SHAREDLIB]) {
            print!("-1");
        } else {
            if (option[NULLSTRINGS]) {
                print!("(char*)0");
            } else {
                print!("\"\"");
            }
        }
        if (option[TYPE]) {
            print!("{}}}", option.get_initializer_suffix());
        }
        column = column + 1;
        i = i + 1;
    }
}

pub fn output_switch_case(
    list: &mut KeywordExt_List,
    indent: i32,
    jumps_away: &mut i32,
) -> &mut KeywordExt_List {
    if (option[DEBUG]) {
        print!(
            "{}{}/* hash value = {}, keyword = \"{}\" */\n",
            indent,
            list.first()._hash_value,
            list.first()._allchars_length,
            list.first()._allchars
        );
    }

    if (option[DUP] && list.first()._duplicate_link) {
        if (option[LENTABLE]) {
            print!(
                "{}lengthptr = &{}[{}];\n",
                indent,
                option.get_lengthtable_name(),
                list.first()._final_index
            );
        }
        print!(
            "{}wordptr = &{}[{}];\n",
            indent,
            option.get_wordlist_name(),
            list.first()._final_index
        );

        let mut count = 0;
        let mut links: Option<&mut KeywordExt> = list.first();
        while !links.is_none() {
            count = count + 1;
            links = links._duplicate_link
        }

        print!(
            "{}wordendptr = wordptr + {};\n{}goto multicompare;\n",
            indent, count, indent
        );
        *jumps_away = 1;
    } else {
        if (option[LENTABLE]) {
            print!(
                "{}if (len == {})\n{}  {{\n",
                indent,
                list.first()._allchars_length,
                indent
            );
            indent = indent + 4;
        }
        print!("{}resword = ", indent);
        if (option[TYPE]) {
            print!(
                "&{}[{}]",
                option.get_wordlist_name(),
                list.first()._final_index
            );
        } else {
            output_string(list.first()._allchars, list.first()._allchars_length);
        }
        print!(";\n");
        print!("{}goto compare;\n", indent);
        if (option[LENTABLE]) {
            indent -= 4;
            print!("{}  }}\n", indent);
        } else {
            *jumps_away = 1;
        }
    }
    return list.rest();
}

pub fn output_switches(
    list: *mut KeywordExt_List,
    num_switches: i32,
    size: i32,
    min_hash_value: i32,
    max_hash_value: i32,
    indent: i32,
) {
    if (option[DEBUG]) {
        print!(
            "{}}}/* know {} <= key <= {}, contains {} cases */\n",
            indent, min_hash_value, max_hash_value, size
        );
    }
    if (num_switches > 1) {
        let part1 = num_switches / 2;
        let part2 = num_switches - part1;
        let size1 = ((size) as f32 / (num_switches) as f32 * (part1) as f32 + 0.5) as i32;
        let size2 = size - size1;

        let temp: &mut KeywordExt_List = list;
        let count = size1;
        while count > 0 {
            temp = temp.rest();
            count = count - 1;
        }

        print!(
            "{{{{{}}}if (key < {})\n{}  {{\n",
            indent,
            temp.first()._hash_value,
            indent
        );

        output_switches(
            list,
            part1,
            size1,
            min_hash_value,
            temp.first()._hash_value - 1,
            indent + 4,
        );

        print!("{}  }}\n{}else\n{}  {{\n", indent, indent, indent);

        output_switches(
            temp,
            part2,
            size2,
            temp.first()._hash_value,
            max_hash_value,
            indent + 4,
        );

        print!("{}  }}\n", indent);
    } else {
        /* Output a single switch.  */
        let lowest_case_value = list.first()._hash_value;
        if (size == 1) {
            let jumps_away = 0;
            assert!(min_hash_value <= lowest_case_value);
            assert!(lowest_case_value <= max_hash_value);
            if (min_hash_value == max_hash_value) {
                output_switch_case(list, indent, &mut jumps_away);
            } else {
                print!(
                    "{}if (key == {})\n{}  {{\n",
                    indent, lowest_case_value, indent
                );
                output_switch_case(list, indent + 4, &mut jumps_away);
                print!("{}  }}\n", indent);
            }
        } else {
            if (lowest_case_value == 0) {
                print!("{}switch (key)\n", indent);
            } else {
                print!("{}switch (key - {})\n", indent, lowest_case_value);
            }
            print!("{}  {{\n", indent);
            while size > 0 {
                let jumps_away = 0;
                print!(
                    "{}{}    case {}:\n",
                    indent,
                    "",
                    list.first()._hash_value - lowest_case_value
                );
                list = output_switch_case(list, indent + 6, &mut jumps_away);
                if (!jumps_away) {
                    print!("{}{}      break;\n", indent, "");
                }
                size = size - 1;
            }
            print!("{}{}  }}\n", indent, "");
        }
    }
}
