// Function copied from StackOverflow to print typeof a variable, used later in Floating Point section.

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


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

    {   // Multiplication 
        let y = y*2;
        println!("The value of y in inner scope is: {y}");

    }

    println!("The value of y is: {y}");

    let spaces = "  ";
    let spaces = spaces.len();

    println!("Spaces: {spaces}");

    // Floating Point + Addition

    let x = 2.0;
    let y :f32 = 3.0;

    let sum: f32 = x + y;
    
    println!("Sum: {}",sum);
    print_type_of(&sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("Difference: {}",difference);

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("Quotient: {}, Truncated: {}",quotient,truncated);

    // remainder
    let remainder = 43 % 5;
    println!("Remainder: {}",remainder);

    // Boolean

    let t = true;
    let f: bool = false;
    println!("t: {}, f: {}",t,f);

    // Character Heart eyed cat brought from EmojiPedia

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat:{}",c,z,heart_eyed_cat);

    // Compund Types

    // Tuples

    // Type annotation
    let tup: (i32,f64,u8) = (50,3.6,9);

    // Tuple Destructuring

    let tup = (900,9.3,8);
    
    let (x,y,z) = tup;
    println!("x: {x}, y: {y}, z: {z}");

    let nine_hundred = tup.0;
    let nine_point_three = tup.1;
    let eight = tup.2;
    println!("nine_hundred: {nine_hundred}, nine_point_three: {nine_point_three}, eight: {eight}");

    // Unit Tuple

    let unit = ();
    println!("Unit: {:?}",unit);

    

}