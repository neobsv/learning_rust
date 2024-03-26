// Advanced Types

// Why newtypes are useful as types?
// Type aliases, a feature similar to newtypes but with slightly different semantics?

// Using the Newtype Pattern for Type Safety and Abstraction

// 1. Statically enforcing that values are never confused and indicating the units of a value
    // Using newtypes to indicate units: recall that the Millimeters and Meters structs wrapped u32 values in a newtype. 
    // If we wrote a function with a parameter of type Millimeters, we couldn’t compile a program that accidentally tried to call that function with a value of type Meters or a plain u32.
// 2. Encapsulation: We can also use the newtype pattern to abstract away some implementation details of a type, the new type can expose a public API that is different from the API of the private inner type.
    // Newtypes can also hide internal implementation. For example, we could provide a People type to wrap a HashMap<i32, String> that stores a person’s ID associated with their name. 
    // Code using People would only interact with the public API we provide, such as a method to add a name string to the People collection; that code wouldn’t need to know that we assign an i32 ID to names internally

fn main() {
    // Creating Type Synonyms with Type Aliases
    // Rust provides the ability to declare a type alias to give an existing type another name:

    type Kilometers = i32;

    // Now, the alias Kilometers is a synonym for i32; Kilometers is not a separate, new type unlike Millimeters and Meters mentioned above.

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // IMPORTANT: Because Kilometers and i32 are the same type, we can add values of both types and we can pass Kilometers values to functions that take i32 parameters.

    // The main use case for type synonyms is to reduce repetition.

    // Lets say that we have a type: Box<dyn Fn() + Send + 'static> ; we need to write lengthy types in function signatures and type annotations
    let _f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));

    fn _takes_long_type(__f: Box<dyn Fn() + Send + 'static>) {}

    // Replace this with a type alias, called 'Thunk'
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let _f1: Thunk = Box::new(|| println!("hi"));

    fn _takes_long_type1(__f: Thunk) {}

    // This is much more readable, but it helps to choose meaningful names

    // Type aliases are also commonly used with the Result<T, E> type for reducing repetition.
    // Lets consider the Write trait in the std::io library, which often uses Result types containing std::io::Error as E

    use std::fmt;
    use std::io::Error;

    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        fn flush(&mut self) -> Result<(), Error>;

        fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
    }

    // The Result<..., Error> is repeated a lot. As such, std::io has this type alias declaration:
    

    // Because this declaration is in the std::io module, we can use the fully qualified alias std::io::Result<T>; that is, 
    // a Result<T, E> with the E filled in as std::io::Error. The Write trait function signatures end up looking like this:

    {
        type Result<T> = std::result::Result<T, std::io::Error>;
        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<()>;
        
            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
        }
    }

    // This is much more readable.
    // It makes code easier to write and it gives us a consistent interface across all of std::io. 
    // It is an alias for Result<T, E> which means we can use any methods that work on Result<T, E> with it, as well as special syntax like the ? operator.


    // The Never Type that Never Returns

    // Rust has a special type named ! that’s known in type theory lingo as the empty type OR never type because it has no values.

    /*
        fn bar() -> ! {
            // This function will never return!
        }
    */

    // This code is read as “the function bar returns never.” Functions that return never are called diverging functions.
    // But what use is a type you can never create values for?
    loop {
        let guess: u32 = match "5".trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        break;
    }
    // Rust requires that guess have only one type. So what does continue return?
    // Rust computes the type of guess, it looks at both match arms, the former with a value of u32 and the latter with a ! value. 
    // Because ! can never have a value, Rust decides that the type of guess is u32.

    // IMPORTANT: type ! can be coerced into any other type.
    // The never type is useful with the panic! macro as well. Recall the unwrap function that we call on Option<T> values to produce a value or panic with this definition:

    /*
        impl<T> Option<T> {
            pub fn unwrap(self) -> T {
                // Rust sees that val has the type T and panic! has the type !, so the result of the overall match expression is T
                match self {
                    Some(val) => val,
                    None => panic!("called `Option::unwrap()` on a `None` value"),
                }
            }
        }
    */

    // One final expression that has the type ! is a loop:
    /*
        loop {
            print!("forever");
            print!("and ever ");
        }
    */

    // Dynamically Sized Types and the Sized Trait

    // Rust needs to know certain details about its types, such as how much space to allocate for a value of a particular type. 
    // Dynamically Sized Types: Sometimes referred to as DSTs or unsized types, these types let us write code using values whose size we can know only at runtime.

    // Details of a dynamically sized type called str, which we’ve been using throughout the book:

    /*  ERROR: Because, Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must use the same amount of memory.
        
        let s1: str = "Hello there!";
        let s2: str = "How's it going?";

        // s1 needs 12 bytes of storage and s2 needs 15. This is why it’s not possible to create a variable holding a dynamically sized type.
    */

    // We make the types of s1 and s2 a &str rather than a str. Recall that the slice data structure just stores the starting position and the length of the slice.
    // So although a &T is a single value that stores the memory address of where the T is located, a &str is two values: the address of the str and its length.

    /*  IMPORTANT: How are dynamically sized types possible?
    
        As such, we can know the size of a &str value at compile time: it’s twice the length of a usize.
        That is, we always know the size of a &str, no matter how long the string it refers to is.
        
        In general, this is the way in which dynamically sized types are used in Rust: they have an extra bit of metadata that stores the size of the dynamic information.
        The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.
        
        We can combine str with all kinds of pointers: for example, Box<str> or Rc<str>. 
        In fact, you’ve seen this before but with a different dynamically sized type: traits. 
        EVERY TRAIT is a DYNAMICALLY SIZED TYPE we can refer to by using the name of the trait.

        To use traits as trait objects, we must put them behind a pointer, such as &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).

    */

    // To work with DSTs, Rust provides the Sized trait to determine whether or not a type’s size is known at compile time. 
    // This trait is automatically implemented for everything whose size is known at compile time.
    // In addition, Rust implicitly adds a bound on Sized to every generic function.


    fn generic<T>(t: T) {
        ()
    }

    // is actually,
    fn _generic<T: Sized>(t: T) {
        ()
    }

    // You can use the special syntax to relax the restriction that generic functions need types with known sizes at compile time:
    fn __generic<T: ?Sized>(t: &T) { // Also note that we switched the type of the t parameter from T to &T. Because the type might not be Sized, we need to use it behind some kind of pointer.
        ()
    }

    // A trait bound on ?Sized means “T may or may not be Sized” and this notation overrides the default that generic types must have a known size at compile time.
    // NOTE: The ?Trait syntax with this meaning is only available for Sized, not any other traits.


}
