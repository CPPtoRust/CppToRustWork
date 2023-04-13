#[path = "positions.rs"]
mod positions;
use positions::{Positions, PositionIterator};

extern crate getopts;
use getopts::{Options};

use std::io::{Write};
use std::env;

/* Enumeration of the possible boolean options.  */


  /* --- Input file interpretation --- */

  /* Handle user-defined type structured keyword input.  */
  pub const TYPE: i32 = 1 << 0;

  /* Ignore case of ASCII characters.  */
  pub const UPPERLOWER: i32 = 1 << 1;

  /* --- Language for the output code --- */

  /* Generate K&R C code: no prototypes, no const.  */
  pub const KRC: i32 = 1 << 2;

  /* Generate C code: no prototypes, but const (user can #define it away).  */
  pub const C: i32  = 1 << 3;

  /* Generate ISO/ANSI C code: prototypes and const, but no class.  */
  pub const ANSIC: i32  = 1 << 4;

  /* Generate C++ code: prototypes, const, class, inline, enum.  */
  pub const CPLUSPLUS: i32 = 1 << 5;

  /* --- Details in the output code --- */

  /* Assume 7-bit, not 8-bit, characters.  */
  pub const SEVENBIT: i32 = 1 << 6;

  /* Generate a length table for string comparison.  */
  pub const LENTABLE: i32 = 1 << 7;

  /* Generate strncmp rather than strcmp.  */
  pub const COMP: i32 = 1 << 8;

  /* Make the generated tables readonly (const).  */
  pub const CONST: i32 = 1 << 9;

  /* Use enum for constants.  */
  pub const ENUM: i32 = 1 << 10;

  /* Generate #include statements.  */
  pub const INCLUDE: i32 = 1 << 11;

  /* Make the keyword table a global variable.  */
  pub const GLOBAL: i32 = 1 << 12;

  /* Use NULL strings instead of empty strings for empty table entries.  */
  pub const NULLSTRINGS: i32 = 1 << 13;

  /* Optimize for position-independent code.  */
  pub const SHAREDLIB: i32 = 1 << 14;

  /* Generate switch output to save space.  */
  pub const SWITCH: i32 = 1 << 15;

  /* Don't include user-defined type definition in output -- it's already
     defined elsewhere.  */
  pub const NOTYPE: i32 = 1 << 16;

  /* --- Algorithm employed by gperf --- */

  /* Use the given key positions.  */
  pub const POSITIONS: i32 = 1 << 17;

  /* Handle duplicate hash values for keywords.  */
  pub const DUP: i32 = 1 << 18;

  /* Don't include keyword length in hash computations.  */
  pub const NOLENGTH: i32 = 1 << 19;

  /* Randomly initialize the associated values table.  */
  pub const RANDOM: i32 = 1 << 20;

  /* --- Informative output --- */

  /* Enable debugging (prints diagnostics to stderr).  */
  pub const DEBUG: i32 = 1 << 21;





/* Class manager for gperf program Options.  */
pub struct Options1 {

    /* Records count of command-line arguments.  */
    _argument_count: i32,
    
    /* Stores a pointer to command-line argument vector.  */
    _argument_vector: Vec<String> ,
    
    /* Holds the boolean options.  */
    _option_word: i32,
    
    /* Name of input file.  */
    _input_file_name: String,
    
    /* Name of output file.  */
    _output_file_name: String,
    
    /* The output language.  */
    _language: String,
    
    /* Jump length when trying alternative values.  */
    _jump: i32,
    
    /* Initial value for asso_values table.  */
    _initial_asso_value: i32,
    
    /* Number of attempts at finding good asso_values.  */
    _asso_iterations: i32,
    
    /* Number of switch statements to generate.  */
    _total_switches: i32,
    
    /* Factor by which to multiply the generated table's size.  */
    _size_multiple: f32,
    
    /* Names used for generated lookup function.  */
    _function_name: String,
    
    /* Name used for keyword key.  */
    _slot_name: String,
    
    /* Suffix for empty struct initializers.  */
    _initializer_suffix: String,
    
    /* Name used for generated C++ class.  */
    _class_name: String,
    
    /* Name used for generated hash function.  */
    _hash_name: String,
    
    /* Name used for hash table array.  */
    _wordlist_name: String,
    
    /* Name used for length table array.  */
    _lengthable_name: String,
    
    /* Name used for the string pool.  */
    _stringpool_name: String,
    
    /* Separates keywords from other attributes.  */
    _delimiters: String,
    
    /* Contains user-specified key choices.  */
    _key_positions: Positions

}

/* Global option coordinator for the entire program.  */
// static option: Options1 = Options1::new();

/* Records the program name.  */

/* Size to jump on a collision.  */
const DEFAULT_JUMP_VALUE: i32 = 5;



/* Default name for generated lookup function.  */
const DEFAULT_FUNCTION_NAME: &str = "in_word_set"; 

/* Default name for the key component.  */
const DEFAULT_SLOT_NAME: &str = "name";


/* Default struct initializer suffix.  */
const DEFAULT_INITIALIZER_SUFFIX: &str = "";


/* Default name for the generated class.  */
const DEFAULT_CLASS_NAME: &str = "Perfect_Hash";

/* Default name for generated hash function. */
const DEFAULT_HASH_NAME: &str = "hash";

/* Default name for generated hash table array.  */
const DEFAULT_WORDLIST_NAME: &str = "wordlist";

/* Default name for generated length table array.  */
const DEFAULT_LENGTHTABLE_NAME: &str = "lengthable";

