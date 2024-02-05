// Collections

// Rust’s standard library includes a number of very useful data structures called collections
// the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time
// and can grow or shrink as the program runs.

// 1. Vector: allows you to store a number of values next to each other.

// 2. String: collection of chars, not just the String type, lots of others.

// 3. HashMap: associative data structure, the more general data structure is called map, hashmap is a particular implementation.

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

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

    main3();

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

    // Indexing into Strings

    // Accessing individual characters in a string by referencing them by index is a valid and common operation. However, if you try to access parts of a String using indexing syntax in Rust, you’ll get an error.
    let _s1 = String::from("hello");
    // let h = s1[0]; // ERROR: String cannot be indexed by integer

    // A String is a wrapper over a Vec<u8>. The vector storing the string “Hola” is 4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8

    // However, consider the following non english language,
    let _hello = String::from("Здравствуйте");
    // The number of bytes it takes to encode “Здравствуйте” in UTF-8 is 24, because each Unicode scalar value in that string takes 2 bytes of storage. Therefore, an index into the string’s bytes will not always correlate to a valid Unicode scalar value

    // Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).
    // If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,224, 165, 135]

    // That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:
    // ['न', 'म', 'स', '्', 'त', 'े']

    // There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:
    // ["न", "म", "स्", "ते"]

    // One more reason that Rust doesn't allow us to index Strings is because operations are expected to take O(1) time but rust linear searches the String so, we can't guarantee that

    // You can convert a String or a &str to a vector of chars and then index that vector
    let s = "Hello world!";
    let my_vec: Vec<char> = s.chars().collect();
    println!("my_vec[0]: {}", my_vec[0]);
    println!("my_vec[1]: {}", my_vec[1]);

    // Slicing Strings

    //  It’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.

    // Indexing is possible with a range specified within []
    let hello = "Здравствуйте";
    let _s = &hello[0..4];
    // Here, s will be a &str that contains the first 4 bytes of the string. Earlier, we mentioned that each of these characters was 2 bytes, which means s will be Зд.

    // NOTE: Use ranges with caution! If we were to try to slice only part of a character’s bytes with something like &hello[0..1], Rust would panic at runtime in the same way as if an invalid index were accessed in a vector

    // Methods for iterating over strings

    // For unicode scalar values, use the chars() method:
    for c in "Зд".chars() {
        println!("{c}");
    }
    
    // The bytes() method returns each raw byte
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // There is a crate called unicode-segmentation which can be used to print individual grapheme clusters
    for g in  UnicodeSegmentation::graphemes("नमस्ते", true).collect::<Vec<&str>>() {
        println!("{g}");
    }

    // Programmers have to put more thought into handling UTF-8 data upfront. This trade-off exposes more of the complexity of strings than is apparent in other programming languages, but it prevents you from having to handle errors involving non-ASCII characters later in your development life cycle.


}


// Hashmaps

// Storing keys with associated values in Hash Maps, the type HashMap<K, V> stores a mapping of type K to type V using a hashing function

fn main3() {

    // Creating a hashmap with new() and inserting elements
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //  We need to bring it into scope using the 'use' keyword, It’s not included in the features brought into scope automatically in the prelude.

    // Hashmaps are homogeneous, the keys and values must have the same data types, and like vectors, hash maps store data on the heap

    // Accessing Values in a Hash Map

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // get() returns an Option<&V>, so we need to unwrap_or() it, meaning if it returns None then convert it into '0'

    dbg!("Blue team score: {:?}", score);

    // Iterating over a hashmap

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hashmaps and Ownership

    // IMPORTANT: For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values!
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Updating a HashMap

    // When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned:
    // 1. You could replace the old value with the new value, completely disregarding the old value. 
    // 2. You could keep the old value and ignore the new value, only adding the new value if the key doesn’t already have a value. 
    // 3.Or you could combine the old value and the new value    

    // Case 1: overwriting a value, simple just use insert() over and over again
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Case 2: Adding a key and value only if a value isn't present
    // Hash maps have a special API for this called entry that takes the key you want to check as a parameter.
    // The return value of the entry method is an enum called Entry that represents a value that might or might not exist
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // The or_insert() method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists.  
    // And if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value

    println!("{:?}", scores);

    // Case 3: Updating a value based on an old value

    // IMPORTANT: The or_insert method returns a mutable reference (&mut V) to the value for the specified key. Here we store that mutable reference in the count variable, 
    // so in order to assign to that value, we must first dereference count using the asterisk (*). 
    // The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Updated hashmap: {:?}", map);

    // Hashing Functions

    // By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables1. 
    // This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it
    //  You can switch to another function by specifying a different hasher. A hasher is a type that implements the BuildHasher trait. crates.io has libraries which provide hashers implementing many common hashing algorithms.

}

