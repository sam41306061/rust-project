use std::io::stdin;
fn main() {
    let mut message: String = String::new(); //create a mutable string variable (empty)
    println!("Enter your message");

    stdin().read_line(&mut message ).unwrap();
    println!("message is {}", message);
}
