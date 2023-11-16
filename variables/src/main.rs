fn main() {

    // Variables & Mutability

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constants

    const THREE_HOURS_IN_SECONDS :u32 = 60 * 60 * 3;

    println!("3 hours in seconds: {THREE_HOURS_IN_SECONDS}");

    // Shadowing

    let y = 5;

    let y=y+1;

    {
        let y = y*2;
        println!("The value of y in inner scope is: {y}");

    }

    println!("The value of y is: {y}");

    let spaces = "  ";
    let spaces = spaces.len();

    println!("Spaces: {spaces}")
}