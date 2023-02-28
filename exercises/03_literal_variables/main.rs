////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `math!()` macro.
macro_rules! math {
    ($metavar1:literal plus $metavar2:literal) => {
      $metavar1 + $metavar2
    };
    (square $metavar3:literal) => {
      $metavar3 * $metavar3
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(3 plus 5));
    print_result(math!(square 2));
}
