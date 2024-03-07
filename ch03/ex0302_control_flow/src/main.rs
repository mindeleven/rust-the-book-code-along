fn main() {
    // most common constructs for controling the flow of execution of Rust code 
    // are if expressions and loops
    println!("------------------------------------------------------------");
    println!("if Expressions");
    println!("------------------------------------------------------------");
    // an if expression allows you to branch your code depending on conditions
    // the condition must be a bool
    // Rust will not try to convert non-boolean types to a boolean
    // multiple conditions can be handeled by combining if and else in an else if expression
    let number = 666;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("------------------------------------------------------------");
    println!("Using if in a let Statement");
    println!("------------------------------------------------------------");
    // the potential result of each arm must be the same type
    let condition = true;
    let number_two = if condition {
        27
    } else {
        42
    };
    println!("number two turned out to be {}", number_two);

    println!("------------------------------------------------------------");
    println!("Repetition with Loops");
    println!("------------------------------------------------------------");
    // loops will run through the code inside the loop body to the end 
    // and then start immediately back at the beginning
    let mut counter = 0;
    loop {
        if counter == 0 {
            println!("LOOP 01 executing....")
        }
        println!("looping at conter {}", counter);
        if counter == 6 {
            println!("time to go....");
            break;
        }
        counter += 1;
    }

    println!("------------------------------------------------------------");
    println!("Returning Values from Loops");
    println!("------------------------------------------------------------");
    // passing the result out of an operation that got checked inside the loop
    let mut counter = 0;
    // declare a variable result to hold the value returned from the loop
    let result = loop {
        if counter == 0 {
            println!("LOOP 02 executing....")
        }
        println!("looping at conter {}", counter);
        if counter == 4 {
            break counter * counter;
        };
        counter += 1;
    };
    println!("The number we've been looking for is {}", result);

    println!("------------------------------------------------------------");
    println!("Loop Labels to Disambiguate Between Multiple Loops");
    println!("------------------------------------------------------------");
    // usually break and continue apply to the innermost loop at that point
    // alternatively you could specify a loop label on a loop 
    // it can be used to apply break or continue the labeled loop
    let mut counter = 0;
    'counting_loop: loop {
        println!("counter inside outer loop: {}", counter);
        let mut inner_counter = 10;

        loop {
            println!("inner counter inside inner loop: {}", inner_counter);

            if inner_counter == 0 {
                break;
            }

            if counter == 2 {
                break 'counting_loop;
            }

            inner_counter -= 1;
        }


        counter += 1;
    }
    
    println!("------------------------------------------------------------");
    println!("Conditional Loops with while");
    println!("------------------------------------------------------------");
    // evaluating a condition within a loop
    // while the condition is true the loop runs
    // when the condition ceases to be true the program calls break
    let mut while_number = 3;
    while while_number > 0 {
        println!("the number from the first while loop is {}", while_number);
        if while_number == 0 {
            break;
        }
        while_number -= 1;
    }
    println!("afthermath of first while loop (we're at {})", while_number);

}
