use std::io::buffered::BufferedReader;
use std::io;
use std::rand;

fn main() {
  let goal = (rand::random::<int>() % 100).abs() + 1;

  let mut reader = BufferedReader::new(io::stdin());

  loop {
    println("Guess :");
    let input = reader.read_line().unwrap_or(~"nothing");
    let guess = from_str::<int>(input.slice_to(input.len() - 1));

    match guess {
      Some(guess) if guess < goal => { println("Moar !") }
      Some(guess) if guess > goal => { println("Less !") }
      Some(guess) if guess == goal => { println("You won !"); break; }
      _ => { println("That's not a number you dumbass."); break; }
    }
  }
}