/* Default name for string pool.  */
const DEFAULT_STRINGPOOL_NAME: &str = "stringpool";

/* Default delimiters that separate keywords from their attributes.  */

const DEFAULT_DELIMITERS: &str = ",";


const CHAR_MAX: u32 = 127;


/* Parses the command line Options and sets appropriate flags in option_word.  */


fn strpbrk(s: &str, set: &str) -> Option<usize> {
    for (i, c) in s.char_indices() {
        if set.contains(c) {
            return Some(i);
        }
    }
    None
}

fn transfer_ownership(s: String) -> String {
    String::from(s)
}

impl Options1 {
     
    /* Constructor.  */
    pub fn new() -> Options1 {
        Options1{
            _argument_count: 0,
            _argument_vector: vec![],
            _key_positions: Positions::default(),
            _option_word: C,
            _input_file_name: String::from(""),
            _output_file_name: String::from(""),
            _language: String::from(""),
            _jump: DEFAULT_JUMP_VALUE,
            _initial_asso_value: 0,
            _asso_iterations: 0,
            _total_switches: 1,
            _size_multiple: 1.0,
            _function_name: String::from(DEFAULT_FUNCTION_NAME),
            _slot_name: String::from(DEFAULT_SLOT_NAME),
            _initializer_suffix: String::from(DEFAULT_INITIALIZER_SUFFIX),
            _class_name: String::from(DEFAULT_CLASS_NAME),
            _hash_name: String::from(DEFAULT_HASH_NAME),
            _wordlist_name: String::from(DEFAULT_WORDLIST_NAME),
            _lengthable_name: String::from(DEFAULT_LENGTHTABLE_NAME),
            _stringpool_name: String::from(DEFAULT_STRINGPOOL_NAME),
            _delimiters: String::from(DEFAULT_DELIMITERS)}
    }

