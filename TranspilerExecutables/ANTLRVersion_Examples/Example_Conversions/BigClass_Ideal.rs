#[derive(Default)]
pub struct Car {
    m_Colour: i32,
    m_Make: i32,
    m_Model: i32,
    m_Speed: i32,
    m_Direction: i32,
}
impl Car {
    pub fn new() -> Car {
        // SetColour(1);
        // SetDirection(1);
        // SetMake(1);
        // SetModel(1);

        /*
            This is a constructor method.
            Please appropriate members to the struct constructor as per your logic.
            Currently the constructor returns a struct with all the defaults for the data types in the struct.
        */
        Car {

			m_Colour: 1,
			m_Direction: 1,
			m_Make : 1,
			m_Model: 1,
            m_Speed: 0,
        }
    }
    pub fn PrintCurrentCar(&mut self) {
        println!("COLOUR:     ");
        self.GetColour();
        println!("\nMAKE:       ");
        self.GetMake();
        println!("\nDIRECTION:  ");
        self.GetDirection();
        println!("\nMODEL:      ");
        self.GetModel();
        println!("\nSPEED:      ");
        self.GetSpeed();
        println!("\n-----------------------------------------\n");
    }
    pub fn GetColour(&mut self) -> i32 {
        match self.m_Colour {
            1 => {
                println!("Red");
            }
            2 => {
                println!("Blue");
            }
            3 => {
                println!("Green");
            }
            4 => {
                println!("Yellow");
            }
            _ => {
                println!("Invalid");
                return 0;
            }
        }
        return 1;
    }
    pub fn SetColour(&mut self, val: i32) {
        if (val > 4) || (val < 1) {
            println!("Error assigning colour. Use numbers 1-4 only");
        } else {
            self.m_Colour = val;
        }
    }
    pub fn GetMake(&mut self) -> i32 {
        match self.m_Make {
            1 => {
                println!("Ford");
            }
            2 => {
                println!("Honda");
            }
            3 => {
                println!("Toyota");
            }
            4 => {
                println!("Pontiac");
            }
            _ => {
                println!("Invalid");
                return 0;
            }
        }
        return 1;
    }
    pub fn SetMake(&mut self, val: i32) {
        if (val > 4) || (val < 1) {
            println!("Error assigning Make. Use numbers 1-4 only");
        } else {
            self.m_Make = val;
        }
    }
    pub fn GetModel(&mut self) -> i32 {
        match self.m_Model {
            1 => {
                println!("Truck");
            }
            2 => {
                println!("Car");
            }
            3 => {
                println!("Van");
            }
            4 => {
                println!("Super Bike");
            }
            _ => {
                println!("Invalid");
                return 0;
            }
        }
        return 1;
    }
    pub fn SetModel(&mut self, val: i32) {
        if (val > 4) || (val < 1) {
            println!("Error assigning Model. Use numbers 1-4 only");
        } else {
            self.m_Model = val;
        }
    }
    pub fn GetSpeed(&mut self) -> i32 {
        println!("{}km/h", self.m_Speed);
        return 1;
    }
    pub fn SetSpeed(&mut self, val: i32) {
        if val > 200 {
            println!("Too fast (max speed: 200km/h)");
        } else {
            self.m_Speed = val;
        }
    }
    pub fn GetDirection(&mut self) -> i32 {
        match self.m_Direction {
            1 => {
                println!("North");
            }
            2 => {
                println!("East");
            }
            3 => {
                println!("South");
            }
            4 => {
                println!("West");
            }
            _ => {
                println!("Invalid");
                return 0;
            }
        }
        return 1;
    }
    pub fn SetDirection(&mut self, val: i32) {
        self.m_Direction = val;
    }
    pub fn ChangeDirection(&mut self, val: i32) {
        let mut tempDirection: i32 = self.m_Direction + val;
        if tempDirection > 4 {
            tempDirection -= 4;
        }
        if tempDirection < 1 {
            tempDirection += 4;
        }
        self.SetDirection(tempDirection);
    }
}
fn main() {
    let mut MyCar = Car::new();
    let mut randomCrapIDontCareAbout: i32;
    println!("OOP Code Challenge - Cars\n");
    MyCar.SetColour(4);
    MyCar.SetMake(3);
    MyCar.SetDirection(2);
    MyCar.SetModel(1);
    MyCar.SetSpeed(0);
    println!("Initial Values:\n");
    MyCar.PrintCurrentCar();
    println!("Step 1 - From rest, accelerate to 60km/h\n");
    MyCar.SetSpeed(60);
    MyCar.PrintCurrentCar();
    println!("Step 2 - Make a 90-degree left-hand turn\n");
    MyCar.ChangeDirection(-1);
    MyCar.PrintCurrentCar();
    println!("Step 3 - Make a 90-degree right-hand turn\n");
    MyCar.ChangeDirection(1);
    MyCar.PrintCurrentCar();
    println!("Step 4 - Accelerate to 90km/h\n");
    MyCar.SetSpeed(90);
    MyCar.PrintCurrentCar();
    println!("Step 5 - Brake to 0km/h\n");
    MyCar.SetSpeed(0);
    MyCar.PrintCurrentCar();

}
