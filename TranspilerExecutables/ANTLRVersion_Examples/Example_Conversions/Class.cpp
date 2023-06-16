#include <bits/stdc++.h>
using namespace std;

class Rectangle
{
public:
    int area()
    {
        return width * height;
    }
    int width;

    int perimeter()
    {
        return 2 * (width + height);
    }

    int setWidth(int w)
    {
        width = w;
        return w;
    }

    int setHeight(int h)
    {
        height = h;
        return h;
    }

    int myRandomProp;

    Rectangle()
    {
        printf("Do nothing\n");
        width = 5;
        height = 9;
        myRandomProp = 10;
    }

    int height;
};

int main()
{
    Rectangle r;

    r.setWidth(4);
    r.setHeight(5);
    cout << "Area of r: " << r.area() << "\n";
    cout << "Perimeter of r: " << r.perimeter() << "\n";
    r.setWidth(6);
    r.setHeight(7);
    cout << "New area of r: " << r.area() << "\n";
    cout << "New perimeter of r: " << r.perimeter() << "\n";

    Rectangle r2;
    r2.setWidth(5);
    r2.setHeight(9);
    cout << "Area of r2: " << r2.area() << "\n";
    cout << "Perimeter of r2: " << r2.perimeter() << "\n";
    r2.setWidth(10);
    r2.setHeight(11);
    cout << "New area of r2: " << r2.area() << "\n";
    cout << "New perimeter of r2: " << r2.perimeter() << "\n";

    return 0;
}