    /* Parses the options given in the command-line arguments.  */   
    pub fn parse_options(&mut self) {

        self._argument_vector = std::env::args().collect();
        self._argument_count = self._argument_vector.len() as i32;
        let program_name = self._argument_vector[0].clone();
        
        let VERSION: String = String::from(env!("CARGO_PKG_VERSION"));
        
        let mut opts = Options::new();
        
        opts.optopt(std::char::from_u32(CHAR_MAX + 1).unwrap().encode_utf8(&mut [0; 4]), "output-file", "", "");
        opts.optflag(std::char::from_u32(CHAR_MAX + 2).unwrap().encode_utf8(&mut [0; 4]), "ignore-case", "");
        opts.optopt("e", "delimiters", "", "");
        opts.optflag("t", "struct-type", "");
        opts.optopt("L", "language", "", "");
        opts.optopt("K", "slot-name", "", "");
        opts.optopt("F", "initializer-suffix", "", "");
        opts.optopt("H", "hash-fn-name", "", "");
        opts.optopt("H", "hash-function-name", "", "");
        opts.optopt("N", "lookup-fn-name", "", "");
        opts.optopt("N", "lookup-function-name", "", "");
        opts.optopt("Z", "class-name", "", "");
        opts.optflag("7", "seven-bit", "");
        opts.optflag("c", "compare-strncmp", "");
        opts.optflag("C", "readonly-tables", "");
        opts.optflag("E", "enum", "");
        opts.optflag("I", "includes", "");
        opts.optflag("G", "global-table", "");
        opts.optopt("W", "word-array-name", "", "");
        opts.optopt(std::char::from_u32(CHAR_MAX + 4).unwrap().encode_utf8(&mut [0; 4]), "length-table-name", "", "");
        opts.optopt("S", "switch", "", "");
        opts.optflag("T", "omit-struct-type", "");
        opts.optopt("k", "key-positions", "", "");
        opts.optflag("l", "compare-strlen", "");
        opts.optflag("l", "compare-lengths", "");
        opts.optflag("D", "duplicates", "");
        opts.optopt("f", "fast", "", "");
        opts.optopt("i", "initial-asso", "", "");
        opts.optopt("j", "jump", "", "");
        opts.optopt("m", "multiple-iterations", "", "");
        opts.optflag("n", "no-strlen", "");
        opts.optflag("o", "occurence-sort", "");
        opts.optflag("O", "optimized-collision-resolution", "");
        opts.optflag("P", "pic", "");
        opts.optopt("Q", "string-pool-name", "", "");
        opts.optflag(std::char::from_u32(CHAR_MAX + 3).unwrap().encode_utf8(&mut [0; 4]), "null-strings", "");
        opts.optflag("r", "random", "");
        opts.optopt("s", "size-multiple", "", "");
        opts.optflag("h", "help", "");
        opts.optflag("v", "version", "");
        opts.optflag("d", "debug", "");
    
        
        let matches = match opts.parse(&self._argument_vector[1..]) {
            Ok(m) => m,
            Err(_) => {
                self.short_usage("stderr", &program_name);
                std::process::exit(1);
            }
        };
    
        if matches.opt_present("a") {
            
        } 
    
        if matches.opt_present("c") {
            self._option_word |= COMP;
        }
    
        if matches.opt_present("C") {
            self._option_word |= CONST;    
        }
    
        if matches.opt_present("d") {
            self._option_word |= DEBUG;    
            eprint!("Starting program {}, version {}, with debugging on.\n",
                            program_name, VERSION);
        }
    
        if matches.opt_present("D") {
            self._option_word |= DUP;
        }
    
        if let Some(x) = matches.opt_str("e") {
            self._delimiters = x;
        }
    
        if matches.opt_present("E") {
            self._option_word |= ENUM;
        }
    
        if matches.opt_present("f") {
            
        }
    
        if let Some(x) = matches.opt_str("F") {
            self._initializer_suffix = x;
        }
    
        if matches.opt_present("g") {
    
        }
    
        if matches.opt_present("G") {
            self._option_word |= GLOBAL;
        }
    
        if matches.opt_present("h") {
            self.long_usage("stdout", &program_name);
            std::process::exit(0);
        }
    
        if let Some(x) = matches.opt_str("H") {
            self._hash_name = x;
        } 
    
        if let Some(x) = matches.opt_str("i") {
            self._initial_asso_value = x.parse().unwrap();
            if self._initial_asso_value < 0 {
                eprint!("Initial value {} should be non-zero, ignoring and continuing.\n", self._initial_asso_value);
            }
    
            // if(option._option_word & RANDOM) != 0 {
            //     eprint!("warning, -r option superceeds -i, ignoring -i option and continuing\n");
            // }
        }
    
        if matches.opt_present("I") {
            self._option_word |= INCLUDE;
        }
    
        if let Some(x) =  matches.opt_str("j") {
            self._jump = x.parse().unwrap();
            if self._jump < 0 {
                eprint!("Jump value {} must be a positive number.\n", self._jump);
                self.short_usage("stdout", &program_name);
                std::process::exit(1);
            } else if (self._jump > 0) && (self._jump % 2) == 0 {
                eprint!("Jump value {} should be odd, adding 1 and continuing...\n", self._jump);
                self._jump += 1;
            }
    
        }
    
        if let Some(y) = matches.opt_str("k") {
            
            let x: &str = y.as_str();
            self._option_word |= POSITIONS;
    
            let BAD_VALUE: i32 = -3;
            let EOS: i32 = PositionIterator::EOS;
            let mut value: i32;
            let mut sparser: PositionStringParser = PositionStringParser::new(x, 1, Positions::MAX_KEY_POS, Positions::LASTCHAR, BAD_VALUE, PositionIterator::EOS);
            
            if x.chars().nth(0).unwrap() == '*' { /* Use all the characters for hashing!!!! */
                self._key_positions.set_useall(true);
            } else {
                self._key_positions.set_useall(false);
                let mut key_positions: [i32; 256] = self._key_positions.pointer();
                let mut key_pos: i32 = 0;
                value = sparser.nextPosition();
                while (value != PositionIterator::EOS) {
    
                    if(value == BAD_VALUE) {
                        eprint!("Invalid position value or range, use 1,2,3-{},'$' or '*'.\n",
                             Positions::MAX_KEY_POS);
                        self.short_usage("stdout", &program_name);
                    }
    
                    if(key_pos == Positions::MAX_SIZE) {
    
                        /* More than Positions::MAX_SIZE key positions.
                        Since all key positions are in the range
                        0..Positions::MAX_KEY_POS-1 or == Positions::LASTCHAR,
                        there must be duplicates.  */
                        eprint!("Duplicate key positions selected\n");
                        self.short_usage("stdout", &program_name);
                        std::process::exit(1);
                    }
    
                    if(value != Positions::LASTCHAR) {
                        /* We use 0-based indices in the class Positions.  */
                        value = value - 1;
                    }
                    key_positions[key_pos as usize] = value;
                    key_pos += 1;
                    value = sparser.nextPosition();
                }
    
                let mut total_keysig_size: u32 = key_pos as u32;
                if(total_keysig_size == 0) {
                    eprint!("No key positions selected.\n");
                    self.short_usage("stdout", &program_name);
                    std::process::exit(1);
                }
    
                self._key_positions.set_size (total_keysig_size);
    
                /* Sorts the key positions *IN REVERSE ORDER!!*
                This makes further routines more efficient.  Especially
                when generating code.  */
                if (!self._key_positions.sort()) {
                    eprint!("Duplicate key positions selected\n");
                    self.short_usage("stdout", &program_name);
                    std::process::exit(1);
                }
    
            }
        }
            
        if let Some(x) = matches.opt_str("K") {
            self._slot_name = x;
        }
    
        if matches.opt_present("l") {
            self._option_word |= LENTABLE;
        }
    
        if let Some(x) = matches.opt_str("L") {
            self._language = String::from("");
            self.set_language(&x);
        }
    
        if let Some(x) = matches.opt_str("m") {
            self._asso_iterations = x.parse().unwrap();
            if self._asso_iterations < 0 {
                eprint!("asso_iterations value must not be negative, assuming 0\n");
                self._asso_iterations = 0;
            }
        }
    
        if matches.opt_present("n") {
            self._option_word |= NOLENGTH;
        }
    
        if let Some(x) = matches.opt_str("N") {
            self._function_name = x;
        }
    
        if matches.opt_present("o") {
    
        }
        
        if matches.opt_present("O") {
            
        }
    
        if matches.opt_present("p") {
            
        }
    
        if matches.opt_present("P") {
            self._option_word |= SHAREDLIB;
        }
    
        if let Some(x) = matches.opt_str("Q") {
            self._stringpool_name = x;
        }
    
        if matches.opt_present("r") {
            self._option_word = RANDOM;
            if (self._initial_asso_value != 0) {
                eprint!("warning, -r option supersedes -i, disabling -i option and continuing\n");
            }
        }
    
    
        if let Some(x) = matches.opt_str("s") {
            
            let mut numerator: f32 = 0.0;
            let mut denominator: f32 = 1.0;
            let mut invalid: bool = false;
    
            let nums: Vec<&str> = x.split("/").collect();
            
            if !(nums.len() <= 1 || nums.len() >= 3) {
                invalid = true;
            } else {
                    numerator = match nums[0].parse::<f32>() {
                    Ok(num) => num,
                    Err(_) => {invalid = true; -1.0}
                };
    
                denominator = match nums[1].parse::<f32>() {
                    Ok(num) => num,
                    Err(_) => {invalid = true; 0.0}
                };
                
            }
    
            if (invalid ) {
                eprint!("Invalid value for option -s.\n");
                self.short_usage("stderr", &program_name);
                std::process::exit(1);
            }
    
            self._size_multiple = numerator / denominator;
            /* Backward compatibility: -3 means 1/3.  */
            if (self._size_multiple < 0.0){
                self._size_multiple = 1.0 / (- self._size_multiple); 
            }
            /* Catch stupid users.  */
            if (self._size_multiple == 0.0) {
                self._size_multiple = 1.0;
            }
            /* Warnings.  */
            if (self._size_multiple > 50.0) {
                eprint!("Size multiple {} is excessive, did you really mean this?! (try '{} --help' for help)\n", self._size_multiple, program_name);
            }
            
            else if (self._size_multiple < 0.01) {
                eprint!("Size multiple {} is extremely small, did you really mean this?! (try '{} --help' for help)\n", self._size_multiple, program_name);
            }
        }
    
        if let Some(x) = matches.opt_str("S") {
            self._option_word |= SWITCH;
            self._total_switches = x.parse().unwrap();
    
            if (self._total_switches <= 0) {
                eprint!("number of switches {} must be a positive number\n", x);
                self.short_usage("stderr", &program_name);
                std::process::exit(1);
            }
    
        } 
    
        if matches.opt_present("t") {
            self._option_word |= TYPE;
        }
    
        if matches.opt_present("T") {
            self._option_word |= NOTYPE;
        }
    
        if matches.opt_present("v") {
            print!("GNU gperf {}\n", VERSION);
            print!("Copyright (C) {} Free Software Foundation, Inc.\n\
                    License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>\n\
                    This is free software: you are free to change and redistribute it.\n\
                    There is NO WARRANTY, to the extent permitted by law.\n\
                            ",
                    "1989-1998, 2000-2004, 2006-2009");
            print!("Written by {} and {}.\n",
                    "Douglas C. Schmidt", "Bruno Haible");
            std::process::exit(0);        
        }
    
    
        if let Some(x) = matches.opt_str("W") {
            self._wordlist_name = x;
        }
    
        if let Some(x) = matches.opt_str("Z") {
            self._class_name = x;
        }
    
        if let Some(x) = matches.opt_str("7") {
            self._option_word |= SEVENBIT;
        }
    
        if let Some(x) = matches.opt_str(std::char::from_u32(CHAR_MAX + 1).unwrap().encode_utf8(&mut [0; 4])) {
            self._output_file_name = x;
        }
    
        if matches.opt_present(std::char::from_u32(CHAR_MAX + 2).unwrap().encode_utf8(&mut [0; 4])) {
            self._option_word |= UPPERLOWER;
        }
    
        if matches.opt_present(std::char::from_u32(CHAR_MAX + 3).unwrap().encode_utf8(&mut [0; 4])) {
            self._option_word |= NULLSTRINGS;
        }
    
        if let Some(x) = matches.opt_str(std::char::from_u32(CHAR_MAX + 4).unwrap().encode_utf8(&mut [0; 4])) {
            self._lengthable_name = x;
        }
    

    }

