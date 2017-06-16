extern crate yarns;

fn main() {
    let val1 = String::from("hello");
    let val2 = String::from("goodbye");
    println!("The concat of 'hello' 'goodbye' is {}", yarns::concat(&val1, &val2));
}