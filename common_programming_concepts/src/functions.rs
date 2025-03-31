fn main(){

    println!("Inside main function");
    another_function();
    another_function2(5);
    let x = another_function3(5, 10);
    println!("The value of x is: {}", x);
}

fn another_function(){
    println!("Another function");
}
fn another_function2(x: u32){
    println!("Another function of {}", x);
}
fn another_function3(x: u32, y: u32) -> u32{
    x + y
}