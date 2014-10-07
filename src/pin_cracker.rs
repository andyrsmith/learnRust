///Takes a random number which it calls a pin, and then tells you how fast it will take to crack it.
extern crate time;
//look a struct
struct Point {
  x: int,
  y: int,
}

fn main(){
  //i means integer
  let (code, five) = (111995i, 5i);
  
  let first_match = (1i,2i,3i);
  let second_match = (1i,2i,3i);
  //let's assign values from a struct
  let point = Point {x: 1i, y: 6i};
  //calling value from struct
  println!("The points are at {} and {}", point.x, point.y);
  if first_match == second_match {
    println!("A match!");
  } else {
    println!("nope");
  }

  let start = time::precise_time_s();
  crack_pin(code, 6);
  let stop = time::precise_time_s();

  print_something(code);

  let elapsed = stop - start;
  println!("Looks like it can be cracked in {} seconds ", elapsed);
 
  //returns either 10 or 5 and stores it in variable y
  let y = if five < 10 { 10i } else { 5i };
  println!("Look at this number {}", y);

  let (thing1, thing2) = next_two(5i);
  println!("thing1, thing2 = {}, {}", thing1, thing2);
}

fn crack_pin(x: int, y: int){
  let mut guess = 1i;
  while guess != x {
    guess = add_one(guess);
  }
  println!("Random number {}", y);
}

fn add_one(x: int) -> int {
  x + 1
}

fn next_two(x: int) -> (int, int) { (x + 1i, x+2i) }
fn print_something(code: int){
  if code < 1000 {
    println!("Wow, that is pretty weak");
  } else {
    println!("Weak, but not as weak");
  }
}

