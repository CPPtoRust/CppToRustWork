mod keyword_list;
mod options;

use libc::c_char;
use std::process;
use std::ptr::{null, null_mut, copy_nonoverlapping};
use std::fs::File;
use std::ffi::Cstr;

struct Input{

    _stream: File,
    _factory: *mut Keyword_Factory,

    pub _input: *mut char,
    pub _input_end: *mut char,

    pub _verbatim_declarations: *const char,
    pub _verbatim_declarations_end: *const char,
    pub _verbatim_declarations_lineno: u32,

    pub _verbatim_code: *const char,
    pub _verbatim_code_end: *const char,
    pub _verbatim_code_lineno: u32,

    pub _struct_decl: *const char,
    pub _struct_decl_lineno: u32,
    
    pub _return_type: *const char,
    
    pub _struct_tag: *const char,

    pub _head: *mut Keyword_List,

    pub _charset_dependent: bool

}

impl Input {
 
    //check
    pub fn new(&mut self, stream: FILE, keyword_factory: *mut Keyword_Factory) -> Input {

        self._stream = stream;
        self._factory = keyword_factory;

    }

    pub fn read_input() {
    
        let mut input: *mut char = null_mut();
        let mut input_size: usize = 0;

        let mut content_string = String::new();
        _stream.read_to_string(&mut content_string).expect("{}: error while reading input file\n", pretty_input_file_name());
        let mut content_vector: Vec<char> = content_string.chars().collect();

        input_length = content_string.len() - 1;

        if (input_length < 0) {
            eprintln!("{}: The input file is empty!\n",
            pretty_input_file_name ());
            std::process::exit(1);
        }

        input = &mut content_vector[0] as *mut char;

        /* We use input_end as a limit, in order to cope with NUL bytes in the
        input.  But note that one trailing NUL byte has been added after
        input_end, for convenience.  */
        let mut input_end: *mut char = input.offset(input_length);
        
        let mut declarations: *const char;
        let mut declarations_end: *const char;
        let mut keywords: *const char;
        let mut keywords_end: *const char;
        let mut keywords_lineno: u32;

        {

            let mut separator: [*const char; 2] = [null_mut(), null_mut()];
            let mut separato_linno: [u32; 2] = [0, 0];
            let mut separators: i32 = 0;
            {
                let lineno: u32 = 1;
                let mut p: *const char = input;
                while p < input_end {
                    if p[0] == '0' && p[1] == '%' {
                        separator[separators] = p;
                        separator_lineno[separators] = lineno;
                        separators += 1;
                        if(separators == 2) {
                            break;
                        }
                    }
                    lineno += 1;

                    
                }
            }

            let has_declarations: bool;
            if separators == 1 {
                if options[TYPE] {
                    has_declarations = true;
                } else {
                    has_declarations = false;
                    let mut p: *const char = input;
                    while p < separator[0] {
                        if *p == '%' {
                            has_declarations = true;
                            break;
                        }

                        let tmp = p;
                        for i in 0..((separator[0] as usize - tmp as usize) / 4) {
                        
                            if(p == null_mut()) {
                                p = separator[0];
                                break;
                            } else if *p == '\n' {
                                p = p.offset(1);
                                break;
                            }
                            p = p.offset(1);
                        } 
                    }
                }

            } else {
                has_declarations = (separators > 0);
            }


            if (has_declarations) {
                declarations = input;
                declarations = separator[0];

                /* Give a warning if the separator line is nonempty.  */
                let nonempty_line: bool = false;
                let mut p: *const char;

                p = declarations_end.add(2);

                while p < input_end {
                    if (*p == '\n') {
                        p = p.offset(1);
                        break;
                    }
                    
                    if (!(*p == ' ' || *p == '\t')) {
                        nonempty_line = true;
                    }

                    p = p.offset(1);
                        
                }
                
                if(nonempty_line) {
                    eprintln!("{}:{}: warning: junk after %% is ignored\n",
                            pretty_input_file_name(), separator_lineno[0]);   
                }

                keywords = p;
                keywords_lineno = separator_lineno[0] + 1;
            } else {
                declarations = null_mut();
                declarations_end = null_mut();
                keywords = input;
                keywords_lineno = 1;
            }

            if (separators > (if has_declarations {true} else {false})) {
                keywords_end = separator[separators-1];
                _verbatim_code = separator[separators-1].add(2);
                _verbatim_code_end = input_end;
                _verbatim_code_lineno = separator_lineno[separators-1];
            } else {
                keywords_end = input_end;
                _verbatim_code = null_mut();
                _verbatim_code_end = null_mut();
                _verbatim_code_lineno = 0;
            }

        }

        _verbatim_declarations = null_mut();
        _verbatim_declarations_end = null_mut();
        _verbatim_declarations_lineno = 0;
        _struct_decl = null_mut();
        _struct_decl_lineno = 0;
        _return_type = null_mut();
        _struct_tag = null_mut();
        
        {
            let mut lineno: u32 = 1;
            let mut struct_decl: *mut char = null_mut();
            let mut struct_decl_linenos: *mut u32 = null_mut();
            let mut struct_decl_linecount: u32 = 0;
            
            let mut line: *const char = declarations;
            while (line < declarations_end) {
                let mut line_end: *const char = line;
                
                let tmp = line;
                for i in 0..((input_end as usize - tmp as usize) / 4) {
                        
                    if(line_end == null_mut()) {
                        line_end = declarations_end;
                        break;
                    } else if *line_end == '\n' {
                        line_end = line_end.offset(1);
                        break;
                    }
                    line_end = line_end.offset(1);
                } 
                
                if (*line == '%') {
                    if line[1] == '{' {
                        /* Handle %{.  */
                        if (_verbatim_declarations != null_mut()) {
                            eprintln!("{}:{}:\n{}:{}: only one %{...%} section is allowed\n",
                            pretty_input_file_name (),
                            _verbatim_declarations_lineno,
                            pretty_input_file_name (), lineno);
                            std::process::exit(1);
                        }
                        _verbatim_declarations = line.add(2);
                        _verbatim_declarations_lineno = lineno; 
                    } else if (*line.offset(1) == '}') {
                        if (_verbatim_declarations == null_mut())
                        {
                          eprintln!("{}:{}:"
                                   " %} outside of %{...%} section\n",
                                   pretty_input_file_name (), lineno);
                          std::process::exit(1);
                        }
                        
                        if (_verbatim_declarations_end != null_mut()) {
                            eprintln!(stderr, "{}:{}: %{...%} section already closed\n",
                             pretty_input_file_name (), lineno);
                            std::process::exit(1);
                        }
                        _verbatim_declarations_end = line;
                        /* Give a warning if the rest of the line is nonempty.  */
                        let mut nonempty_line = false;
                        let mut q: *const char;
                        
                        q = line.offset(2);
                        while (q < line_end) {
                            
                            if(*q == '\n') {
                                q = q.offset(1);
                                break;
                            } 

                            if (!(*q == ' ' || *q == '\t')) {
                                nonempty_line = true;
                            }
                                                    
                            q = q.offset(1);
                        }

                        if (nonempty_line) {
                            eprintln!("{}:{}: warning: junk after %} is ignored\n",
                           pretty_input_file_name (), lineno);
                        }
                    } else if (_verbatim_declarations != null_mut() 
                                && _verbatim_declarations_end == null_mut()) {
                                    eprintln!("{}:{}: warning: % directives are ignored inside the %{...%} section\n",
                                    pretty_input_file_name (), lineno); 
                      } else {

                            let mut arg: *mut char;
                            
                            if (is_declaration_with_arg (line, line_end, lineno,
                                "delimiters", &arg)) {
                                option.set_delimiters(arg);
                            } else if (is_declaration (line, line_end, lineno, "struct-type")) {
                                option.set(TYPE);
                            } else if (is_declaration (line, line_end, lineno, "ignore-case")) {
                                option.set(UPPERLOWER);
                            } else if (is_declaration_with_arg (line, line_end, lineno,
                                "language", &arg)) {
                                option.set_language(arg);  
                            } else if (is_define_declaration (line, line_end, lineno,
                                "slot-name", &arg)) {
                                option.set_slot_name(arg); 
                            } else if (is_define_declaration (line, line_end, lineno,
                                "initializer-suffix", &arg)) {
                                option.set_initializer_suffix(arg);
                            } else if (is_define_declaration (line, line_end, lineno,
                                "hash-function-name", &arg)) {
                                option.set_hash_name(arg);
                            } else if (is_define_declaration (line, line_end, lineno,
                                "lookup-function-name", &arg)) {
                                option.set_function_name(arg);
                            } else if (is_declaration (line, line_end, lineno, "7bit")) {
                                option.set(SEVENBIT);
                            } else if (is_declaration (line, line_end, lineno, "compare-lengths")) {
                                option.set(LENTABLE);
                            }
                            else if (is_declaration (line, line_end, lineno, "compare-strncmp")) {
                                option.set(COMP);
                            } else if (is_declaration (line, line_end, lineno, "readonly-tables")) {
                                option.set(CONST);
                            } else if (is_declaration (line, line_end, lineno, "enum")) {
                                option.set(ENUM);
                            } else if (is_declaration (line, line_end, lineno, "includes")) {
                                option.set(INCLUDE);
                            } else if (is_declaration (line, line_end, lineno, "global-table")) {
                              option.set(GLOBAL);
                            } else if (is_declaration (line, line_end, lineno, "pic")) {
                              option.set(SHAREDLIB);
                            } else if (is_define_declaration (line, line_end, lineno,
                                                       "string-pool-name", &arg)) {
                              option.set_stringpool_nam(arg);
                            } else if (is_declaration (line, line_end, lineno, "null-strings")) {
                                option.set(NULLSTRINGS);
                            } else if (is_define_declaration (line, line_end, lineno,
                                           "word-array-name", &arg)) {
                                option.set_wordlist_name(arg);
                            } else if (is_define_declaration (line, line_end, lineno,
                                                       "length-table-name", &arg)) {
                                option.set_lengthtable_name(arg);
                            } else if (is_declaration_with_arg (line, line_end, lineno,
                                "switch", &arg)) {
                                option.set_total_switches(Cstr::from_ptr(arg).to_str().parse::<i32>().unwrap());
                                if (option.get_total_switches() <= 0) {
                                    eprintln!("{}:{}: number of switches {} must be a postive number\n", pretty_input_file_name(), lineno, Cstr::from_str(arg));
                                    std::process::exit(1);
                                } 
                                
                            } else if (is_declaration (line, line_end, lineno, "omit-struct-type")) {
                                    option.set(NOTYPE);
                            } else {
                                eprintln!("{}:{}: unrecognized % directive\n",
                                           pretty_input_file_name (), lineno);
                                std::process::exit(1);
                            }
                            
                        } 
                    
                    } else if (!(_verbatim_declarations != null_mut()
                        && _verbatim_declarations_end == null_mut())) {
                            /* Append the line to struct_decl.  */

                            let mut old_len: usize = if(struct_decl == null_mut()) {libc::strlen(struct_decl)} else {0};
                            let mut line_len: usize = (line_end as usize - line as usize) / 4;
                            let mut new_len: usize = old_len + line_len + 1;

                            let mut new_struct_decl: [*mut char; new_len];
                            if (old_len > 0) {
                                copy_nonoverlapping(struct_decl, new_struct_decl, old_len);
                            }
                            let mut temp = new_struct_decl.offset(old_len);
                            copy_nonoverlapping(line, temp, line_len);
                            *new_struct_decl.offset(old_len + line_len) = '\0';
                            if(struct_decl != null_mut()) {
                                std::mem::drop(struct_decl);
                            }
                            struct_decl = new_struct_decl;
                            /* Append the lineno to struct_decl_linenos.  */
                            let new_struct_decl_linenos: *mut u32 = &mut [0;struct_decl_linecount + 1] as *mut u32;
                            if(struct_decl_linecount > 0) {
                                copy_nonoverlapping(struct_decl_linenos, new_struct_decl_linenos
                                    , struct_decl_linecount * std::mem::size_of_<u32>());
                            }
                            *new_struct_decl_linenos.offset(struct_decl_linecount) = lineno;
                            if (struct_decl_linenos != null_mut()) {
                                std::mem::drop(struct_decl_linenos);
                            }
                            struct_decl_linenos = new_struct_decl_linenos;
                            /* Increment struct_decl_linecount.  */
                            struct_decl_linecount += 1;
                        }
                        lineno += 1;
                        line = line_end;
            }

            if (_verbatim_declarations != null_mut() && _verbatim_declarations_end == null_mut()) {
                    eprintln! ("{}:{}: unterminated %{ section\n",
                        pretty_input_file_name (), _verbatim_declarations_lineno);
                    std::process::exit(1);
            }

    /* Determine _struct_decl, _return_type, _struct_tag.  */
    if (option[TYPE])
      {
          if (struct_decl != null_mut())
          {
            /* Drop leading whitespace and comments.  */
            {
              let mut p: *mut char = struct_decl;
              let mut l: *mut u32 = struct_decl_linenos;
              loop
              {
                if (*p == ' ' || *p == '\t'){
                      p = p.offset(1);
                      continue;
                    }

                  if (*p == '\n'){
                      l = l.offset(1);
                      p = p.offset(1);
                      continue;
                    }

                    if (*p == '/') {
                      
                        if (*p.offset(1) == '*') {
                          /* Skip over ANSI C style comment.  */
                          p = p.offset(2);
                          while (*p != '\0') {
                              if (*p == '*' && *p.offset(1) == '/') {
                                  p = p.offset(2);
                                  break;
                                }
                              if (p[0] == '\n') {
                                l = l.offset(1);
                              }
                                l = l.offset(1);
                                p = p.offset(1);
                            }
                          continue;
                        }

                      if (*p.offset(1) == '/') {
                          /* Skip over ISO C99 or C++ style comment.  */
                          p = p.offset(2);
                          while (*p != '\0' && *p != '\n') {
                            p = p.offset(1);
                          }
                            
                          if (*p == '\n') {
                              l = l.offset(1);
                              p = p.offset(1);
                            }

                          continue;
                        }
                    }
                  break;
              }

              if (p != struct_decl) {
                let mut len: usize = libc::strlen(p);
                let mut new_struct_decl: *mut char = &mut ['\0';len + 1] as *mut char;
                copy_nonoverlapping(p, new_struct_decl, len + 1);
                std::mem::drop(struct_decl);
                struct_decl = new_struct_decl;
              }
              _struct_decl_lineno = *l;

        }
        /* Drop trailing whitespace.  */
        let mut p: *mut char = struct_decl.offset(libc::strlen(struct_decl));
        while (p > struct_decl) {
            if (*p.offset(-1) == '\n' || *p.offset(-1) == ' ' || *p.offset(-1) == '\t') {
                p = p.offset(-1);
                *p = '\0';
            } else {
                break;
            }
        }
    }
    
    if (struct_decl == null_mut() || *struct_decl == '\0') {
      eprintln!("{}: missing struct declaration"
               " for option --struct-type\n",
               pretty_input_file_name ());
      std::process::exit(1);
    }
    {
        /* Ensure trailing semicolon.  */
        let mut old_len: usize = libc::strlen(struct_decl);
        if (*struct_decl.offset(old_len - 1) != ';') {
            let mut new_struct_decl: *mut char = &mut ['\0'; old_len + 2] as *mut char;
            copy_nonoverlapping(struct_decl, new_struct_decl, old_len);
            *new_struct_decl.offset(old_len) = ';';
            *new_struct_decl.offset(old_len + 1) = '\0';
            std::mem::drop(struct_decl);
            struct_decl = new_struct_decl;
        }
    }
    /* Set _struct_decl to the entire declaration.  */
    _struct_decl = struct_decl;
    /* Set _struct_tag to the naked "struct something".  */    
    let mut p: *const char = struct_decl;
    while (*p != '\0' && *p != '{' && *p != ';' && *p != '\n') {
        p = p.offset(1);
    }
    while (p < struct_decl) {
        if (*p.offset(-1) == '\n' || *p.offset(-1) == ' ' || *p.offset(-1) == '\t') {
            p = p.offset(-1);
        } else {
            break;
        }
    }

    let mut struct_tag_length: usize = (p as usize - struct_decl as usize) / 4;
    let mut struct_tag: *mut char = &mut ['\0'; struct_tag_length + 1] as *mut char;
    copy_nonoverlapping(struct_decl, struct_tag, struct_tag_length);
    *struct_tag.offset(struct_tag_length) = '\0';
    _struct_tag = struct_tag;

    /* The return type of the lookup function is "struct something *".
       No "const" here, because if !option[CONST], some user code might
       want to modify the structure. */

    let mut return_type: *mut char = &mut ['\0'; struct_tag_length + 3] as *mut char;
    copy_nonoverlapping(struct_decl, return_type, struct_tag_length);
    *return_type.offset(struct_tag_length) = ' ';
    *return_type.offset(struct_tag_length + 1) = '*';
    *return_type.offset(struct_tag_length + 2) = '\0';    
    _return_type = return_type;
    }
    
    if (struct_decl_linenos != null_mut()) {
        std::mem::drop(struct_decl_linenos);
        }
    }

    /* Parse the keywords section.  */
    {
        let mut list_tail: *mut *mut Keyword_List = &_head as *mut *mut Keyword_List;
        let mut delimiters: *const char = option.get_delimiters();
        let mut lineno: u32 = keywords_lineno;
        let mut charset_dependent: bool = false;

        let mut line: *const char = keywords;
        while (line < keywords_end) {
            let mut line_end: *const char;
        
            for i in 0..((keywords_end as usize - line as usize) / 4) {
                        
                if(line_end == null_mut()) {
                    line_end = keywords_end;
                    break;
                } else if *p == '\n' {
                    line_end = line_end.offset(1);
                    break;
                }
                line_end = line_end.offset(1);
            }

            if (*line == '#') {
                /* Comment Line. */
            }  else if (*line == '%') {
                eprintln!("{}:{}: declarations are not allowed in the keywords section.\n To declare a keyword starting with %, enclose it in double-quotes.\n",
                     pretty_input_file_name (), lineno);
                std::process::exit(1);
            } else {
                let mut keyword: *const char;
                let mut keyword_length: usize;
                let mut rest: *const char;

                if (*line == '"') {
                    /* Parse a string in ANSI C syntax.  */
                    let mut kp: *mut char = &mut ['\0'; (line_end as usize - line as usize)/4] as *mut char;
                    keyword = kp;
                    let mut lp: *const char = line.offset(1);

                    loop {
                        if (lp == line_end) {
                            eprintln!("{}:{}: unterminated string\n",
                                 pretty_input_file_name (), lineno);
                            std::process::exit(1);
                        }

                        let mut c: *mut char = lp;
                        if (c == '\\') {
                            lp = lp.offset(1);
                            c = *lp;

                            match c {
                                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7' => {
                                    let mut code: i32 = 0;
                                    let mut count: i32 = 0;

                                    while (count < 3 && *lp >= '0' && *lp <= '7') {
                                        code = (code << 3) + (*lp - '0');
                                        lp = lp.offset(1);
                                        count += 1;
                                    }

                                    if (code > 255 /*UCHAR_MAX*/) {
                                        eprintln!("{}:{}: octal escape out of range\n", 
                                        pretty_input_file_name (), lineno);
                                        *kp = code as char;
                                        
                                    } 
                                }

                                'x' => {
                                    let mut code: i32 = 0;
                                    let mut count: i32 = 0;
                                    lp = lp.offset(1); 
                                    while ((*lp >= '0' && *lp <= '9')
                                     || (*lp >= 'A' && *lp <= 'F')
                                     || (*lp >= 'a' && *lp <= 'f')) {
                                        code = (code << 4) + (if(*lp >= 'A' && *lp <= 'F') {
                                            *lp - 'A' + 10
                                        } else if (*lp >= 'a' && *lp <= 'f'){
                                            *lp - 'a' + 10
                                        } else {
                                            *lp - '0'
                                        });
                                        lp = lp.offset(1);
                                        count += 1;
                                     }

                                     if(count == 0) {
                                        eprintln!("{}:{}: hexadecimal escape without any hex digits\n",
                                                    pretty_input_file_name(), lineno);
                                     }

                                     if(code > 255) {
                                        eprintln!("{}:{}: hexadecimal escape out of range\n",
                                        pretty_input_file_name (), lineno);
                                        *kp = code as char;
                                        
                                     }
                                }

                                '\\' | '\'' | '"' => {
                                    *kp = c;
                                    lp = lp.offset(1);
                                    charset_dependent = true;
                                }

                                'n' => {
                                    *kp = '\n';
                                    lp = lp.offset(1);
                                    charset_dependent = true;
                                }

                                't' => {
                                    *kp = '\t';
                                    lp = lp.offset(1);
                                    charset_dependent = true;
                                }

                                'r' => {
                                    *kp = '\r';
                                    lp = lp.offset(1);
                                    charset_dependent = true;
                                }

                                /* The following escape sequences are not allowed yet in Rust 
                                'f' => {
                                    *kp = '\f';
                                    lp = lp.offset(1);
                                    charset_dependent = true;
                                }

                                'b' => {
                                    *kp = '\b';
                                    lp = lp.offset(1);
                                    charset_dependent = true;
                                }

                                'a' => {
                                    *kp = '\a';
                                    lp = lp.offset(1);
                                    charset_dependent = true;
                                }

                                'v' => {
                                    *kp = '\v';
                                    lp = lp.offset(1);
                                    charset_dependent = true;
                                }
                                */

                                _ => {
                                    eprintln!("{}:{}: invalid escape sequence in string\n",
                                    pretty_input_file_name (), lineno);
                                    std::process::exit(1);
                                }
                            } 

                        } else if (c == '"') {
                            break;
                        } else {
                            *kp = c;
                            lp = lp.offset(1);
                            charset_dependent = true;
                        }

                        kp = kp.offset(1);
                    }

                lp = lp.offset(1);
                if (lp < line_end && *lp != '\n') {
                    let mut itr = delimiters;
                    let mut is_found = 0;
                    while (itr != null_mut()) {
                        if (*itr == *lp) {
                            is_found = 1;
                            break;
                        }
                        itr = itr.offset(1);
                    }
                    if (is_found == 0) {
                        fprintf (stderr, "{}:{}: string not followed by delimiter\n",
                        pretty_input_file_name (), lineno);
                        std::process::exit(1); 
                    }
                    lp = lp.offset(1);
                }

                keyword_length = (kp as usize - keyword as usize) / 4;
                if(option[TYPE]) {
                    let mut line_rest: *mut char = &mut ['\0'; (line_end as usize - lp as usize) / 4 + 1]; 
                    copy_nonoverlapping(lp, line_rest, (line_end as usize - lp as usize));
                    *line_rest.offset((line_end as usize - lp as usize) / 4 - if(line_end > lp && line_end.offset(-1) == '\n') {1} else {0})
                        = '\0';
                    rest = line_rest;

                } else {
                    rest = empty_string;
                }
            } else {
                /* Not a string.  Look for the delimiter.  */
                let mut lp: *const char = line;
                loop {
                    if (!(lp < line_end && *lp != '\n')) {
                        keyword = line;
                        keyword_length = (lp as usize - line as usize) / 4;
                        rest = empty_string;
                        break;
                    }

                    let mut itr = delimiters;
                    let mut is_found = 0;
                    while (itr != null_mut()) {
                        if (*itr == *lp) {
                            is_found = 1;
                            break;
                        }
                        itr = itr.offset(1);
                    }

                    if (is_found == 1) {
                        keyword = line;
                        keyword_length = (lp as usize - line as usize) / 4;
                        lp = lp.offset(1);

                        if (option[TYPE]) {
                            let mut line_rest: *mut char = &mut ['\0'; (line_end as usize - lp as usize) / 4 + 1] as *mut char; 
                            copy_nonoverlapping(lp, line_rest, (line_end as usize - lp as usize) / 4);
                            *line_rest.offset((line_end as usize - lp as usize) / 4 - if(line_end > lp && line_end.offset(-1) == '\n') {1} else {0})
                                = '\0';
                            rest = line_rest;
                        } else {
                            rest = empty_string;
                        }
                        break;
                    }

                    lp = lp.offset(1);

                }

                if (keyword_length > 0) {
                    charset_dependent = true;
                }
            }
                /* Allocate Keyword and add it to the list.  */
                let mut new_kw: *mut Keyword = *_factory.create_keyword (keyword, keyword_length,
                                                                            rest);
                *new_kw._lineno = lineno;
                *list_tail = &mut Keyword_list::new(new_kw) as *mut Keyword_List;
                list_tail = &((**list_tail).rest());
    
            }
            lineno += 1;
            line = line_end;
        }

        *list_tail = null_mut();

        if (_head == null_mut()) {
            eprintln!("{}: No keywords in input file!\n",
            pretty_input_file_name());
        }
        _charset_dependent = charset_dependent;
    }