    /* Prints the given options.  */
    pub fn print_options(&mut self) {
        print!("/* Command-line: ");

        let mut i: i32 = 0;
        while (i < self._argument_count) {

            let mut arg: &str = self._argument_vector[i as usize].as_str();
            let mut arg_idx: usize = 0;
            /* Escape arg if it contains shell metacharacters.  */
            if (arg.chars().nth(arg_idx).unwrap() == '-') {

                print!("{}", arg.chars().nth(arg_idx).unwrap());
                arg_idx += 1;
                let mut x = arg.chars().nth(arg_idx).unwrap();
                if(x >= 'A' && x <= 'Z' || x >= 'a' && x <= 'z') {
                    print!("{}", x);
                    arg_idx += 1;
                } else if(x == '-') {
                    loop {
                        print!("{}", x);
                        arg_idx += 1;
                        x = arg.chars().nth(arg_idx).unwrap();
                        if(x >= 'A' && x <= 'Z' || x >= 'a' && x <= 'z' || x == '-') {
                            break;
                        }
                    }
                    x = arg.chars().nth(arg_idx).unwrap();
                    if(x == '=') {
                        print!("{}", x);
                        arg_idx += 1;
                    }
                } 
            }

            if(strpbrk (arg, "\t\n !\"#$&'()*;<>?[\\]`{|}~") != None) {
                if arg.find('\\') != None {
                    print!("\"");
                    let x = arg.chars().nth(arg_idx).unwrap();
                    while (x != '\0') {
                        if (x == '\"' || x == '\\' || x == '$' || x == '`') {
                            print!("\\");
                        }
                        print!("\"");
                        arg_idx += 1;
                    }

                    print!("\"");
                } else {
                    print!("\'");
                    let mut x = arg.chars().nth(arg_idx).unwrap();
                    while x !='\0' {
                        x = arg.chars().nth(arg_idx).unwrap();
                        if (x == '\\') {
                            print!("\\");
                        }
                        print!("{}", x);
                        arg_idx += 1;
                    }
                    print!("\'");
                }
 
            } else {
                print!("{}", arg);
            }

            print!(" ");

            i += 1;
        }

        print!(" */");
    }

