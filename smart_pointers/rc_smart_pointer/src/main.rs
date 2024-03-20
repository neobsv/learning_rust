// Rc<T> The Reference Counted Smart Pointer

// There are cases when a single value might have multiple owners. For example, in graph data structures, multiple edges might point to the same node, 
// and that node is conceptually owned by all of the edges that point to it. A node shouldn't be cleaned up unless it doesn't have any edges pointing to it and no owners.

// You have to enable multiple ownership explicitly by using the Rust type Rc<T>, which is an abbreviation for reference counting. (keeps track of the number of refs to a value, if zero then the value can be cleaned up)
// We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last.

// Example: Cons list using Rc<T> to share data

// We defined it using Box<T>. This time, we’ll create two lists that both share ownership of a third list.

enum BoxList {
    BCons(i32, Box<BoxList>),
    BNil,
}

use crate::BoxList::{BCons, BNil};

use std::rc::Rc;

enum RcList {
    RCons(i32, Rc<RcList>),
    RNil,
}

use crate::RcList::{RCons, RNil};


fn main() {
    let _a = BCons(5, Box::new(BCons(10, Box::new(BNil))));
    let _b = BCons(3, Box::new(_a)); // points to A
    // ERROR: let c = Cons(4, Box::new(a)); // points to A, because value 'a' was already moved to the list b

    // When we create the b list, a is moved into b and b owns a. Then, when we try to use a again when creating c, we’re not allowed to because a has been moved.
    // We could change the definition of Cons to hold references instead, but then we would have to specify lifetime parameters. By specifying lifetime parameters, we would be specifying that 
    // every element in the list will live at least as long as the entire list. (not great)
    
    // we’ll change our definition of List to use Rc<T> in place of Box<T>. Each Cons variant will now hold a value and an Rc<T> pointing to a List.
    // When list b is created, we clone the Rc<List> a is holding then list a's Rc<T> increases by one (total 2), and let list a and list b share ownership of the data in the Rc<List>.
    // When list c is created, again list a's Rc<List> is cloned, and then list a's Rc<T> increases by one again (total 3).

    let a = Rc::new(RCons(5, Rc::new( RCons(10, Rc::new(RNil)) ) ));
    let _b = RCons(3, Rc::clone(&a)  );
    let _c = RCons(4, Rc::clone(&a)  );

    // IMPORTANT: Every time we call Rc::clone, the reference count to the data within the Rc<List> will increase, and the data won’t be cleaned up unless there are zero references to it.
    // IMPORTANT: The implementation of Rc::clone doesn’t make a deep copy of all the data. The call to Rc::clone only increments the reference count, which doesn’t take much time.
    // When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to Rc::clone.


    // Cloning an Rc<T> increases the reference count

    // Add an inner scope around list c; then we can see how the reference count changes when c goes out of scope.

    let a1 = Rc::new(RCons(5, Rc::new(RCons(10, Rc::new(RNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a1));
    let _b1 = RCons(3, Rc::clone(&a1));
    println!("count after creating b = {}", Rc::strong_count(&a1));
    {
        let _c1 = RCons(4, Rc::clone(&a1));
        println!("count after creating c = {}", Rc::strong_count(&a1));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a1));

    // We can see that the Rc<List> in a has an initial reference count of 1; then each time we call clone, the count goes up by 1. When c goes out of scope, the count goes down by 1.

    // What we can’t see in this example is that when b and then a go out of scope at the end of main, the count is then 0, and the Rc<List> is cleaned up completely. Using Rc<T> allows a single value to have multiple owners, 
    // and the count ensures that the value remains valid as long as any of the owners still exist.

    // NOTE: Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only. Immutable Rc<T> refs are not possible due to borrowing rules.



}
