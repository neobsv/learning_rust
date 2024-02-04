// Collections

// Rust’s standard library includes a number of very useful data structures called collections
// the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time
// and can grow or shrink as the program runs.

// 1. Vector: allows you to store a number of values next to each other.

// 2. String: collection of chars, not just the String type, lots of others.

// 3. HashMap: associative data structure, the more general data structure is called map, hashmap is a particular implementation.

fn main() {
    println!("Hello, world!");

    // Storing lists of values with vectors

    // We’ll look at is Vec<T>, also known as a vector. Vectors allow you to store more than one value in a single data structure that
    // puts all the values next to each other in memory. Vectors can only store values of the same type

    let _v: Vec<i32> = Vec::new(); // creating a vector
                                   // Note: the type needs to be specified here because there are no values being inserted into this

    // More often, you’ll create a Vec<T> with initial values and Rust will infer the type of value you want to store,
    // so you rarely need to do this type annotation. Rust conveniently provides the vec! macro

    let _v = vec![1, 2, 3];

    // Updating a vector: (for this we need to make it mutable)

    let mut v = Vec::new();

    v.push(50);
    v.push(51);

    // Reading elements of vectors:

    // via indexing or using the get method:

    let v = vec![2, 5, 43, 23, 3];

    let third: &i32 = &v[2];
    println!("The third elem is: {}", third);

    let third: Option<&i32> = v.get(2); // In case the index is out of range this returns None

    match third {
        Some(third) => println!("The third elem is: {}", third),
        None => println!("Not found!"),
    }

    // Ownership and borrowing rules

    let mut v = vec![1, 2, 3, 4, 5];

    let _first = &v[0];

    v.push(6); // ERROR: Can't borrow v mutably here, because we have already created an immutable reference 'first' in the previous line which is still in scope

    // println!("The first element is: {first}"); // ERROR: the immutable borrow is still in scope, after performing a mutable borrow in the previous line!

    // Note: The push statement could invoke a dynamic resize of the vector, in which case a new vector would be created and then the reference 'first'
    // would be pointing to deallocated memory! The borrowing rules prevent us from ending up in that situation.

    // Iterating over Values in a Vector

    let v = vec![10, 20, 30];
    for i in &v {
        println!("{i}");
    }

    // we can also iterate over mutable references to elements
    let mut v = vec![10, 20, 30];
    for i in &mut v {
        *i += 10; // Dereference operator: used to access the value in the reference i, thereby changing the values of the vector
    }

    // Using an enum to store multiple types:

    // Vectors can only store values that are the same type. This can be inconvenient;
    // there are definitely use cases for needing to store a list of items of different types

    // We can define an enum whose variants will hold the different value types, and all the
    // enum variants will be considered the same type: that of the enum

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Note: If you don’t know the exhaustive set of types a program will get at runtime to store in a vector,
    // the enum technique won’t work. Instead, you can use a trait object (discussed later).

    // Dropping a Vector, drops its elements

    // Like any other struct, a vector is freed when it goes out of scope

    {
        let _v = vec![1, 3, 4, 5];
    } // v goes out of scope here, so all the elements along with the reference to v are dropped.

    // The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.

    main2();
}

// Strings

// string slices (&str), are references to some UTF-8 encoded string data stored elsewhere.  String literals, for example, are stored in the program’s binary and are therefore string slices.
// The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. These are stored on the heap.

fn main2() {
    // Creating a new string

    let mut _s = String::new();

    // Operations available with Vec<T> are available with String as well, because String is actually implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities

    // using the to_string() method on a string slice to load data into a String type.
    let data = "initial contents";
    let _s = data.to_string();

    // can be done directly like this
    let _s = "initial contents".to_string();

    // String::from can also be used to create a String from a string literal
    let _s = String::from("initial contents");

    // Updating a string

    let mut s = String::from("hello ");
    s.push_str("bar"); // append a string slice to a String

    // The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter
    // This means that, we can still use the string slice after the push_str,
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // s2 is not being taken ownership of, by push_str
    println!("s2 is {s2}"); // can be used here because s2 was not moved

    // The push method can be used to add one char to the string,
    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with the + Operator or the format! macro

    // Using the + operator,
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //  The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator.
    // The + operator uses the add method, whose signature looks something like this:

    // fn add(self, s: &str) -> String {

    // because of the s parameter in the add function: we can only add a &str to a String; we can’t add two String values together.
    // But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add

    // IMPORTANT: The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].
    // we can see in the signature that add takes ownership of self, because self does not have an &. This means s1 will be 'moved' into the add call and will no longer be valid after that.
    // So, let s3 = s1 + &s2; This statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result.

    // The format! macro

    // The format macro works just like println! except it takes in references to the input Strings and returns a concatenated String result:

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // NOTE: this call doesn’t take ownership of any of its parameters!
    let s = format!("{s1}-{s2}-{s3}");
    println!("concatenated: {}", s);
}
