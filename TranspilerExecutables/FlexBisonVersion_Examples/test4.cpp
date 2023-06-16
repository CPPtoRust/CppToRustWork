class Car
{
    int m_Colour;
    int m_Make;
    int m_Model;
    int m_Speed;
    int m_Direction;

    Car()
    {
        cout<<"new object";
    }

    void PrintCurrentCar()
    {
        cout << "COLOUR:     ";
        this.GetColour();
        cout << endl
             << "MAKE:       ";
        this.GetMake();
        cout << endl
             << "DIRECTION:  ";
        this.GetDirection();
        cout << endl
             << "MODEL:      ";
        this.GetModel();
        cout << endl
             << "SPEED:      ";
        this.GetSpeed();
        cout << endl
             << "-----------------------------------------" << endl;
    }
    bool GetColour()
    {
        switch (this.m_Colour)
        {
        case 1:
            cout << "Red";
            break;
        case 2:
            cout << "Blue";
            break;
        case 3:
            cout << "Green";
            break;
        case 4:
            cout << "Yellow";
            break;
        default:
            cout << "Invalid";
            return false;
        }
        return true;
    }
    void SetColour(int val)
    {
        if ((val > 4) || (val < 1))
        {
            cout << "Error assigning colour. Use numbers 1-4 only";
        }
        else
        {
            this.m_Colour = val;
        }
    }

    bool GetMake()
    {
        switch (this.m_Make)
        {
        case 1:
            cout << "Ford";
            break;
        case 2:
            cout << "Honda";
            break;
        case 3:
            cout << "Toyota";
            break;
        case 4:
            cout << "Pontiac";
            break;
        default:
            cout << "Invalid";
            return false;
        }
        return true;
    }
    void SetMake(int val)
    {
        if ((val > 4) || (val < 1))
        {
            cout << "Error assigning Make. Use numbers 1-4 only";
        }
        else
        {
            this.m_Make = val;
        }
    }
    bool GetModel()
    {
        switch (this.m_Model)
        {
        case 1:
            cout << "Truck";
            break;
        case 2:
            cout << "Car";
            break;
        case 3:
            cout << "Van";
            break;
        case 4:
            cout << "Super Bike";
            break;
        default:
            cout << "Invalid";
            return false;
        }
        return true;
    }
    void SetModel(int val)
    {
        if ((val > 4) || (val < 1))
        {
            cout << "Error assigning Model. Use numbers 1-4 only";
        }
        else
        {
            this.m_Model = val;
        }
    }
    bool GetSpeed()
    {
        cout << this.m_Speed << "km/h";
        return true;
    }
    void SetSpeed(int val)
    {
        if (val > 200)
        {
            cout << "Too fast (max speed: 200km/h)";
        }
        else
        {
            this.m_Speed = val;
        }
    }
    bool GetDirection()
    {
        switch (this.m_Direction)
        {
        case 1:
            cout << "North";
            break;
        case 2:
            cout << "East";
            break;
        case 3:
            cout << "South";
            break;
        case 4:
            cout << "West";
            break;
        default:
            cout << "Invalid";
            return false;
        }
        return true;
    }
    void SetDirection(int val)
    {
        this.m_Direction = val;
    }
    void ChangeDirection(int val)
    {
        int tempDirection = this.m_Direction + val;
        if (tempDirection > 4)
        {
            tempDirection -= 4;
        }
        if (tempDirection < 1)
        {
            tempDirection += 4;
        }
        this.SetDirection(tempDirection);
    }
}

int main()
{

    Car myCar;
    int randomCrapIDontCareAbout;
    cout << "OOP Code Challenge - Cars" << endl;

    myCar.SetColour(4);
    myCar.SetMake(3);
    myCar.SetDirection(2);
    myCar.SetModel(1);
    myCar.SetSpeed(0);

    cout << "Initial Values:" << endl;
    myCar.PrintCurrentCar();

    cout << "Step 1 - From rest, accelerate to 60km/h" << endl;
    myCar.SetSpeed(60);
    myCar.PrintCurrentCar();

    cout << "Step 2 - Make a 90-degree left-hand turn" << endl;
    myCar.ChangeDirection(1);
    myCar.PrintCurrentCar();

    cout << "Step 3 - Make a 90-degree right-hand turn" << endl;
    myCar.ChangeDirection(1);
    myCar.PrintCurrentCar();

    cout << "Step 4 - Accelerate to 90km/h" << endl;
    myCar.SetSpeed(90);
    myCar.PrintCurrentCar();

    cout << "Step 5 - Brake to 0km/h" << endl;
    myCar.SetSpeed(0);
    myCar.PrintCurrentCar();
}