    /* Accessors.  */

    
    /* Sets a given boolean option.  */
    #[inline]
    pub fn set(&mut self, mut val: i32) {
        self._option_word |= val;
    }

    /* Returns the input file name.  */
    #[inline]
    pub fn get_input_file_name(&self) -> &str {
        return &self._input_file_name;
    }

    /* Returns the output file name.  */
    #[inline]
    pub  fn get_output_file_name(&self) -> &str {
        return &self._output_file_name;
    }

    /* Sets the output language, if not already set.  */
    pub fn set_language(&mut self, language: &String) {
        if self._language == "" {
            self._language = language.clone();
            self._option_word &= !(KRC | C | ANSIC | CPLUSPLUS as i32);
            if language == "KR-C" {
                self._option_word |= KRC;
            }
                
            else if language == "C" {
                self._option_word |= C;
            }
                
            else if language == "ANSI-C" {
                self._option_word |= ANSIC;
            }
                
            
            else if language == "C++" {
                self._option_word |= CPLUSPLUS;
            }
                
            else {
                eprintln!("unsupported language option {}, defaulting to C\n",
                        language);
                        self._option_word |= C;
            }
        }
    }

    /* Returns the jump value.  */
    #[inline]
    pub  fn get_jump(&self) -> i32 {
        return self._jump;
    } 

    /* Returns the initial associated character value.  */
    #[inline]
    pub  fn get_initial_asso_value(&self) -> i32 {
        return self._initial_asso_value;
    }

    /* Returns the number of iterations for finding good asso_values.  */
    #[inline]
    pub  fn get_asso_iterations(&self) -> i32 {
        return self._asso_iterations;
    }

    /* Returns the total number of switch statements to generate.  */
    #[inline]
    pub  fn get_total_switches(&self) -> i32 {
        return self._total_switches;
    }

    /* Sets the total number of switch statements, if not already set.  */
    pub fn set_total_switches(&mut self, mut total_switches: i32) {
        if ((self._option_word & SWITCH) == 0)
        {
            self._option_word |= SWITCH;
            self._total_switches = total_switches;
        }
    }

    /* Returns the factor by which to multiply the generated table's size.  */
    #[inline]
    pub  fn get_size_multiple(&self) -> f32 {
        return self._size_multiple;
    }

    /* Returns the generated function name.  */
    #[inline]
    pub fn get_function_name(&self) -> &str {
        return &self._function_name;
    }

    /* Sets the generated function name, if not already set.  */
    pub fn set_function_name(&mut self, name: &String) {
        
        if (self._function_name == String::from(DEFAULT_FUNCTION_NAME)) {
            self._function_name = name.clone();
        }
            
    }

    /* Returns the keyword key name.  */
    #[inline]
    pub fn get_slot_name(&self) -> &str {
        return &self._slot_name;
    }

    /* Sets the keyword key name, if not already set.  */
    pub fn set_slot_name(&mut self, name: &String) {
        
        if (self._slot_name == DEFAULT_SLOT_NAME) {
            self._slot_name = name.clone();
        }
        
    }

    /* Returns the struct initializer suffix.  */
    #[inline]
    pub  fn get_initializer_suffix(&self) -> &str {
        return &self._initializer_suffix;
    } 

    /* Sets the struct initializer suffix, if not already set.  */
    pub fn set_initializer_suffix(&mut self, mut initializers: &String) {
        
        if (self._initializer_suffix == DEFAULT_INITIALIZER_SUFFIX){
            self._initializer_suffix = initializers.clone();
        }
            
    }

    /* Returns the generated class name.  */
    #[inline]
    pub  fn get_class_name(&self) -> &str {
        return &self._class_name;
    }

    /* Sets the generated class name, if not already set.  */
    pub fn set_class_name(&mut self, name: &String) {

        if (self._class_name == DEFAULT_CLASS_NAME) {
            self._class_name = name.clone();
        }

    }

    /* Returns the hash function name.  */
    #[inline]
    pub  fn get_hash_name(&self) -> &str {
        return &self._hash_name;
    }

    /* Sets the hash function name, if not already set.  */
    pub fn set_hash_name(&mut self, name: &String) {
        
        if (self._hash_name == DEFAULT_HASH_NAME) {
            self._hash_name = name.clone();
        }
    }

    /* Returns the hash table array name.  */
    #[inline]
    pub  fn get_wordlist_name(&self) -> &str {
        return &self._wordlist_name;
    }

    /* Sets the hash table array name, if not already set.  */
    pub fn set_wordlist_name(&mut self, mut name: &String) {
        
        if (self._wordlist_name == DEFAULT_WORDLIST_NAME) {
            self._wordlist_name = name.clone();
        }
    
    }

    /* Returns the length table array name.  */
    pub  fn get_lengthable_name(&self) -> &str {
        return &self._lengthable_name;
    }

    /* Sets the length table array name, if not already set.  */
    pub fn set_lengthable_name(&mut self, name: &String) {

        if (self._lengthable_name == DEFAULT_LENGTHTABLE_NAME) {
            self._lengthable_name = name.clone();
        }

    }

    /* Returns the string pool name.  */
    #[inline]
    pub  fn get_stringpool_name(&self) -> &str {
        return &self._stringpool_name;
    }

    /* Sets the string pool name, if not already set.  */
    pub fn set_stringpool_name(&mut self, mut name: &String) {
        
        if (self._stringpool_name == DEFAULT_STRINGPOOL_NAME) {
            self._stringpool_name = name.clone();
        }
    }

