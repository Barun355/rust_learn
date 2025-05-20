

enum Shape {
    Rectangle(f64, f64),
    Square(f64),
    Circle(f64)
}


pub fn start_enum() {
    let dire = Shape::Circle(10.00);

    let ans = match dire {
        Shape::Circle(radius) => 3.124 * (radius * radius),
        Shape::Square(side) => side * side,
        Shape::Rectangle(length, breadth) => length * breadth
    };

    println!("{}", ans);
}
