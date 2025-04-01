fn main(){
    let number =3;
    if number > 3{
        println!("The number is greater than 3");
    } else if number == 3 {
        println!("The number is equal to 3");
    } else {
        println!("The number is less than 3");
    }

    let condition=true;
    let number = if condition {5} else {6};
    println!("The number is {}", number);
    // beacuse if is an expression, it can be used to assign a value to a variable
    
    // endless loop
    // loop{
    //     println!("Looping...");
    // }

    let mut counter =0;

    let result = loop {
        counter +=1;
        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // loop with lables
    let mut count=0;
    'counting_up: loop{
        println!("count = {}", count);
        let mut remaining =10;
        loop{
            if remaining==9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            println!("remaining = {}", remaining);
            remaining -=1;

        }
        count +=1;

    }
    println!("End count = {}", count);
    

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!");


    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;
    while index<5{
        println!("The value is {}", arr[index]);
        index +=1;
    }

    for number in arr{
        println!("The value is {}", number);
    }

    for number in(1..5).rev(){
        println!("The value is {}", number);
    }
}