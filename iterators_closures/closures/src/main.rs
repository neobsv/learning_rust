// Closures

// These are functions that capture the environment around them as well, meaning any variables declared
// above them in the same scope. It is a function like construct which you can store in a variable.

// Create the closure in one place and then call the closure elsewhere to evaluate it in a different context

use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    /*
    In the giveaway() method, we get the user preference as a parameter of type Option<ShirtColor> and call the unwrap_or_else() method on user_preference. 
    The unwrap_or_else() method on Option<T> is defined by the standard library. It takes one argument: a closure without any arguments that returns a value T (the same type stored in the Some variant of the Option<T>, in this case ShirtColor). 
        => If the Option<T> is the Some variant, unwrap_or_else() returns the value from within the Some. 
        => If the Option<T> is the None variant, unwrap_or_else() calls the closure and returns the value returned by the closure.
    We specify the closure expression || self.most_stocked() as the argument to unwrap_or_else(). This is a closure that takes no parameters itself (if the closure had parameters, they would appear between the two vertical bars). The body of the closure calls self.most_stocked().
    
    IMPORTANT: The closure captures an immutable reference to the self Inventory instance and passes it with the code we specify to the unwrap_or_else() method. Functions, on the other hand, are not able to capture their environment in this way.
    */


    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {

    /*
    The store defined in main has two blue shirts and one red shirt remaining to distribute for this limited-edition promotion.
    We call the giveaway method for a user with a preference for a red shirt and a user without any preference.
    */


    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    main1();

}

// Closure Type Inference and Annotation

fn main1() {

    // Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do
    // Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler can infer the types of the parameters and the return type.
    // Type annotations can be added to closures to increase clarity, and they can be stored in variables and passed around as well.

    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // println!("expensive closure: {}", _expensive_closure(5));

    // This illustrates how closure syntax is similar to function syntax except for the use of pipes and the amount of syntax that is optional:
    fn  _add_one_v1   (x: u32) -> u32 { x + 1 } // function
    let _add_one_v2 = |x: u32| -> u32 { x + 1 }; // closure fully annotated
    let _add_one_v3 = |x: u32| { x + 1 }; // closure partially annotated
    let _add_one_v4 = |x: u32|  x + 1  ; // closure without braces

    // For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value

    let example_closure = |x| x;

    let s = example_closure(String::from("hello")); // the compiler now infers the type of x in example_closure as a String.
    // let n = example_closure(5); // ERROR: expecting type String!
    println!("example: {}", s);

    // Capturing References or Moving Ownership

    // Closures can capture values from their environment in three ways:
        // 1. borrowing immutably
        // 2. borrowing mutably
        // 3. taking ownership


    // The variable list is an immutable reference, and it can be accessed before the closure, before calling the closure, after calling the closure.
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // A variable can bind to a closure definition, and we can later call the closure by using the variable name and parentheses as if the variable name were a function name
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    
    // We change the closure body so that it adds an element to the list vector. The closure now captures a mutable reference:
    let mut list2 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list2);
    let mut borrows_mutably = || list2.push(7);

    // NOTE: Removed the println! statement from this part, can't call it here between the point where the list2 is mutably borrowed and assigned to the variable, and when the closure borrows_mutably() is called.
    // println!("calling a mut list: {:?}", list2); ERROR

    borrows_mutably();
    println!("After calling closure: {:?}", list2);


    // The move keyword

    // If you want to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, 
    // you can use the move keyword before the parameter list.

    // This technique is mostly useful when passing a closure to a new thread to move the data so that it’s owned by the new thread.

    let list3 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list3);


    // We spawn a new thread, giving the thread a closure to run as an argument. The closure body prints out the list.
    /*
        VERY IMPORTANT:
        Even though the closure body still only needs an immutable reference, we need to specify that list should be moved into the closure by putting the move keyword at the beginning of the closure definition. 
        The new thread might finish before the rest of the main thread finishes, or the main thread might finish first. 
        If the main thread maintained ownership of list but ended before the new thread did and dropped list, the immutable reference in the thread would be invalid.
        Therefore, the compiler requires that list be moved into the closure given to the new thread so the reference will be valid.
    */
    thread::spawn(move || println!("From thread: {:?}", list3))
        .join()
        .unwrap();


    // A closure body can do any of the following: 
        // 1. move a captured value out of the closure
        // 2. mutate the captured value
        // 3. neither move nor mutate the value
        // 4. capture nothing from the environment to begin with

    // Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:

    /* Three types of Fn Traits for a Closure
        1. FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. 
        A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
        2. FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
        3. Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. 
        These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
    */

    #[allow(dead_code, unused_variables)]
    enum Option2<T> {
        Some(T),
        None
    }

    #[allow(dead_code, unused_variables)]
    impl<T> Option2<T> {

        // The unwrap_or_else() function has the additional generic type parameter F. 
        // The F type is the type of the parameter named f, which is the closure we provide when calling unwrap_or_else().

        // The trait bound specified on the generic type F is FnOnce() -> T, which means F must be able to be called once. 
        // Take no arguments, and return a T. And unwrap_or_else() can only call f() one time.
        pub fn unwrap_or_else<F>(self, f: F) -> T
        where
            F: FnOnce() -> T
        {
            match self {
                Option2::Some(x) => x,
                Option2::None => f(),
            }
        }
    }


    // Let’s look at the standard library method sort_by_key() defined on slices, to see how that differs from unwrap_or_else() and why sort_by_key() uses FnMut instead of FnOnce for the trait bound.

    // This function is useful when you want to sort a slice by a particular attribute of each item.
    // We have a list of Rectangle instances and we use sort_by_key to order them by their width attribute from low to high:

    #[allow(dead_code, unused_variables)]
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list4 = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];


    // The reason sort_by_key is defined to take an FnMut closure is that it calls the closure multiple times: once for each item in the slice. 
    // The closure |r| r.width doesn’t capture, mutate, or move out anything from its environment, so it meets the trait bound requirements.

    list4.sort_by_key(|r| r.width);
    println!("{:#?}", list4);

    // An example of a closure that implements just the FnOnce trait, because it moves a value out of the environment. The compiler won’t let us use this closure with sort_by_key:

    #[allow(dead_code, unused_variables, unused_mut)]
    let mut sort_operations: Vec<String> = vec![];
    #[allow(dead_code, unused_variables, unused_mut)]
    let value = String::from("by key called");

    list4.sort_by_key(|r| {
        // sort_operations.push(value); ERROR: Can't move the variable value out of the closure since it is an FnMut closure.
        r.width
    });
    println!("{:#?}", list4);

    // To count the number of times sort_by_key() is called, keeping a counter in the environment and incrementing its value in the closure body is a more straightforward way to calculate that. 
    // The closure works with sort_by_key() because it is only capturing a mutable reference to the num_sort_operations counter and can therefore be called more than once:

    let mut num_sort_operations = 0;
    list4.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list4);


}