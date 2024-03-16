// Smart Pointers

// A pointer or a reference is a variable that holds a memory address
// Usually this address refers to or points to some data. They are indicated by the &x symbol and borrow the value they point to.


// Smart pointers on the other hand are data structures that act like a pointer and have additional metadata and capabilities.
// They come from C++ which has its RAII (resource acqusition is initialization) concepts. This means that if the pointer is never used then it will never be initialized, and they are implicitly destroyed after use.
// Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers: while references only borrow data, in many cases, smart pointers own the data they point to.

// String and Vec<T> are techically smart pointers because they store metadata and data.
// Smart pointers are implemented using structs, and they implement addtional traits called the Drop and Deref traits.
// The Deref trait allows an instance of the smart pointer struct to behave like a reference.
// The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

// We will look at these three types of smart pointers:

// Box<T> for allocating values on the heap
// Rc<T>, a reference counting type that enables multiple ownership
// UNSAFE: Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time.

// UNSAFE: In addition, interior mutability will also be discussed, which is where an immutable type exposes an API for mutating the inner value. This is considered UNSAFE code.


// Using Box<T> to point to data on the heap

// Boxes allow you to store data on the heap rather than the stack. They have a fixed size on the stack, and point to data of variable size on the heap.
// Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack

// The situations that mandate the use of Box<T>:
    // 1. Type whose size can't be known at compile time, so you make it a Box<T> so that it gets stored on the heap
    // 2. You have a large amount of data and you do not want to call clone() on the variable but still transfer ownership
    // 3. You want a Type that implements a particular trait and you want to own the value. This is known as a trait object. (discussed later)

// Enabling recursive types like linked lists is an excellent use case for Box<T>.

// Using Box<T> to store data on the heap

use std::ops::Deref;

fn main() {
    println!("Hello, world!");

    // The variable b is stored on the heap. Both the pointer b stored on the stack, and the value 5 stored on the heap get deallocated
    // when the main function ends.
    let b = Box::new(5);
    println!("b = {}", b);

    // Enabling recursive types

    // A value of recursive type can have another value of the same type as part of itself. Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up. Recursive types could take up infinite space
    // The example of this is a linked list: (1, (2, (3, Nil)))
    // Each item contains two elements, one is an integer and the second is a pointer to the next element. The last item contains a value called Nil.

    // Enum to define a linked list

    /* ERROR: size of the enum is infinite because it contains a recursive definition of the List enum
    enum List {
        Cons(i32, List),
        Nil
    }
    */

    enum _Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Consider the Message enum discussed before, to allocate space for a message value, the compiler goes through each of the variants to see which ones need more space
    // Message::Quit doesn't need any space and Message::Move needs two i32s amount of space. The most space a Message value will need is the space it would take to store the largest of its variants.
    // This is similar to how the space needed for a union is computed in C/C++. 


    // Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to. 
    // This means we can put a Box<T> inside the Cons variant instead of another List value directly

    use crate::List::{Cons, Nil};

    let _list = Cons(1, Box::new( Cons(2,  Box::new( Cons(3,  Box::new(Nil)))))); // Nil, the non-recursive variant that signals the end of the list.

    // The Cons variant needs the size of an i32 plus the space to store the box’s pointer data.
    // By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value

    // Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities. They also don’t have the performance overhead that these special capabilities incur.
    // The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. 
    // When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation.

    main2();

    main3();

}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Treating Smart Pointers Like Regular References with the Deref Trait

// Implementing the Deref trait allows you to customize the behavior of the dereference operator *

// Let’s first look at how the dereference operator works with regular references:

// Following the Pointer to the Value:
// A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else
// Create a ref y, if we want to make an assertion about the value in y, we have to use *y to follow the reference to the value it’s pointing to (hence dereference) so the compiler can compare the actual value
/*
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
*/

// Using Box<T> Like a Reference

// We can rewrite the code in to use a Box<T> instead of a reference:
/*
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
*/
// Just like with references, we see that we are able to call the dereference operator on the Box<T> instance to get the value it holds

// Defining Our Own Smart Pointer

// The Box<T> type is ultimately defined as a tuple struct with one element, 
// We define a MyBox<T> type in the same way. We’ll also define a new function to match the new function defined on Box<T>.
// The MyBox type is a tuple struct with one element of type T. The new function returns a MyBox instance that holds the value passed in.

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main2() {

    let x = 5;
    let _y = MyBox::new(x);

    assert_eq!(5, x);
    //ERROR: Deref trait not implemented: assert_eq!(5, *y);

}

// Treating a Type Like a Reference by Implementing the Deref Trait

