// Patterns and Matching

// Patterns are a special syntax in Rust for matching against the structure of types, both complex and simple. 
// Using patterns in conjunction with match expressions and other constructs gives you more control over a program’s control flow.

// Patterns consist of some combination of the following:
    // Literals
    // Destructured arrays, enums, structs, tuples
    // Variables
    // Wildcards
    // Placeholders

// Some example patterns include x, (a, 3), and Some(Color::Red). In the contexts in which patterns are valid, these components describe the shape of data. 
// Our program then matches values against the patterns to determine whether it has the correct shape of data to continue running a particular piece of code.

// If the pattern matches the value, we use the value parts in our code. Recall the match expressions in Chapter 6 that used patterns, such as the coin-sorting machine example. 
// If the value fits the shape of the pattern, we can use the named pieces. If it doesn’t, the code associated with the pattern won’t run.

// All the places Patterns can be used

fn main() {
    // 1. match Arms

    // We use patterns in the arms of match expressions. Formally, match expressions are defined as the keyword match, a value to match on, 
    // and one or more match arms that consist of a pattern and an expression to run if the value matches that arm’s pattern.

    /*

    match VALUE {
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
        PATTERN => EXPRESSION,
    }

    */

    // One requirement for match expressions is that they need to be exhaustive in the sense that all possibilities for the value in the match expression must be accounted for
    // You can use a catchall: '_' for the last arm just to be sure any unexpected cases are covered, and this means "ignore any values not specified".
    let x: Option<i32> = Some(8);
    let _res = match x {
        Some(i) => Some(i+1),
        _ => None
    };

    // 2. Conditional if let Expressions

    // if let expressions are a shorter way to write the equivalent of a match that only matches one case. 
    // Optionally, if let can have a corresponding else containing code to run if the pattern in the if let doesn’t match.

    // it’s also possible to mix and match if let, else if, and else if let expressions. Doing so gives us more flexibility than a match expression 
    // in which we can express only one value to compare with the patterns. A series of if let, else if, else if let arms don't need to relate to each other.

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    
    // Shadowed Variables: You can see that if let can also introduce shadowed variables in the same way that match arms can: the line if let Ok(age) = age introduces a new shadowed age variable 
    // that contains the value inside the Ok variant. This means we need to place the if age > 30 condition within that block: we can’t combine these two conditions into if let Ok(age) = age && age > 30. The shadowed age we want to compare to 30 isn’t valid until the new scope starts.
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // This conditional structure lets us support complex requirements. With the hardcoded values we have here, this example will print Using purple as the background color.
    // The downside of using if let expressions is that the compiler doesn’t check for exhaustiveness, unlike in match.

    // 3. while let Conditional Loops

    // Similar in construction to if let, the while let conditional loop allows a while loop to run for as long as a pattern continues to match.

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // If the vector is empty, pop returns None. The while loop continues running the code in its block as long as pop returns Some

    // 4. for Loops

    // In a for loop, the value that directly follows the keyword for is a pattern. For example, in for x in y the x is the pattern.
    // We can use for the pattern in the for loop to destructure or break apart tuples (similar to python).

    let v = vec!['a', 'b', 'c'];

    // (0, 'a') is matched to the pattern (index, value)
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }


    // 5. let Statements

    // Every time you've used a let statement like this you've been using patterns, although you might not have realized it
    // let PATTERN = EXPRESSION;

    // In let x = 5; Because the name x is the whole pattern, this pattern effectively means “bind everything to the variable x, whatever the value is.”
    // let to destructure a tuple: let (x, y, z) = (1, 2, 3);
    // ERROR: let (x, y) = (1, 2, 3); We could ignore one or more of the values in the tuple using _ or ..

    // 6. Function Parameters
    fn foo(x: i32) { }
    // The x part is a pattern! As we did with let, we could match a tuple in a function’s arguments to the pattern.

    // This code prints Current location: (3, 5). The values &(3, 5) match the pattern &(x, y), so x is the value 3 and y is the value 5.
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);

    // We can also use patterns in closure parameter lists in the same way as in function parameter lists, because closures are similar to functions.


    // Refutability: Whether a Pattern Might Fail to Match

    // Patterns come in two forms: refutable and irrefutable. 
    // Patterns that will match for any possible value passed are irrefutable. An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match. 
    // Patterns that can fail to match for some possible value are refutable. An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.

    // Function parameters, let statements, and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match.
    // The if let and while let expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns, because a conditional needs the pattern to be false sometimes, can't have an always true pattern.

    // Attempting to use a refutable pattern with let: let Some(x) = some_option_value; None value, it would fail to match the pattern Some(x)
    // At compile time, Rust will complain that we’ve tried to use a refutable pattern where an irrefutable pattern is required

    // If we have a refutable pattern where an irrefutable pattern is needed, we can fix it by changing the code that uses the pattern: instead of using let, we can use if let.

    let some_option_value: Option<i32> = None;

    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // However, If we give if let a pattern that will always match, the compiler will give a warning.
    if let x = 5 { // always true, irrefutable so there is no need for an if
        println!("{}", x);
    };

    // For this reason, match arms must use refutable patterns, except for the last arm, which should match any remaining values with an irrefutable pattern. 

    // Pattern Syntax

    // We gather all the syntax valid in patterns and discuss why and when you might want to use each one:

    // 1. Matching Literals
    // match patterns against literals directly:
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything")
    }

    // 2. Matching Named Variables
    // Named variables are irrefutable patterns that match any value. Because match starts a new scope, variables declared as part of a pattern inside the 
    // match expression will shadow those with the same name outside the match construct, as is the case with all variables. We declare a variable named x with 
    // the value Some(5) and a variable y with the value 10. We then create a match expression on the value x

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // SURPRISE: this arm matches x!!!
        _ => println!("Default case, x = {:?}", x)
    }
    println!("at the end: x = {:?}, y = {y}", x);

    // IMPORTANT:
    // The pattern in the second match arm introduces a new variable named y that will match any value inside a Some value. Because we’re in a new scope inside the 
    // match expression, this is a new y variable, not the y we declared at the beginning with the value 10.
    // If x had been a None value instead of Some(5), the patterns in the first two arms wouldn’t have matched, so the value would have matched to the underscore.

    // To create a match expression that compares the values of the outer x and y, rather than introducing a shadowed variable, we would need to use a MATCH GUARD conditional instead!

    // 3. Multiple Patterns
    // In match expressions, you can match multiple patterns using the | syntax, which is the pattern OR operator.
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything")
    }
    // prints "one or two"

    // 4. Matching Ranges of Values with ..=
    // The ..= syntax allows us to match to an inclusive range of values, any of the values in the range will execute the arm
    // IMPORTANT: The compiler checks that the range isn’t empty at compile time, and because the only types for which Rust can tell if a range is empty or not are char and numeric values, 
    // ranges are only allowed with numeric or char values.
    
    // numeric range:
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else")
    }

    // char range:
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else")
    }

    // Destructing to Break Apart Values

    // patterns to destructure structs, enums, and tuples

    // 5. Destructuring Structs
    // This code creates the variables a and b that match the values of the x and y fields of the p struct.
    
    struct Point { x: i32, y: i32 }
    
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // This example shows that the names of the variables a and b in the pattern don’t have to match the field names of the struct. 
    
    // Rust has a shorthand for patterns that match struct fields: you only need to list the name of the struct field, and the variables created from the pattern will have the same names
    let Point { x, y } = p;

    // We can also destructure with literal values as part of the struct pattern rather than creating variables for all the fields. Doing so allows us to test some of the fields for particular values while creating variables to destructure the other fields.
    // We have a match expression that separates Point values into three cases: points that lie directly on the x axis (which is true when y = 0), on the y axis (x = 0), or neither.

    let p = Point { x: 0, y: 7 };

    match p {
        // The first arm will match any point that lies on the x axis by specifying that the y field matches if its value matches the literal 0
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        // The second arm matches any point on the y axis by specifying that the x field matches if its value is 0
        Point { x: 0, y } => println!("On the y axis at {y}"),
        // The third arm doesn’t specify any literals, so it matches any other Point
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // IMPORTANT: match is not switch!! Once a case matches, it breaks automatically
    // So even though Point { x: 0, y: 0} is on the x axis and the y axis, this code would only print "On the x axis at 0".

    // 6. Destructuring Enums
    // We use the Message enum and write a match with patterns that will destructure each inner value

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    // This code will print Change the color to red 0, green 160, and blue 255
    
    // For enum variants without any data, like Message::Quit, we can’t destructure the value any further.
    
    // For struct-like enum variants, such as Message::Move, we can use a pattern similar to the pattern we specify to match structs.
    // After the variant name, we place curly brackets and then list the fields with variables so we break apart the pieces to use in the code for this arm

    // For tuple-like enum variants, like and Message::ChangeColor that holds a tuple with three elements, the pattern is similar to the pattern we specify to match tuples. 
    // The number of variables in the pattern must match the number of elements in the variant we’re matching.


    // 7. Destructuring Nested Structs and Enums

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum MessageII {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = MessageII::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        // Matches a Message::ChangeColor enum variant that contains a Color::Rgb variant; then the pattern binds to the three inner i32 values.
        MessageII::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        // Matches a Message::ChangeColor enum variant, but the inner enum matches Color::Hsv instead
        MessageII::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => ()
    }
    // These complex matches can be specified because patterns destructure enums and match them.


    // 8. Destructuring Structs and Tuples
    // Complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values:

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {feet} inches: {inches} pointx: {x} pointy: {y}");


    // Ignoring Values in a Pattern

    // Like the last arm of a match, to get a catchall that doesn’t actually do anything but does account for all remaining possible values

    // 1. Ignoring an entire Value with _
    // We can also use it in any pattern, including function parameters, match arms, let statements:
    fn foo1(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo1(3, 4);
    // Useful for implementing a trait when you need a certain type signature but the function body in your implementation doesn’t need one of the parameters


    // 2. Ignoring Parts of a Value with a Nested _
    // Use _ inside another pattern to ignore just part of a value, for example, when we want to test for only part of a value but have no use for the other parts

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {

        // Test for the case when setting_value and new_setting_value are the Some variant. In that case, we print the reason for not changing setting_value.
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }

        // The business requirements are that the user should not be allowed to overwrite setting_value, but give it a new value if it was not set.
        // If either setting_value or new_setting_value are None, expressed by the _ pattern, we want to allow new_setting_value to become setting_value.
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    // To ignore parts of a tuple, like this:
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // 3. Ignore Unused Variable by Starting its Name with _
    // I have used this a lot, while writing this codebase, without this the compiler will issue a warning.
    // Note that there is a subtle difference between using only _ and using a name that starts with an underscore. 
    // The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all.

    // Starts with _s but it still binds to a value:
    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    // println!("{:?}", s); ERROR: because s is moved into _s, the pattern still binds to the value s

    // Using ONLY _ , does not bind to a value:
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    // In this case s remains unchanged, it reads "Hello!"
    println!("{:?}", s);

    // 4. Ignoring Remaining Parts of a Value with ..
    // With values that have many parts, we can use the .. syntax to use specific parts and ignore the rest, avoiding the need to list underscores for each ignored value.

    // Example 1:
    struct PointII {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = PointII { x: 0, y: 0, z: 0 };
    match origin {
        PointII { x, .. } => println!("x is {}", x),
    }

    // Example 2:
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // However, using .. must be unambiguous. If it is unclear which values are intended for matching and which should be ignored, Rust will give us an error:
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        // Here we don't know which number "second" should refer to so this is an ERROR!
        // (.., second, ..) => {
        //     println!("Some numbers: {}", second)
        // },
        _ => { println!("none"); }
    }

    // Extra Conditionals with Match Guards
    // A match guard is an ADDITIONAL IF condition, specified after the pattern in a match arm, that must also match for that arm to be chosen
    let num = Some(4);
    match num {
        // The num 4 is even, so it will match the if condition and also the Some(x) so we can say it is an even number.
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        // Otherwise, it will match this second case without a match guard, and it can be said that it is odd.
        Some(x) => println!("The number {} is odd", x),
        // If neither odd nor even, then the Optional must be None, so do nothing.
        None => ()
    }
    // IMPORTANT: The downside of this additional expressiveness is that the compiler doesn't try to check for exhaustiveness when match guard expressions are involved.

    // There is no way to express the if x % 2 == 0 condition within a pattern, so the match guard gives us the ability to express this logic. 

    // We mentioned that we could use match guards to solve our pattern-shadowing problem. Recall that we created a new variable inside the pattern in the match expression instead 
    // of using the variable outside the match. That new variable meant we couldn’t test against the value of the outer variable.
    let x = Some(5);
    let y = 10;

    match x {

        Some(50) => println!("Got 50"),

        // The pattern in the second match arm doesn’t introduce a new variable y that would shadow the outer y, meaning we can use the outer y in the match guard. 
        // Instead of specifying the pattern as Some(y), which would have shadowed the outer y, we specify Some(n). This creates a new variable n that doesn’t shadow anything because there is no n variable outside the match.
        // The match guard if n == y is not a pattern and therefore doesn’t introduce new variables. This y is the outer y rather than a new shadowed y!
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x)
    }

    // This code will now print Default case, x = Some(5). 

    println!("at the end: x = {:?}, y = {y}", x);

    // You can also use the or operator | in a match guard to specify multiple patterns; the match guard condition will apply to all the patterns.
    // The important part of this example is that the if y match guard applies to 4, 5, and 6, even though it might look like if y only applies to 6.
    let x = 4;
    let y = false;
    match x {
        // x is 4, but the match guard if y is false, so the first arm is not chosen
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no")
    }
    // The match condition states that the arm only matches if the value of x is equal to (4 OR 5 OR 6) AND (y is true).

    // @ Bindings

    // The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match.

    // We want to test that a Message::Hello id field is within the range 3..=7. We also want to bind the value to the variable id_variable so we can use it in the code associated with the arm.

    enum MessageIII {
        Hello { id: i32 },
    }
    let msg = MessageIII::Hello { id: 5 };
    match msg {

        MessageIII::Hello {
            // By specifying id_variable @ before the range 3..=7, we’re capturing whatever value matched the range
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        
        // We only have a range specified in the pattern, the code associated with the arm doesn’t have a variable that contains the actual value of the id field. 
        // The id field’s value could have been 10, 11, or 12, but the code that goes with that pattern doesn’t know which it is.
        // The pattern code isn’t able to use the value from the id field, because we haven’t saved the id value.
        MessageIII::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }

        // We’ve specified a variable without a range, we do have the value available to use in the arm’s code in a variable named id. 
        // The reason is that we’ve used the struct field shorthand syntax. But we haven’t applied any test to the value in the id field in this arm and any value would match id.
        MessageIII::Hello { id } => println!("Found some other id: {}", id)
    }



}
