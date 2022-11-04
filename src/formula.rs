// The keyword std::io is used in rust to allow the input and output features.
use std::io;

//This function is used to declare the variables a and b has mutable because
fn inverse(j: i32, k: i32) {
    let mut a = 0;
    let mut b = 0;
    let h = gcd_extended(j, k, &mut a, &mut b);
    //the if block runs if and only if the value of h is not equal to 1 this means that for a number's multiplicative inverse to exist it must be congruent, it's greatest common divisor must equal 1
    if h != 1 {
        println!("The inverse doesn't exist");
    } else {
        //the else code only runs when h is equal to 1 and rem is the multiplicative inverse.
        let rem = (a % k + k) % k;
        println!("The modular multiplicative inverse of the number is {}", rem);
    }
}

// the function used is being declared
fn gcd_extended(a: i32, b: i32, x: &mut i32, y: &mut i32) -> i32 {
    if a == 0 {
        // we dereference here to assign to those values in memory.
        // the following code allows to change the values of a and b
        *x = 0;
        *y = 1;
        //y is returned meaning the y is what is sent to main program whenever function is called and terminates if the value of x is 0
        return b;
    }

    // the function takes referenced mutable values, the variables a1 and b1 can be declared
    //the variable x and y also stores results of recursive call
    let mut a1 = 0;
    let mut b1 = 0;
    //bind function values to variables, also note that the gcd_extended is a recursive call meaning the function is calling itself and as gcd works in practice it will continue to happen until there's a final answer and it terminates
    let gcd = gcd_extended(b % a, a, &mut a1, &mut b1);

    // remember, we dereference with the asterisks so we can assign new values to a and b
    //a and b here use results of recursive call stored in x1 and y1
    *x = b1 - (b / a) * a1;
    *y = a1;
    //gcd is returned meaning the gcd value is what is sent to main program whenever function is called and terminates
    return gcd;
}
// pub is responsible for making the functions accessible from outside the file
pub fn run() {
    //This is the prompt to tell the user to enter a number
    println!("Enter the number:");
    //The initializes the number entered to a value
    let mut input1 = String::new();
    //the io module handles input and output and provides related functions, here the line is read and the unwrap simply collects result of computation and if there was an error or no value program execution stops
    io::stdin().read_line(&mut input1).unwrap();
    //This is the prompt to tell the user to input the modulus
    println!("Enter the modulus:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    //i32 means 32 bit signed integer type, it specifies number of integer bits the variable can accept, trim removes white space and parse is used for the type conversion from string to 32 bit signed integer type as specified
    let num: i32 = input1.trim().parse().unwrap();
    let modulus: i32 = input2.trim().parse().unwrap();
    inverse(num, modulus);
}