// Advanced Features

// Unsafe Rust: how to opt out of some of Rust’s guarantees and take responsibility for manually upholding those guarantees

// Although the code might be okay, if the Rust compiler doesn’t have enough information to be confident, it will reject the code.
// If Rust didn’t let you do unsafe operations, you couldn’t do certain tasks. Rust needs to allow you to do low-level systems programming, such as directly interacting with the operating system.

// Unsafe Superpowers
    // 1. Dereference a raw pointer
    // 2. Call an unsafe function or method
    // 3. Access or modify a mutable static variable
    // 4. Implement an unsafe trait
    // 5. Access fields of unions

// It’s important to understand that unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked.
// In addition, unsafe does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety problems: the intent is that as the programmer, 
// you’ll ensure the code inside an unsafe block will access memory in a valid way.

// Keep unsafe blocks small

// it’s best to enclose unsafe code within a safe abstraction and provide a safe API, which we’ll discuss later in the chapter when we examine unsafe functions and methods. 
// Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited.

use std::slice;

fn main() {
    // 1. Dereferencing a Raw Pointer
    // Unsafe Rust has two new types called raw pointers that are similar to references.
    // Raw pointers can be immutable or mutable and are written as *const T and *mut T
    // In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.

    // Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    // Aren’t guaranteed to point to valid memory
    // Are allowed to be null
    // Don’t implement any automatic cleanup

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // We’ve created raw pointers by using as to cast an immutable and a mutable reference into their corresponding raw pointer types
    // We know these particular raw pointers are valid, but we can’t make that assumption about just any raw pointer

    // Trying to use arbitrary memory is undefined: there might be data at that address or there might not:
    let address = 0x012345usize;
    let _r = address as *const i32;

    // We can create raw pointers in safe code, but we can’t dereference raw pointers and read the data being pointed to

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // NOTE: OWNERSHIP RULES WERE BROKEN
    // We created *const i32 and *mut i32 raw pointers that both pointed to the same memory location, where num is stored. 
    // If we instead tried to create an immutable and a mutable reference to num, the code would not have compiled because 
    // Rust’s ownership rules don’t allow a mutable reference at the same time as any immutable references.
    // If you change data through the mutable pointer you could potentially create a data race!

    // 2. Calling an Unsafe Function or Method

    // The unsafe keyword in this context indicates the function has requirements we need to uphold when we call this function, because Rust can’t guarantee we’ve met these requirements
    
    unsafe fn dangerous() {
        // Bodies of unsafe functions are effectively unsafe blocks, so to perform other unsafe operations within an unsafe function, we don’t need to add another unsafe block.
    }

    unsafe {
        dangerous();
    }

    // Creating a Safe Abstraction over Unsafe Code

    // Just because a function contains unsafe code doesn’t mean we need to mark the entire function as unsafe. 
    // As an example, let’s study the split_at_mut function from the standard library, which requires some unsafe code.

    // BORROWING RULE BREAK:We return two mutable slices in a tuple: one from the start of the original slice to the mid index and another from mid to the end of the slice.
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
    
        assert!(mid <= len);
    
        // Two mutable refs of the same variable in the same scope, not allowed!
        // Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice.
        
        // ERROR: (&mut values[..mid], &mut values[mid..])

        let ptr = values.as_mut_ptr();

        unsafe {
            (
                // The unsafe code: the slice::from_raw_parts_mut function takes a raw pointer and a length, and it creates a slice.
                slice::from_raw_parts_mut(ptr, mid),
                // The ptr.add() function is also unsafe
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }

    }

    // The resulting split_at_mut function as unsafe, and we can call this function from safe Rust. We’ve created a safe abstraction to the unsafe code with an implementation of the function that uses unsafe code in a safe way


    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    /* DANGEROUS: We don't know this memory location, and we don't know if this slice contians valid i32 values
        
        let address = 0x01234usize;
        let r = address as *mut i32;

        let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

        Undefined behavior.
    
    */

    // Using extern to Call External Code

    // FFI: Foreign Function Interface : define functions and enable a different programming language to call those functions

    // Set up an integration with the abs function from the C standard library. Functions declared within extern blocks are always unsafe to call from Rust code
    // Within the extern "C" block, we list the names and signatures of external functions from another language we want to call. 
    // The "C" part defines which application binary interface (ABI) the external function uses and the ABI defines how to call the function at the assembly level.
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 3. Access of Modify a Mutable Static Variable

    // we’ve not yet talked about global variables, which Rust does support but can be problematic with Rust’s ownership rules. 
    // If two threads are accessing the same mutable global variable, it can cause a data race. Global variables are called static variables.

    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);

    // Static variables are similar to constants. Written in SCREAMING_SNAKE_CASE. Static variables can only store references with the 'static lifetime, compiler can figure out its lifetime and we aren’t required to annotate it explicitly.
    // Accessing an immutable static variable is safe. 
    // A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory. Using the value will always access the same data.
    // Constants, on the other hand, are allowed to duplicate their data whenever they’re used
    // Static variables can be mutable. Accessing and modifying mutable static variables is unsafe!

    // As with regular variables, we specify mutability using the mut keyword
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            // Having multiple threads access COUNTER would likely result in data races.
            COUNTER += inc;
        }
    }

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // It’s difficult to ensure there are no data races, which is why Rust considers mutable static variables to be unsafe!
    // Where possible, it’s preferable to use the concurrency techniques and thread-safe smart pointers.


    // 4. Implementing an Unsafe Trait
    // A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.

    unsafe trait Foo {
        // methods go here
    }
    
    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // As an example, recall the Sync and Send marker traits
    // The compiler implements these traits automatically if our types are composed entirely of Send and Sync types. 
    // If we implement a type that contains a type that is not Send or Sync, such as raw pointers, and we want to mark that type as Send or Sync, we must use unsafe!
    // Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads
    // Therefore we need to verify these checks manually and mark the Trait as unsafe

    // 5. Accessing the Fields of a Union
    // A union is similar to a struct, but only one declared field is used in a particular instance at one time.
    // Unions are primarily used to interface with unions in C code. Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.


}


// To call Rust code from a C function, we specify the no_mangle macro, which means the compiler will preserve the function name
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
// This use of extern is not unsafe!