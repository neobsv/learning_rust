// Slices

// Slices let you reference a contiguous sequence of elements in a collection rather
// than the whole collection. A slice is a kind of reference so it DOES NOT have ownership.

fn main() {
    println!("Hello, world!");

    let mut s = String::from("xyz id fff");

    let idx = first_word(&s);

    s.clear();

    // **** idx is no longer valid since the string has changed!
    println!("idx of the first word: {idx}");

    main2();

    main3();

    main4();
}

// Program to take string of words seperated by spaces and return the first word.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // so that each byte can be compared to b' '

    for (i, &item) in bytes.iter().enumerate() {
        // ^ Because we get a reference to the element from .iter().enumerate(), we use & in the pattern
        if item == b' ' {
            return i;
        }
    }

    s.len() // if there are no spaces, return string length
}

// **** We are returning a single usize value, but because it’s a separate value from the String,
// there’s no guarantee that it will still be valid in the future

// Introducing String Slices

fn main2() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // Rather than a reference to the entire string, hello is a reference to a portion
    // of the String, specified by the range operator within square brackets:  &s[0..5]

    // In memory, string slices contain a reference to the value from the particular start index
    // of the string and its length. The start, end indices are not stored as numbers explicitly.

    println!("H: {hello} W: {world}");

    // If the start is zero, that can be dropped. If the end is the last char/string length
    // then that can be dropped from the range operator. Both can be dropped to take the entire string.

    let hello = &s[..5];
    let world = &s[6..];

    let entire = &s[..];

    println!("H: {hello} W: {world} E: {entire}");

    // Now lets go back and rewrite first_word() to return a slice

    let s = String::from("hello world");
    let word = first_word2(&s);

    // Recall from the borrowing rules that if we have an immutable reference
    // to something, we cannot also take a mutable reference. clear() needs a mutable reference.

    // s.clear(); // ERROR: mutable and immutable references combined in the same scope
    // This is because word is also still an immutable reference to s, so s.clear() can't create a mutable reference.

    println!("the first word is: {word}");
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// String Literals as Slices

fn main3() {
    // To properly understand string literals, their type is actually &str
    // String literals are stored inside the binary executable.
    let l = "hello";
    // This is a slice s pointing to a specific point in the binary executable. That's also why string literals are immutable (immutable reference).

    print!("{l}");

    // String Slices as function Parameters

    let s = String::from("hello world");

    // `first_word3` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    first_word3(&s);
    first_word3(&s[..8]);

    // `first_word3` also works on slices of string literals
    let l2 = "xyz zzzz";
    first_word3(&l2[..6]);

    // `first_word3` also works on string literals directly! (string literals are string slices)
    first_word3(l2);
}

// We can use the same function on both &String values and &str values by changing
// the parameter type to a reference to a String Slice instead of a String.
fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// Other Slices

fn main4() {
    // There is a more general slice type, like &[i32] , &[u32] and so on...
    // It works the same way as string slices do, by storing a reference to the first element and a length
    let a = [1, 3, 5, 6, 7];

    let slice = &a[1..3];

    assert_eq!(slice, &[3, 5]);
}