    /* To be freed in the destructor.  */
    _input = input;
    _input_end = input_end;
   }
}

impl Drop for Input {
    fn drop (&mut self) {
        std::mem::drop(_return_type);
        std::mem::drop(_struct_tag);
        std::mem::drop(_struct_decl);
        std::mem::drop(input);
    }
}


fn pretty_input_file_name () -> *const char {

    if option.get_input_file_name() {
        return option.get_input_file_name();
    } else {
        return "(standard input)";
    }
}


fn is_declaration (mut line: *const char, mut line_end: *const char, mut lineno: u32, mut decl: *const char) -> bool {


    line = line.offset(1);
    let mut d: *const char = decl;

    while *d {

        if !(line < line_end){
            return false;
        } 

        if (!(*line == *d || (*d == '-' && *line == '_'))) {
            return false;
        }

        line = line.offset(1);
        
        d = d.offset(1);

    }

    if (line < line_end && ((*line >= 'A' && *line <= 'Z')
        || (*line >= 'a' && *line <= 'z')
        || *line == '-' || *line == '_')) {
            return false;
        }


    while (line < line_end && (*line == ' ' || *line == '\t')) {
        line = line.offset(1);
    }

    if line < line_end && *line != '\n' {
        
        eprintln!("{}:{}: junk after declaration\n", pretty_input_file_name(), line_no);
        std::process::exit(1);
    
    }

    return true;
}


