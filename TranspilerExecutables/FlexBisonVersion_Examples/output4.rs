#[derive(Default)]
struct Car {
pub m_Colour: i32,
pub m_Make: i32,
pub m_Model: i32,
pub m_Speed: i32,
pub m_Direction: i32,
}
impl Car{
pub fn new() -> Car
{
println!("new object");
return Car{ ..Default::default() }
}
pub fn PrintCurrentCar( &mut self ) {
println!("COLOUR:     ");
self.GetColour();
println!("\n");
println!("MAKE:       ");
self.GetMake();
println!("\n");
println!("DIRECTION:  ");
self.GetDirection();
println!("\n");
println!("MODEL:      ");
self.GetModel();
println!("\n");
println!("SPEED:      ");
self.GetSpeed();
println!("\n");
println!("-----------------------------------------");
println!("\n");
}
pub fn GetColour( &mut self ) -> bool {
match self.m_Colour{
1=>{
println!("Red");
},

2=>{
println!("Blue");
},

3=>{
println!("Green");
},

4=>{
println!("Yellow");
},

_ =>{
println!("Invalid");
return false;
},
}
return true;
}
pub fn SetColour( &mut self, val : i32) {
if ( val > 4 ) == ( val < 1 )
{
println!("Error assigning colour. Use numbers 1-4 only");
}

else 
{
self.m_Colour = val;
}
}
pub fn GetMake( &mut self ) -> bool {
match self.m_Make{
1=>{
println!("Ford");
},

2=>{
println!("Honda");
},

3=>{
println!("Toyota");
},

4=>{
println!("Pontiac");
},

_ =>{
println!("Invalid");
return false;
},
}
return true;
}
pub fn SetMake( &mut self, val : i32) {
if ( val > 4 ) == ( val < 1 )
{
println!("Error assigning Make. Use numbers 1-4 only");
}

else 
{
self.m_Make = val;
}
}
pub fn GetModel( &mut self ) -> bool {
match self.m_Model{
1=>{
println!("Truck");
},

2=>{
println!("Car");
},

3=>{
println!("Van");
},

4=>{
println!("Super Bike");
},

_ =>{
println!("Invalid");
return false;
},
}
return true;
}
pub fn SetModel( &mut self, val : i32) {
if ( val > 4 ) == ( val < 1 )
{
println!("Error assigning Model. Use numbers 1-4 only");
}

else 
{
self.m_Model = val;
}
}
pub fn GetSpeed( &mut self ) -> bool {
println!("{}",self.m_Speed);
println!("km/h");
return true;
}
pub fn SetSpeed( &mut self, val : i32) {
if val > 200
{
println!("Too fast (max speed: 200km/h)");
}

else 
{
self.m_Speed = val;
}
}
pub fn GetDirection( &mut self ) -> bool {
match self.m_Direction{
1=>{
println!("North");
},

2=>{
println!("East");
},

3=>{
println!("South");
},

4=>{
println!("West");
},

_ =>{
println!("Invalid");
return false;
},
}
return true;
}
pub fn SetDirection( &mut self, val : i32) {
self.m_Direction = val;
}
pub fn ChangeDirection( &mut self, val : i32) {
let mut tempDirection: i32 = self.m_Direction + val;
if tempDirection > 4
{
tempDirection = tempDirection - 4;}
if tempDirection < 1
{
tempDirection = tempDirection + 4;}
self.SetDirection(tempDirection);
}
}
pub fn main() {
let mut myCar: Car = Car::new();
let mut randomCrapIDontCareAbout: i32;
println!("OOP Code Challenge - Cars");
println!("\n");
myCar.SetColour(4);
myCar.SetMake(3);
myCar.SetDirection(2);
myCar.SetModel(1);
myCar.SetSpeed(0);
println!("Initial Values:");
println!("\n");
myCar.PrintCurrentCar();
println!("Step 1 - From rest, accelerate to 60km/h");
println!("\n");
myCar.SetSpeed(60);
myCar.PrintCurrentCar();
println!("Step 2 - Make a 90-degree left-hand turn");
println!("\n");
myCar.ChangeDirection(1);
myCar.PrintCurrentCar();
println!("Step 3 - Make a 90-degree right-hand turn");
println!("\n");
myCar.ChangeDirection(1);
myCar.PrintCurrentCar();
println!("Step 4 - Accelerate to 90km/h");
println!("\n");
myCar.SetSpeed(90);
myCar.PrintCurrentCar();
println!("Step 5 - Brake to 0km/h");
println!("\n");
myCar.SetSpeed(0);
myCar.PrintCurrentCar();
}