    /* Returns the string used to delimit keywords from other attributes.  */
    #[inline]
    pub  fn get_delimiters(&self) -> &str {
        return &self._delimiters;
    }

    /* Sets the delimiters string, if not already set.  */
    pub fn set_delimiters(&mut self, delimiters: &String) {

        if (self._delimiters == DEFAULT_DELIMITERS) {
            self._delimiters = delimiters.clone();
        }
                
    }

    
    /* Returns key positions.  */
    #[inline]
    pub const fn get_key_positions(&self) -> &Positions {
        return &self._key_positions;
    }

    /* Prints program usage to given stream.  */
    fn short_usage(&self, stream: &str, program_name: &String) {

        if stream == "stderr" {
            let stderr = std::io::stderr();
            let mut handle = stderr.lock();
            handle.write_all(format!("Try '{} --help' for more information.\n", program_name).as_bytes()).unwrap();
            handle.flush(); 
        } else if stream == "stdout" {
            let stdout = std::io::stdout();
            let mut handle = stdout.lock();
            handle.write_all(format!("Try '{} --help' for more information.\n", program_name).as_bytes()).unwrap();
            handle.flush();
        }
    }

    /* Prints program usage to given stream.  */
    fn long_usage(&self,  mut stream: &str, program_name: &String) {

        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        handle.write_all(b"GNU 'gperf' generates perfect hash functions.\n");
        handle.write_all(b"\n");
        handle.write_all(format!("Usage: {} [OPTION]... [INPUT-FILE]\n", program_name).as_bytes());
        handle.write_all( b"\n");
        handle.write_all( b"If a long option shows an argument as mandatory, then it is mandatory\n for the equivalent short option also.\n");
        handle.write_all( b"\n");
        handle.write_all( b"Output file location:\n");
        handle.write_all( b"      --output-file=FILE Write output to specified file.\n");
        handle.write_all( b"The results are written to standard output if no output file is specified\n or if it is -.\n");
        handle.write_all( b"\n");
        handle.write_all( b"Input file interpretation:\n");
        handle.write_all( b"  -e, --delimiters=DELIMITER-LIST\n                         Allow user to provide a string containing delimiters\n                         used to separate keywords from their attributes.\n                         Default is \",\".\"\n");
        handle.write_all( b"  -t, --struct-type      Allows the user to include a structured type\n                         declaration for generated code. Any text before %%%%\n                         is considered part of the type declaration. Key\n                         words and additional fields may follow this, one\n                         group of fields per line.\n");    
        handle.write_all( b"      --ignore-case      Consider upper and lower case ASCII characters as\n                         equivalent. Note that locale dependent case mappings\n                         are ignored.\n");
        handle.write_all( b"\n");
        handle.write_all( b"Language for the output code:\n");
        handle.write_all( b"  -L, --language=LANGUAGE-NAME\n                         Generates code in the specified language. Languages\n                         handled are currently C++, ANSI-C, C, and KR-C. The                         default is C.\n");
        handle.write_all( b"/n");
        handle.write_all( b"Details in the output code:\n");
        handle.write_all( b"  -K, --slot-name=NAME   Select name of the keyword component in the keyword\n                         structure.\n");
        handle.write_all( b"  -F, --initializer-suffix=INITIALIZERS\n                         Initializers for additional components in the keyword\n                         structure.\n");
        handle.write_all( b"  -H, --hash-function-name=NAME\n                         Specify name of generated hash function. Default is\n                         'hash'.\n");
        handle.write_all( b"  -N, --lookup-function-name=NAME\n                         Specify name of generated lookup function. Default\n                         name is 'in_word_set'.\n");
        handle.write_all( b"  -Z, --class-name=NAME  Specify name of generated C++ class. Default name is\n                         'Perfect_Hash'.\n");
        handle.write_all( b"  -7, --seven-bit        Assume 7-bit characters.\n");
        handle.write_all( b"  -l, --compare-lengths  Compare key lengths before trying a string\n                         comparison. This is necessary if the keywords\n                         contain NUL bytes. It also helps cut down on the\n                         number of string comparisons made during the lookup.\n");
        handle.write_all( b"  -c, --compare-strncmp  Generate comparison code using strncmp rather than\n                         strcmp.\n");
        handle.write_all( b"  -C, --readonly-tables  Make the contents of generated lookup tables\n                         constant, i.e., readonly.\n");
        handle.write_all( b"  -E, --enum             Define constant values using an enum local to the\n                         lookup function rather than with defines.\n");
        handle.write_all( b"  -I, --includes         Include the necessary system include file <string.h>\n                         at the beginning of the code.\n");
        handle.write_all( b"  -G, --global-table     Generate the static table of keywords as a static\n                         global variable, rather than hiding it inside of the\n                         lookup function (which is the default behavior).\n");
        handle.write_all( b"  -P, --pic              Optimize the generated table for inclusion in shared\n                         libraries.  This reduces the startup time of programs\n                         using a shared library containing the generated code.\n");
        handle.write_all( b"  -Q, --string-pool-name=NAME\n                         Specify name of string pool generated by option --pic.\n                         Default name is 'stringpool'.\n");
        handle.write_all( b"      --null-strings     Use NULL strings instead of empty strings for empty\n                         keyword table entries.\n");
        handle.write_all( b"  -W, --word-array-name=NAME\n                         Specify name of word list array. Default name is\n                         'wordlist'.\n");
        handle.write_all( b"  -S, --switch=COUNT     Causes the generated C code to use a switch\n                         statement scheme, rather than an array lookup table.\n                         This can lead to a reduction in both time and space\n                         requirements for some keyfiles. The COUNT argument\n                         determines how many switch statements are generated.\n                         A value of 1 generates 1 switch containing all the\n                         elements, a value of 2 generates 2 tables with 1/2\n                         the elements in each table, etc. If COUNT is very\n                         large, say 1000000, the generated C code does a\n                         binary search.\n");
        handle.write_all( b"  -T, --omit-struct-type\n                         Prevents the transfer of the type declaration to the\n                         output file. Use this option if the type is already\n                         defined elsewhere.\n");
        handle.write_all( b"\n");
        handle.write_all( b"Algorithm employed by gperf:\n");
        handle.write_all( format!("  -k, --key-positions=KEYS\n                         Select the key positions used in the hash function.\n                         The allowable choices range between 1-{}, inclusive.\n                         The positions are separated by commas, ranges may be\n                         used, and key positions may occur in any order.\n                         Also, the meta-character '*' causes the generated\n                         hash function to consider ALL key positions, and $\n                         indicates the \"final character\" of a key, e.g.,\n                         $,1,2,4,6-10.\n", Positions::MAX_KEY_POS).as_bytes());
        handle.write_all( b"  -D, --duplicates       Handle keywords that hash to duplicate values. This\n                         is useful for certain highly redundant keyword sets.\n");
        handle.write_all( b"  -m, --multiple-iterations=ITERATIONS\n                         Perform multiple choices of the -i and -j values,\n                         and choose the best results. This increases the\n                         running time by a factor of ITERATIONS but does a\n                         good job minimizing the generated table size.\n");
        handle.write_all( b"  -i, --initial-asso=N   Provide an initial value for the associate values\n                         array. Default is 0. Setting this value larger helps\n                         inflate the size of the final table.\n");
        handle.write_all( format!("  -j, --jump=JUMP-VALUE  Affects the \"jump value\", i.e., how far to advance\n                         the associated character value upon collisions. Must\n                         be an odd number, default is {}.", DEFAULT_JUMP_VALUE).as_bytes());
        handle.write_all( b"  -n, --no-strlen        Do not include the length of the keyword when\n                         computing the hash function.\n");
        handle.write_all( b"  -r, --random           Utilizes randomness to initialize the associated\n                         values table.\n");
        handle.write_all( b"  -s, --size-multiple=N  Affects the size of the generated hash table. The\n                         numeric argument N indicates \"how many times larger\n                         or smaller\" the associated value range should be,\n                         in relationship to the number of keys, e.g. a value\n                         of 3 means \"allow the maximum associated value to\n                         be about 3 times larger than the number of input\n                         keys\". Conversely, a value of 1/3 means \"make the\n                         maximum associated value about 3 times smaller than\n                         the number of input keys\". A larger table should\n                         decrease the time required for an unsuccessful\n                         search, at the expense of extra table space. Default\n                         value is 1.\n");
        handle.write_all( b"\n");
        handle.write_all( b"Informative output:\n  -h, --help             Print this message.\n  -v, --version          Print the gperf version number.\n  -d, --debug            Enables the debugging option (produces verbose\n                         output to the standard error).\n" );
        handle.write_all( b"\n");
        handle.write_all( b"Report bugs to <bug-gnu-gperf@gnu.org>.\n");
    }

}

