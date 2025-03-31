fn main(){
    // let guess = "42".parse().expect("Not a number!"); 
    // the above line will cause a compilation error because the type of `guess` is not specified
    // let guess: u32 = "42".parse().expect("Not a number!");
    // the above line will work because we specified the type of `guess` as `u32`

    // SCALAR TYPES
    //  single value
    // integers, floating-point numbers, booleans, characters
    // integers
    // let x: i32 = 5; // 32-bit signed integer
    // let y: u32 = 5; // 32-bit unsigned integer
    // let z: i64 = 5; // 64-bit signed integer
    // let a: u64 = 5; // 64-bit unsigned integer
    // let b: i8 = 5; // 8-bit signed integer
    // let c: u8 = 5; // 8-bit unsigned integer
    // let d: i16 = 5; // 16-bit signed integer
    // let e: u16 = 5; // 16-bit unsigned integer
    // let f: isize = 5; // pointer-sized signed integer
    // let g: usize = 5; // pointer-sized unsigned integer
    // // floating-point numbers
    // let h: f32 = 5.0; // 32-bit floating-point number
    // let i: f64 = 5.0; // 64-bit floating-point number
    // // booleans
    // let j: bool = true; // boolean value
    // let k: bool = false; // boolean value
    // // characters
    // let l: char = 'a'; // character value
    // let m: char = 'A'; // character value
    // let n: char = '1'; // character value
    // let o: char = '!'; // character value
    // let p: char = ' '; // character value
    // let q: char = '\n'; // newline character
    // let r: char = '\t'; // tab character
    // let s: char = '\u{1F600}'; // unicode character
    // let t: char = '\u{1F600}'; // unicode character


    // integer overflow causes 2s complement that is wrapping
    // let u: u8 = 256; // this will cause a compilation error because `u8` can only hold values from 0 to 255

    // COMPOUND TYPES

    // tuple
    let tup: (i32, f64, char) = (5, 6.7, 'a'); // tuple with 3 elements
    let (x, _y, _z) = tup; // destructuring the tuple
    println!("The value of x is: {}", x); // This will print 5
    println!("The value of y is: {}", tup.1); // This will print 6.7 after '.' is the index


    // array
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // array with 5 elements
    let arr2: [i32; 5] = [1; 5]; // array with 5 elements, all initialized to 1
    println!("The value of arr[0] is: {}", arr[0]); // This will print 1
    


}