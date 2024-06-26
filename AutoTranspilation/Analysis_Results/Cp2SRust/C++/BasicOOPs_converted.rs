// Using Namespace directives are not yet supported in this transpiler... Copying as it is
// using namespace std ;
#[derive(Default)]
pub struct Geeks {
    id: i32,
}
impl Geeks {
    pub fn new() -> Geeks {
        println!("Default Constructor called\n");
        self.id = -1;

        /*
            This is a constructor method.
            Please appropriate members to the struct constructor as per your logic.
            Currently the constructor returns a struct with all the defaults for the data types in the struct.
        */
        Geeks {
            ..Default::default()
        }
    }
    pub fn new(mut x: i32) -> Geeks {
        println!("Parameterized Constructor called \n");
        self.id = x;

        /*
            This is a constructor method.
            Please appropriate members to the struct constructor as per your logic.
            Currently the constructor returns a struct with all the defaults for the data types in the struct.
        */
        Geeks {
            ..Default::default()
        }
    }
}
fn main() {
    let mut obj1 = Geeks::new();
    println!("Geek id is: {}\n", obj1.id);
    let mut obj2 = Geeks::new(21);
    println!("Geek id is: {}\n", obj2.id);
    return 0;
}
