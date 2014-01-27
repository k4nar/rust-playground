
fn main() {
  for i in range(0, 5) {
    do spawn {
      println!("Hello {} !", i * 10);
    }
  }
}