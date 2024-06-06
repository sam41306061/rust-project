use std::io::stdin;
fn main() {
    let mut message: String = String::new(); //create a mutable string variable (empty)
    println!("Enter your message");

    stdin().read_line(&mut message ).unwrap();
    println!("message is {}", message);


    // Expressions vs. Statements
	let a = 5;
	let b = a + 1;

	println!("Expression Evaluation: {}", a + 1 * b);

	// Control flow using conditional expressions
	if a == 1 {
		println!("Conditional Expression evaluated to true!");
	} else {
		println!("Conditional Expression evaluated to false!");
	}
}
