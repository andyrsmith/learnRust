extern crate time;
fn main(){
  let code = 111995i;

  let start = time::precise_time_s();
  crack_pin(code, 6);
  let stop = time::precise_time_s();

  print_something(code);

  let elapsed = stop - start;
  println!("Looks like it can be cracked in {} seconds ", elapsed);
  
  let y = if code < 10000 { 10i } else { 5i };
  println!("Look at this number {}", y);
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

fn print_something(code: int){
  if code < 1000 {
    println!("Wow, that is pretty weak");
  } else {
    println!("Weak, but not as weak");
  }
}

