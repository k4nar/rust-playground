use std::f64::consts::PI;


trait Shape {
  fn area(&self) -> f64;
  fn print_area(&self) { println!("My area is {}.", self.area()) }
}

struct Circle {
  radius: f64
}

impl Shape for Circle {
  fn area(&self) -> f64 { (self.radius * PI).pow(&2.) }
}

fn main() {
  let circle = Circle { radius: 2. };
  circle.print_area();
}