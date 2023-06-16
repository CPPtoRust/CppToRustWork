// #include <bits/stdc++.h>
// using namespace std;

class Car
{
public:
    Car()
    {
        // ctor
        SetColour(1);
        SetDirection(1);
        SetMake(1);
        SetModel(1);
    }

    void PrintCurrentCar()
    {
        cout << "COLOUR:     ";
        GetColour();
        cout << endl
             << "MAKE:       ";
        GetMake();
        cout << endl
             << "DIRECTION:  ";
        GetDirection();
        cout << endl
             << "MODEL:      ";
        GetModel();
        cout << endl
             << "SPEED:      ";
        GetSpeed();
        cout << endl
             << "-----------------------------------------" << endl;
    }
    int GetColour()
    {
        switch (m_Colour)
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
            return 0;
        }
        return 1;
    }
    void SetColour(int val)
    {
        if ((val > 4) || (val < 1))
        {
            cout << "Error assigning colour. Use numbers 1-4 only";
        }
        else
        {
            m_Colour = val;
        }
    }

    int GetMake()
    {
        switch (m_Make)
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
            return 0;
        }
        return 1;
    }
    void SetMake(int val)
    {
        if ((val > 4) || (val < 1))
        {
            cout << "Error assigning Make. Use numbers 1-4 only";
        }
        else
        {
            m_Make = val;
        }
    }
    int GetModel()
    {
        switch (m_Model)
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
            return 0;
        }
        return 1;
    }
    void SetModel(int val)
    {
        if ((val > 4) || (val < 1))
        {
            cout << "Error assigning Model. Use numbers 1-4 only";
        }
        else
        {
            m_Model = val;
        }
    }
    int GetSpeed()
    {
        cout << m_Speed << "km/h";
        return 1;
    }
    void SetSpeed(int val)
    {
        if (val > 200)
        {
            cout << "Too fast (max speed: 200km/h)";
        }
        else
        {
            m_Speed = val;
        }
    }
    int GetDirection()
    {
        switch (m_Direction)
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
            return 0;
        }
        return 1;
    }
    void SetDirection(int val)
    {
        m_Direction = val;
    }
    void ChangeDirection(int val)
    {
        int tempDirection = m_Direction + val;
        if (tempDirection > 4)
        {
            tempDirection -= 4;
        }
        if (tempDirection < 1)
        {
            tempDirection += 4;
        }
        SetDirection(tempDirection);
    }

protected:
private:
    int m_Colour;
    int m_Make;
    int m_Model;
    int m_Speed;
    int m_Direction;
};

int main()
{
    Car MyCar;
    int randomCrapIDontCareAbout;
    cout << "OOP Code Challenge - Cars" << endl;

    MyCar.SetColour(4);    // 1-4 only
    MyCar.SetMake(3);      // 1-4 only
    MyCar.SetDirection(2); // 1-4 only
    MyCar.SetModel(1);     // 1-4 only
    MyCar.SetSpeed(0);     // 1-200 only

    cout << "Initial Values:" << endl;
    MyCar.PrintCurrentCar();

    cout << "Step 1 - From rest, accelerate to 60km/h" << endl;
    MyCar.SetSpeed(60);
    MyCar.PrintCurrentCar();

    cout << "Step 2 - Make a 90-degree left-hand turn" << endl;
    MyCar.ChangeDirection(-1);
    MyCar.PrintCurrentCar();

    cout << "Step 3 - Make a 90-degree right-hand turn" << endl;
    MyCar.ChangeDirection(1);
    MyCar.PrintCurrentCar();

    cout << "Step 4 - Accelerate to 90km/h" << endl;
    MyCar.SetSpeed(90);
    MyCar.PrintCurrentCar();

    cout << "Step 5 - Brake to 0km/h" << endl;
    MyCar.SetSpeed(0);
    MyCar.PrintCurrentCar();

    return 0;
}
