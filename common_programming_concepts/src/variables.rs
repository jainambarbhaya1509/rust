fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 20;
    println!("The value of x is: {}", x);

    // This will cause a compilation error because `x` is immutable
    // Uncommenting the line below will result in an error
    // x = 10; // Error: cannot assign twice to immutable variable `x`
    // To fix this, we can declare `x` as mutable


    const PI: f64 = 3.14159;
    println!("The value of PI is: {}", PI);
    // This will cause a compilation error because `PI` is a constant


    // shadowing
    let x = 5;
    let x = x + 1; // This shadows the previous `x`
    println!("The value of x is: {}", x); // This will print 6
    {
        let x = x*2; // This shadows the previous `x`
        println!("The value of x is: {}", x); // This will print 10
    }


    let spaces = "   ";
    let spaces = spaces.len();
    println!("The length of spaces is: {}", spaces); // This will print 3
    // shadowing allows us to reuse the same variable name for different types
    // "mut" is not allowed here
    // but it can also lead to confusion if not used carefully


}
