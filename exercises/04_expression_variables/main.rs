////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `math!()` macro.
macro_rules! math {
    (($a:literal * $b:literal), plus, $c:expr) => {
      $a*$b+$c 
    };
    (square $d:expr) => {
      $d*$d
    };
}
////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let var = 5;
    print_result(math!((2 * 3), plus, var));
    print_result(math!(square var));
}
