use tagup_scanner::scan;

fn main() {
  let tokens = scan("
  Hello, world! I am { age } years old.

  { #for test }
    {#if test == 1}
    Hello There
    { /if }
  { /for }
  ");

  println!("{:?}", tokens);
}