/* Tests a given boolean option.  Returns true if set, false otherwise.  */

/* Dumps option status when debugging is enabled.  */
impl Drop for Options1 {
    fn drop(&mut self) {
        if (self._option_word & DEBUG as i32) != 0 {
            eprintln!("\ndumping Options:
\nTYPE is........: {}
\nUPPERLOWER is..: {}
\nKRC is.........: {}
\nC is...........: {}
\nANSIC is.......: {}
\nCPLUSPLUS is...: {}
\nSEVENBIT is....: {}
\nLENTABLE is....: {}
\nCOMP is........: {}
\nCONST is.......: {}
\nENUM is........: {}
\nINCLUDE is.....: {}
\nGLOBAL is......: {}
\nNULLSTRINGS is.: {}
\nSHAREDLIB is...: {}
\nSWITCH is......: {}
\nNOTYPE is......: {}
\nDUP is.........: {}
\nNOLENGTH is....: {}
\nRANDOM is......: {}
\nDEBUG is.......: {}
\nlookup function name = {} 
\nhash function name = {}
\nword list name = {}
\nlength table name = {}
\nstring pool name = {}
\nslot name = {}
\ninitializer suffix = {}
\nasso_values iterations = {}
\njump value = {}
\nhash table size multiplier = {}
\ninitial associated value = {}
\ndelimiters = {}
\nnumber of switch statements = {}",
            if self._option_word & TYPE != 0 {"enabled"} else {"disabled"},
            if self._option_word & UPPERLOWER != 0 {"enabled"} else {"disabled"},
            if self._option_word & KRC != 0 {"enabled"} else {"disabled"},
            if self._option_word & C != 0 {"enabled"} else {"disabled"},
            if self._option_word & ANSIC != 0 {"enabled"} else {"disabled"},
            if self._option_word & CPLUSPLUS != 0 {"enabled"} else {"disabled"},
            if self._option_word & SEVENBIT != 0 {"enabled"} else {"disabled"},
            if self._option_word & LENTABLE != 0 {"enabled"} else {"disabled"},
            if self._option_word & COMP != 0 {"enabled"} else {"disabled"},
            if self._option_word & CONST != 0 {"enabled"} else {"disabled"},
            if self._option_word & ENUM != 0 {"enabled"} else {"disabled"},
            if self._option_word & INCLUDE != 0 {"enabled"} else {"disabled"},
            if self._option_word & GLOBAL != 0 {"enabled"} else {"disabled"},
            if self._option_word & NULLSTRINGS != 0 {"enabled"} else {"disabled"},
            if self._option_word & SHAREDLIB != 0 {"enabled"} else {"disabled"},
            if self._option_word & SWITCH != 0 {"enabled"} else {"disabled"},
            if self._option_word & NOTYPE != 0 {"enabled"} else {"disabled"},
            if self._option_word & DUP != 0 {"enabled"} else {"disabled"},
            if self._option_word & NOLENGTH != 0 {"enabled"} else {"disabled"},
            if self._option_word & RANDOM != 0 {"enabled"} else {"disabled"},
            if self._option_word & DEBUG != 0 {"enabled"} else {"disabled"},
            self._function_name, self._hash_name, self._wordlist_name, self._lengthable_name,
            self._stringpool_name, self._slot_name, self._initializer_suffix,
            self._asso_iterations, self._jump, self._size_multiple, self._initial_asso_value,
            self._delimiters, self._total_switches);

            if (self._key_positions.is_useall()) {
                eprintln!("all characters are used ni the hash function");
            } else {
                eprintln!("maximum keysig = {}\nkey positions are : ", self._key_positions.get_size());

                let mut iter: PositionIterator = self._key_positions.iterator();
                let mut pos: i32;
                pos = iter.next();
                while (pos != PositionIterator::EOS) {
                    if (pos == Positions::LASTCHAR) {
                        eprintln!("$");
                    } else {
                        eprintln!("{}", pos + 1);
                    }
                    pos = iter.next();
                }
            }

            eprintln!("finished dumping Options\n");
        }
    }
}

