//Write a program that prints the numbers from 1 to 100.
// But for multiples of three print “Fizz” instead of the number and
// for the multiples of five print “Buzz”.
// For numbers which are multiples of both three and five print “FizzBuzz

fn main() {
  for i in range(1, 101) {
    match i {
      i if i % 15 == 0 => { println("FizzBuzz") }
      i if i % 3 == 0 => { println("Fizz") }
      i if i % 5 == 0 => { println("Buzz") }
      _ => { println!("{}", i) }
    }
  }
}
