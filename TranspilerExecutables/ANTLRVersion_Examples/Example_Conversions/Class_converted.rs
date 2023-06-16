// Using Namespace directives are not yet supported in this transpiler... Copying as it is
// using namespace std ;
#[derive(Default)]
pub struct Rectangle {
    width: i32,
    myRandomProp: i32,
    height: i32,
}
impl Rectangle {
    pub fn area(&mut self) -> i32 {
        return self.width * self.height;
    }
    pub fn perimeter(&mut self) -> i32 {
        return 2 * (self.width + self.height);
    }
    pub fn setWidth(&mut self, w: i32) -> i32 {
        self.width = w;
        return w;
    }
    pub fn setHeight(&mut self, h: i32) -> i32 {
        self.height = h;
        return h;
    }
    pub fn new() -> Rectangle {
        println!("Do nothing\n");
        self.width = 5;
        self.height = 9;
        self.myRandomProp = 10;

        /*
            This is a constructor method.
            Please appropriate members to the struct constructor as per your logic.
            Currently the constructor returns a struct with all the defaults for the data types in the struct.
        */
        Rectangle {
            ..Default::default()
        }
    }
}
fn main() {
    let mut r = Rectangle::new;
    r.setWidth(4);
    r.setHeight(5);
    println!("Area of r: {}\n", r.area());
    println!("Perimeter of r: {}\n", r.perimeter());
    r.setWidth(6);
    r.setHeight(7);
    println!("New area of r: {}\n", r.area());
    println!("New perimeter of r: {}\n", r.perimeter());
    let mut r2 = Rectangle::new;
    r2.setWidth(5);
    r2.setHeight(9);
    println!("Area of r2: {}\n", r2.area());
    println!("Perimeter of r2: {}\n", r2.perimeter());
    r2.setWidth(10);
    r2.setHeight(11);
    println!("New area of r2: {}\n", r2.area());
    println!("New perimeter of r2: {}\n", r2.perimeter());
    return 0;
}
