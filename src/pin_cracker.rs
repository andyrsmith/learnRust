///Takes a random number which it calls a pin, and then tells you how fast it will take to crack it.
extern crate time;

//look a struct
struct Point {
  x: int,
  y: int,
}

enum OptionalInt {
  Value(int),
  Missing,
}

fn main(){
  //i means integer
  let (code, five) = (111995i, 5i);
  
  mult_value();

  a_struct();

  let start = time::precise_time_s();
  crack_pin(code, 6);
  let stop = time::precise_time_s();

  print_something(code);

  let elapsed = stop - start;
  println!("Looks like it can be cracked in {} seconds ", elapsed);

  if_else_value(five); 

  just_a_tuple();

  enum_fun();

  match_maker();

  for_loop();

  fun_with_strings();

  this_vector();
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

fn mult_value() {
  let first_match = (1i,2i,3i);
  let second_match = (1i,2i,3i);

  if first_match == second_match {
    println!("A match!");
  } else {
    println!("nope");
  }
}

fn a_struct() {
  //let's assign values from a struct
  let point = Point {x: 1i, y: 6i};
  //calling value from struct
  println!("The points are at {} and {}", point.x, point.y);
}

fn if_else_value(five: int) {
  //returns either 10 or 5 and stores it in variable y
  let y = if five < 10 { 10i } else { 5i };
  println!("Look at this number {}", y);
}

fn enum_fun() {
  let real_value = Value(5);
  let fake_value = Missing;

  match real_value {
    Value(n) => println!("x is {:d}", n),
    Missing => println!("x is missing"),
  }
  match fake_value {
    Value(n) => println!("x ix {:d}", n),
    Missing => println!("x is missing"),
  }
} 

fn just_a_tuple(){
  let (thing1, thing2) = next_two(5i);
  println!("thing1, thing2 = {}, {}", thing1, thing2);
}

fn match_maker(){
  let matchy = 15i;

  match matchy {
    1 => println!("1"),
    2 => println!("2"),
    3 => println!("3"),
    4 => println!("4"),
    5 => println!("5"),
   _ => println!("no match"),
  }
}

fn for_loop(){

  for x in range(0i, 10i) {
    println!("{:d}", x);
  }
}

fn fun_with_strings() {
  let mut s = "Hello".to_string();
  let s1 = "Hello".to_string();

  println!("{}", s);

  s.push_str(" world.");
  println!("{}", s);

  takes_slice(s1.as_slice());
}

fn takes_slice(slice: &str) {
  println!("Got: {}", slice);
}

fn this_vector() {
  let mut nums = vec![1i, 2i, 7i];
  nums.push(4i);
  let slice = nums.as_slice();
  println!("Here is the slice {}", slice);
  for i in nums.iter() {
    println!("{}", i);
  }
}
