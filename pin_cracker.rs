extern crate time;
fn main(){
  let code: int = 583891281;
  let mut guess: int = 1;

  let start = time::precise_time_s();
  while guess != code {
    guess += 1;
  }
  let stop = time::precise_time_s();

  let elapsed = stop - start;
  println!("Looks like it can be cracked in {} seconds ", elapsed);

}