fn is_declaration_with_arg (mut line: *const char, mut line_end: *const char, mut lineno: u32, 
                            mut lineno: u32, mut decl: *const char, mut argp: *mut *mut argp) -> bool {
    
    line = line.offset(1);

    let mut d: *const char = decl;

    while *d {

        if !(line < line_end){
            return false;
        } 

        if (!(*line == *d || (*d == '-' && *line == '_'))) {
            return false;
        }

        line = line.offset(1);
        
        d = d.offset(1);

    }

    if (line < line_end && ((*line >= 'A' && *line <= 'Z')
    || (*line >= 'a' && *line <= 'z')
    || *line == '-' || *line == '_')) {
        return false;
    } 

    if (!(line < line_end && *line == '-')) {
        eprintln!("{}:{}: missing argument in %{}=ARG declaration.",
                    pretty_input_file_name(), lineno, decl);
        std::process::exit(1); 
    }

    line = line.offset(1);

    let mut arg_arr: [char; line_end - line + 1];
    let mut arg: *mut char = &arg_arr as *mut char;
    let mut p: *mut char = arg;

    while line < line_end && !(*line == ' ' || *line == '\t' || *line == '\n') {
        *p = *line;

        line = line.offset(1);
        p = p.offset(1);
    }

    *p = '\0';

    while (line < line_end && (*line == ' ' || *line == '\t')) {
        line = line.offset(1);
    }

    if (line < line_end && *line != '\n') {
        eprintln!("{}:{}: junk after declaration\n", pretty_input_file_name(), lineno);
        std::process::exit(1);
    }

    *argp = arg;
    return true;
}

