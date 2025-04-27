struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        return (self.width + self.height) / 2;
    }
}

fn main() {
    let rect = Rect{
        width: 30,
        height: 50,
    };
    print!("Area of rectangle is {}", rect.area());
    print!("Perimeter of rectangle is {}", rect.perimeter());
}