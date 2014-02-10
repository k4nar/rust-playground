// Write a program that prints the numbers from 1 to 100.
// But for multiples of three, print “Fizz” instead of the number,
// and for multiples of five, print “Buzz”.
// For numbers that are multiples of both three and five,
// print “FizzBuzz”.

fn main() {
  for i in range(1, 101) {
    println(
      match i {
        _ if i % 15 == 0 => ~"FizzBuzz",
        _ if i % 3 == 0 => ~"Fizz",
        _ if i % 5 == 0 => ~"Buzz",
        _ => i.to_str()
      }
    );
  }
}