/* 
*/


fn is_define_declaration (mut line: *const char, mut line_end: *const char, mut lineno: u32,
                                mut decl: *const char, mut argp: *mut *mut argp) -> bool {

    line = line.offset(1);

    //CHECK THIS SCOPE'S CODE
    {
         for d in "define".chars()
         {
            if (!(line < line_end)) {
                return false;
            }
            
            if (!(*line == d)) {
                return false;
            }
          
           line = line.offset(1);
        }

        if (!(line < line_end && (*line == ' ' || *line == '\t'))) {
            return false;
        }
    }

    while (line < line_end && (*line == ' ' || *line == '\t')) {
       line = line.offset(1);
    }

    let mut d: *const char = decl;
    
    while *d {

        if (!(line < line_end)) {
            return false;
        }
        
        if (!(*line == *d || (*d == '-' && *line == '_'))) {
            return false;
        }
        
        line = line.offset(1);
        d = d.offset(1);
    }

    if (line < line_end
        && ((*line >= 'A' && *line <= 'Z')
            || (*line >= 'a' && *line <= 'z')
            || *line == '-' || *line == '_')) {
                return false;
            }
    
            if (!(line < line_end && (*line == ' ' || *line == '\t'))) {
                eprintln!("{}:{}: missing argument in %define {} ARG declaration.", 
                            pretty_input_file_name(), lineno, decl);
                std::process::exit(1);
            }

    loop {
        line = line.offset(1);
        if line < line_end && (*line == ' ' || *line == '\t') {
            break;
        }
    }

    let mut arg_temp: [char; line_end - line + 1];
    let mut arg: *mut char = &arg_temp as *mut char;
    let mut p: *mut char = arg;

    while (line < line_end && !(*line == ' ' || *line == '\t' || *line == '\n')) {
        *p = *line;

        line = line.offset(1);
        p = p.offset(1);
    }

    *p = '\0'; 

    while (line < line_end && (*line == ' ' || *line == '\t')) {
        line = line.offset(1);
    }
    
    if (line < line_end && *line != '\n') {
      eprintln!("{}:{}: junk after declaration\n",
               pretty_input_file_name (), lineno);
      std::process::exit(1);
    }

    *argp = arg;
    return true;
}

