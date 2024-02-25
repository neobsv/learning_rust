fn main() {
    let num = 3;

    // basic if condition, condition must be a bool!
    if num < 5 {
        println!("less than 5!");
    } else {
        println!("more than 5!");
    }

    // if else if ladder or multiple conditions
    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if can be used on the right side of a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // rust has three kinds of loops: loop, while, for

    // loop is an infinite loop, which supports break and continue

    /*
    loop {
        println!("next");
        break;
    }
    */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // you can add a return value to the break statement
            // this will be set to result, in this case
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loops can have labels, to make it clear when multiple loops
    // are present. appropriately break statements must specify loop labels.

    let mut count = 0;

    'counting_up: loop {
        println!("count: {count}");

        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // you can break a specific loop using its label
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    // while: conditional looping
    let mut number = 3;

    while number != 0 {
        println!("while: {number}!");
        number -= 1;
    }

    let a = [10, 5, 324, 1245, 22];
    let mut index = 0;

    while index < 5 {
        println!("Value of a[index]: {}", a[index]);
        index += 1;
    }

    // for: looping through a collection variable
    // increased safety, because now we can change the length of a and this
    // would still work and it is terse. Also avoids indexing errors.
    for elem in a {
        println!("Value of a[index]: {}", elem);
    }

    // using a range in a for loop, range is specified by (start..end), where
    // the range is mathematically [start, end)

    for n in (1..4).rev() {
        println!("value of n: {}!", n);
    }
}
