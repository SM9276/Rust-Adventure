fn main() {
    //Variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}"); 
    
    //Shawdowing
    let y = 6;

    let y = y + 1;

    {
        let y = y *2;
        println!("The value of y in the inner scope is: {y}");
    }
    
    println!("The value of y is: {y}");
    
    let spaces = "   ";
    let spaces = spaces.len();
    //Reassigning the name instead of reassigning the variable 
}
