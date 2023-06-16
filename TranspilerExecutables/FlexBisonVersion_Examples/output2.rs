#[derive(Default)]
struct Rectangle {
pub width: i32,
pub height: i32,
}
impl Rectangle{
pub fn area( &mut self ) -> i32 {
return self.width * self.height;
}
pub fn perimeter( &mut self ) -> i32 {
return 2 * ( self.width + self.height );
}
pub fn setWidth( &mut self, w : i32) -> i32 {
self.width = w;
return w;
}
pub fn setHeight( &mut self, h : i32) -> i32 {
self.height = h;
return h;
}
}
pub fn main() {
let mut r: Rectangle;
r.setWidth(4);
r.setHeight(5);
println!("Area of r: ");
println!("{}",r.area());
println!("\n");
println!("Perimeter of r: ");
println!("{}",r.perimeter());
println!("\n");
r.setWidth(6);
r.setHeight(7);
println!("New area of r: ");
println!("{}",r.area());
println!("\n");
println!("New perimeter of r: ");
println!("{}",r.perimeter());
println!("\n");
let mut r2: Rectangle;
r2.setWidth(5);
r2.setHeight(9);
println!("Area of r2: ");
println!("{}",r2.area());
println!("\n");
println!("Perimeter of r2: ");
println!("{}",r2.perimeter());
println!("\n");
r2.setWidth(10);
r2.setHeight(11);
println!("New area of r2: ");
println!("{}",r2.area());
println!("\n");
println!("New perimeter of r2: ");
println!("{}",r2.perimeter());
println!("\n");
}
