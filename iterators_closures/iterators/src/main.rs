// Iterators

// Perform a task on a sequence of items in order
// Iterators are lazy, they don't have any effect until you call methods that consume the iterator to
// call next() on it/ use it up.

fn main1() {

    let v1 = vec![1, 2, 3];

    // Create an iterator over the items in the vector v1 by calling the iter method defined on Vec<T>
    let v1_iter = v1.iter();

    // When the for loop is called using the iterator in v1_iter, each element in the iterator is used in one iteration of the loop
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // NOTE: We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.

    // The Iterator trait and the next() method
    pub trait Iterator {
        // New syntax: type Item and Self::Item, which are defining an associated type with this trait
        // Implementing the Iterator trait requires that you also define an Item type, and this Item type is used in the return type of the next method
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }


    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // We needed to make v1_iter mutable: calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. 
    // In other words, this code CONSUMES, or uses up, the iterator.

    // IMPORTANT: The iter() method produces an iterator over immutable references. 
    // IMPORTANT: If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter() instead of iter().
    // IMPORTANT: If we want mutable references we can call iter_mut() instead of iter().

    // Methods that Consume the Iterator

    // The Iterator trait has a number of different methods with default implementations provided by the standard library.
    // Some of these methods call the next() method in their definition, which is why you’re required to implement the next() method while implementing the Iterator trait.

    // Methods that call next are called consuming adaptors, because calling them uses up the iterator.
    // We aren’t allowed to use v1_iter after the call to sum() because sum() takes ownership of the iterator we call it on.

}

#[allow(dead_code, unnameable_test_items)]
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

// Methods that Produce other Iterators
fn main2() {
    // Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator, instead they produce different iterators by changing some aspect of the original iterator

    // The map() method, takes a closure as input, which is called on each item as the items are iterated through. 
    // The map() method returns a new iterator that produces the modified items.

    let _v1: Vec<i32> = vec![1, 2, 3];

    // v1.iter().map(|x| x + 1); // ERROR: Unused map that must be used, since map() returns another iterator, we need to use it, for example call collect() on it.
    // Moreover, this code never gets executed. The closure we’ve specified never gets called. The warning reminds us why: iterator adaptors are lazy, and we need to consume the iterator here.

    // We can use collect() after the map() method to actually execute the iterator
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    
    // Iterator Chaining
    // You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, 
    // you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.
    

    // Using Closures that Capture Their Environment
    // Many iterator adapters take closures as arguments, and commonly the closures we’ll specify as arguments to iterator adapters will be closures that capture their environment.
    // The filter() method takes a closure. The closure gets an item from the iterator and returns a bool


}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[test]
fn filters_by_size() {

    // The shoes_in_size() function takes ownership of a vector of shoes and a shoe size as parameters. It returns a vector containing only shoes of the specified size.
    // In the body of shoes_in_size, we call into_iter to create an iterator that takes ownership of the vector. Then we call filter to adapt that iterator into a new iterator, for only the elements where the closure passed to filter() returns true
    // The closure captures the shoe_size parameter from the environment and compares the value with each shoe’s size in the shoes vector
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}


fn main() {
    println!("Hello, world!");
    main1();
    main2();
}
