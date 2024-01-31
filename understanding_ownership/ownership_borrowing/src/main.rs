fn main() {
    println!("Hello, world!");

    // Three rules of Ownership:
    // 1. Each value in rust has an OWNER.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.


    // Learning ownership using strings
    // create a string literal (stored on heap)
    let s = String::from("hello");

    println!("{}", s);

    // this kind of string can be mutated
    // however string literals cannot, the difference is the way
    // in which they handle memory.
    let mut s = String::from("hello");
    s.push_str(", world"); // push_str appends a literal to a string

    println!("{}", s);

    // the String type allocates memory on the heap, and s is just a ptr reference
    // to the string on the heap. Therefore, if we did let s2 = s; then both will
    // point to the same block of memory on the heap.

    // The problem arises when both s1 and s2 go out of scope at the same time,
    // rust will try to free them both and this results in a possible 'double free' of
    // the same memory on the heap. But this is avoided, as the reference is 'moved' into s2

    // A 'move' happens: meaning when s2 is declared, the first variable s1 is invalidated,
    // and then the reference is moved to s2. Therefore, when we try to print s1 after declaring
    // s2, the following error occurs:

// 2 |     let s1 = String::from("hello");
//   |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
// 3 |     let s2 = s1;
//   |              -- value moved here
// 4 |
// 5 |     println!("{}, world!", s1);
//   |                            ^^ value borrowed here after move

    // This solves the issue, because now only s2 is freed when both s1 and s2 go out of scope
    // because s1 is already invalidated by the move.

    // If we need to perform a deep copy, then we can use the clone() method for String s
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack only data copy
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // This example with integers contradicts what was mentioned above, because x is not
    // being invalidated. The reason is that types such as integers have a known size at
    // compile time and stored entirely on the stack. There is no difference between deep and
    // shallow copy here, so it just copies the data to y, unlike in the case of strings.

    // We cannot add the 'Copy' trait to a type if it has implemented the 'Drop' trait. This
    // is because if we need to do anything special to the type when it goes out of scope, then
    // we will get a compile time error if we try to implement the "Copy" trait for it.
    // Therefore, the only types that implement the copy trait are simple scalar values like 
    // u32, i32, bool, float, char, and tuples with these simple types.

    main2();

    main3();

    main4();

    main5();

    main6();

    main7();

    main8();

    main9();

}


// Ownership and functions

fn main2() {


    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

}   // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.4


// Return Values and Scope

fn main3() {
    let s1 = gives_ownership();         // gives_ownership moves its return
    println!("{s1}");                           // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    println!("{s3}");
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

// Without pass by reference and only pass by value as mentioned above,
// we would need to write code like this: (in order to keep using all values, 
// till the end of the scope)

fn main4() {
    let s1 = String::from("hello");
    let (s2, len) = calc_length(s1);

    println!("The length of {s2} is {len}.");

}

fn calc_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns string length

    (s, length)
}

// What if we want to let a function use a value but not take
// ownership? Pass by reference? So that we do not have to pass
// and return values that do not need to be modified to functions?

// References and Borrowing

fn main5() {

    // A reference is like a pointer, we can follow to access the data stored at an
    // address. Unlike a pointer, a reference is guarnteed to point to a valid value of a
    // particular type for the lifetime of that reference.

    let s1 = String::from("hello");

    let len = calc_length2(&s1); // we pass a reference of s1 into calc_length2

    println!("The length of string {s1} is {len}.");
    
}

fn calc_length2(s: &String) -> usize {  // the function accepts &String as an argument
    s.len()                             // these are references and they let you refer to some value without taking ownership of it.
    // because s does not own s1, the value that s1 has will not be dropped after this function.
}

// We call the action of creating a reference: '&s1' as BORROWING. Meaning, it is temporarily given
// to the reference s and then it gets returned. The value of s1 is never owned by s.

// What if we try to modify something while borrowing?

fn main6() {

    let s1 = String::from("hello");

    // change(&s1);

    println!("The string s1: {s1}.");

}

// fn change(str: &String) {
//     str.push_str(", changed value"); //WRONG, cannot change a reference!
// }

// References are immutable by default, just like variables.


// Mutable References

fn main7() {
    let mut s = String::from("hello");

    change(&mut s); // notice we added &mut instead of just &

    println!("Print after mutate: {s}");
}

fn change(some_string: &mut String) { // this is a mutable reference and hence can be modified.
    some_string.push_str(", world");
}

// NOTE: Mutable references have one big restriction: if you have a mutable reference to a value, 
// you can have no other references to that value!

// For example, this is invalid code:

fn main8() {

    let mut s = String::from("hello");

    let _r1 = &mut s;

    // ERROR: cannot borrow `s` as mutable more than once at a time
    // let r2 = &mut s;

    // println!("Two mutable refs? : {r1} {r2}");

    // Again, this is done in order to prevent 'double free' and other data races.

    // A data race is defined as: (these can cause undefined behavior)
    // 1. Two or more pointers access the same data at the same time.
    // 2. At least one of the pointers is being used to write to the data.
    // 3. There’s no mechanism being used to synchronize access to the data.


    // NOTE: Cannot combine both mutable and immutable references!

    let _r1 = &s;
    let _r2 = &s;
    // let _r3 = &mut s; // PROBLEM: can't borrow s as mutable because it is already borrowed as immutable

    // println!("Refs: {_r1} {_r2} {_r3}");

    // We also cannot have a mutable reference while we have an immutable one to the same value.

    // However, this code block is valid:

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem

    // println!("ref r1: {r1}"); // PROBLEM, r1 is back in scope and a mutable reference of s was just declared.

    println!("{}", r3);

    // IMPORTANT NOTE ABOUT SCOPE OF VARIABLES:
    // Why? Because of scope! Scope is not necessarily till the end of the function, instead it is only
    // till the point where a variable is used!! Therefore, after r1 and r2 are done printing, they are out
    // of scope and a new mutable reference can be created, even before the function ends.

}


// Dangling References

fn main9() {

    // It is easy to create a dangling reference. A dangling reference is a location in memory which may
    // have been given to someone else, i.e. freeing that location while still holding a pointer to the location.

    // let reference_to_nothing = dangle();

    no_dangle();
}

// ERROR: this function's return type contains a borrowed value, but there is no value
// for it to be borrowed from

// fn dangle() -> &String { // returns a reference to a String
//     let s = String::from("hello"); // s is a new String

//     &s // We return a reference to the String s
// } // Here s goes out of scope, and its value is dropped. The memory is freed, but the reference is returned!
// // Dangling!

fn no_dangle() -> String {
    let s = String::from("hello");
    s
} // Here the value of s is not dropped, instead s is 'moved' to the calling function, and
// nothing is deallocated.

// Rules of references:
// 1. At any given time, you can have only one mutable reference or many immutable references.
// 2. References must always be valid.