#[derive(Default)]
struct PositionStringParser {

    /* A pointer to the string provided by the user.  */
    _str: String,
    _str_index: usize,

    /* Smallest possible value, inclusive.  */
    _low_bound: i32,

    /* Greatest possible value, inclusive.  */
    _high_bound: i32,

    /* A value marking the abstract "end of word" ( usually '$').  */
    _end_word_marker: i32,

    /* Error value returned when input is syntactically erroneous.  */
    _error_value: i32,

    /* Value returned after last key is processed.  */
    _end_marker: i32,

    /* Intermediate state for producing a range of positions.  */
    _in_range: bool, /* True while producing a range of positions.  */
    _range_upper_bound: i32, /* Upper bound (inclusive) of the range.  */
    _range_curr_value: i32 /* Last value returned.  */

}


impl PositionStringParser {

    /* Initializes a key position strng parser for string STR.  */
    pub fn new(s: &str, 
                low_bound: i32, high_bound: i32, 
                end_word_marker: i32, error_value: i32, end_marker: i32) -> PositionStringParser {

        PositionStringParser {
            _str: s.to_string(),
            _str_index: 0,
            _low_bound: low_bound,
            _high_bound: high_bound,
            _end_word_marker: end_word_marker,
            _error_value: error_value,
            _end_marker: end_marker,
            _in_range: false,
            _range_upper_bound: -1,
            _range_curr_value: -1
        }            
        
    }

    /* Returns the next key position from the given string.  */
    pub fn nextPosition(&mut self) -> i32 {

        
        if(self._in_range) {
            /* We are inside a range.  Return the next value from the range.  */
            self._range_curr_value += 1;
            if(self._range_curr_value >= self._range_upper_bound) {
                self._in_range = false;
            }
            return self._range_curr_value;
        } else {
            /* Continue parsing the given string.  */
            while(self._str_index < self._str.len()) {
                match self._str.chars().nth(self._str_index).unwrap() {

                    ',' => 
                        /* Skip the comma. */
                        self._str_index += 1,

                    '$' => 
                    /* Valid key position.  */ 
                    { 
                        
                        self._str_index += 1;
                        return self._end_word_marker;
                    }

                    '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => 
                    /* Valid key position.  */
                    {
                        let mut curr_value: i32 = 0;
                        while(self._str.chars().nth(self._str_index).unwrap() >= '0' && self._str.chars().nth(self._str_index).unwrap() <= '9') {

                            curr_value = curr_value * 10 + (self._str.chars().nth(self._str_index).unwrap() as u8 - '0' as u8) as i32;
                            self._str_index += 1;
                        }
                        
                        
                        if(self._str.chars().nth(self._str_index).unwrap() == '-') {
                            self._str_index += 1;
                            /* Starting a range of key positions.  */
                            self._in_range = true;

                            self._range_upper_bound = 0;
                            while (self._str.chars().nth(self._str_index).unwrap() >= '0' && self._str.chars().nth(self._str_index).unwrap() <= '9') {
                                self._range_upper_bound = self._range_upper_bound * 10 + (self._str.chars().nth(self._str_index).unwrap() as u8 - '0' as u8) as i32;
                                self._str_index += 1;
                            }

                            /* Verify range's upper bound.  */
                            if (!(self._range_upper_bound > curr_value && self._range_upper_bound <= self._high_bound)) {
                                return self._error_value;
                            }
                            self._range_curr_value = curr_value;
                        }

                        /* Verify range's lower bound.  */
                        if (!(curr_value >= self._low_bound && curr_value <= self._high_bound)) {
                            return self._error_value;
                        }
                        return curr_value;
                    }
  
                    _ => /*Invalid syntax. */
                        return self._error_value,


                }

            }
            return self._end_marker;
        }

    }

}
