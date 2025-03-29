enum Direction {
    North,
    South,
    East,
    West,
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    let ans= match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(size) => size * size,
        Shape::Rectangle(width, height) => width * height
    };

    return ans;
}

fn main() {
    let circle = Shape::Circle(5.0);
    let square = Shape::Circle(4.0);
    let rectangle = Shape::Rectangle(3.0, 2.0);

    print!("Area of circle: {}", calculate_area(circle));
}