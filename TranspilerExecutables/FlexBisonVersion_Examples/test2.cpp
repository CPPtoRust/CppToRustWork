
class Rectangle {
  int width;
  int height;

  int area() {
    return this.width * this.height;
  }

  int perimeter() {
    return 2 * (this.width + this.height);
  }

  int setWidth(int w) {
    this.width = w;
    return w;
  }

  int setHeight(int h) {
    this.height = h;
    return h;
  }
}

int main() {
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

}