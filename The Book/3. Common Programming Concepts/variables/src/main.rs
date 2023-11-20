fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6; // Reassigning value is only possible with mut
    println!("The value of x is: {x}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2; // This y is only valid in side the block.
        println!("The value of y in the inner scope is: {y}");
    }
    // This y value does not reflect what happened inside inner scope.
    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len(); // Shadowing allows change of the type.

    // This will not work
    // let mut spaces = "   ";
    // spaces = spaces.len();

    println!("Number of spaces are: {spaces}");
}