// To implement a trait, we need to provide implementations for the trait’s required methods. The Deref trait, provided by the standard library, 
// requires us to implement one method named deref that borrows self and returns a reference to the inner data.

impl<T> Deref for MyBox<T> {

    // The type Target = T; syntax defines an associated type for the Deref trait to use. Associated types are a slightly different way of declaring a generic parameter

    type Target = T;

    fn deref(&self) -> &Self::Target {

        // &self.0 so deref returns a reference to the value we want to access with the * operator
        &self.0
    }
}

// NOTE: Without the Deref trait, the compiler can only dereference & references. The deref method gives the compiler the ability to take a value of any type that implements 
// Deref and call the deref method to get a & reference that it knows how to dereference.

/* IMPORTANT:
The reason the deref method returns a reference to a value, and that the plain dereference of y in the main function is still necessary, is to do with the ownership system. 
If the deref method returned the value directly instead of a reference to the value, the value would be moved out of self. 
We don’t want to take ownership of the inner value inside MyBox<T> in this case or in most cases where we use the dereference operator.
*/

// Implicit Deref Coercions with Functions and Methods

// Lets say we have a type like MyBox in the previous example which implements the Deref trait. Then deref coercion can convert &MyBox to &T because MyBox returns a ref to type T in the deref method.

// Definition: Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type. 
// For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str.

// It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. 
// A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

// To see deref coercion in action, let’s use the MyBox<T> type we defined as well as the implementation of Deref that we added. Shows the definition of a function that has a string slice parameter:

fn main3() {

    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    // We can call the hello function with a string slice as an argument, such as hello("Rust"); for example. 
    // Deref coercion makes it possible to call hello with a reference to a value of type MyBox<String>:

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Because we implemented the Deref trait on MyBox<T>, Rust can turn &MyBox<String> into &String by calling deref. The standard library provides an implementation of Deref on String 
    // that returns a string slice, and this is in the API documentation for Deref. Rust calls deref again to turn the &String into &str. This is why the function hello() works with a &MyBox<String> as input.


    // How Deref Coercion Interacts with Mutability

    // Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

    // Rust does deref coercion when it finds types and trait implementations in three cases:
        // 1. From &T to &U when T: Deref<Target=U>
        // 2. From &mut T to &mut U when T: DerefMut<Target=U>
        // We just saw examples of these two cases above with the &MyBox<String> -> &String -> &str

        // 3. From &mut T to &U when T: Deref<Target=U>
        // The third case is trickier: Rust will also coerce a mutable reference to an immutable one. 
        // But the reverse is not possible: immutable references will never coerce to mutable references. This is due to the borrowing rules where you cannot have more than one mutable reference of an object in a given scope.
    
    // The first two cases are the same as each other except that the second implements mutability.
    

    // Running code on Cleanup with the Drop trait

    // The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope.
    // We’re introducing Drop in the context of smart pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer.
    // Shows Drop in the context of smart pointers because the functionality of the Drop trait is almost always used when implementing a smart pointer. For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to.

    // Other languages have destructors, this is similar.

    // The Drop trait requires you to implement one method named drop that takes a mutable reference to self. To see when Rust calls drop, let’s implement drop with println! statements for now.

    struct CustomSmartPointer {
        data: String,
    }
    
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }


    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Variables are dropped in the reverse order of their creation, so d was dropped before c. This example’s purpose is to give you a visual guide to how the drop method works

    // Dropping a Value Early using std::mem::drop

    // You might want to clean up a value early. One example is when using smart pointers that manage locks: you might want to force the drop method that releases the lock so that other code in the same scope can acquire the lock.
    // Rust doesn’t let you call the Drop trait’s drop method manually; instead you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.

    // Side Note: Can't disable the automatic drop functionality, it’s not straightforward to disable the automatic drop functionality. Disabling drop isn’t usually necessary; the whole point of the Drop trait is that it’s taken care of automatically.

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    // println!("CustomSmartPointer created.");
    // c.drop(); ERROR: Calling the drop method from the Drop trait, This would cause a DOUBLE FREE error because Rust would be trying to clean up the same value twice while calling drop at the end of the main function automatically!
    // println!("CustomSmartPointer dropped before the end of main.");

    // NOTE: So, if we need to force a value to be cleaned up early, we use the std::mem::drop function. The function is in the prelude.

    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // The text Dropping CustomSmartPointer with data `some data`! is printed between the CustomSmartPointer created and CustomSmartPointer dropped before the end of main.

    // NOTE: You also don’t have to worry about problems resulting from accidentally cleaning up values still in use: the ownership system that makes sure references are always valid also ensures that drop gets called only once when the value is no longer being used.


}