// RefCell<T> and the Interior Mutability Pattern

// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. 
// UNSAFE: To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing

// We can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can’t guarantee that. 
// The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.

// Enforcing Borrowing Rules at Runtime with RefCell<T>

// RefCell<T> type represents single ownership over the data it holds. So, what makes RefCell<T> different from a type like Box<T>?
// Borrowing rules:
    // 1. At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
    // 2. References must always be valid.

// With references and Box<T>, the borrowing rules’ invariants are enforced at compile time, you'll get a compile error if it breaks the borrowing rules.
// With RefCell<T>, these invariants are enforced at runtime, your program will panic and exit if it breaks the borrowing rules.

// The advantage of checking the borrowing rules at runtime instead is that certain memory-safe scenarios are then allowed, where they would’ve been disallowed by the compile-time checks
// The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

// IMPORTANT: RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.

/* 

IMPORTANT: Reasons to choose Box<T>, Rc<T>, RefCell<T>

Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

Mutating the value inside an immutable value is the INTERIOR MUTABILITY pattern.

*/

// Interior Mutability: A mutable borrow to an immutable value

use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    // An immutable value, you can’t borrow it mutably:
    let _x = 5;
    // ERROR: let y = &mut x;

    // RefCell<T> doesn’t get around the borrowing rules completely: the borrow checker in the compiler allows this interior mutability, and the borrowing rules are checked at runtime instead. 
    // If you violate the rules, you’ll get a panic! instead of a compiler error.
    let y = RefCell::new(_x);
    println!("this is y: {:?}", y);

    // Mock Objects: Usecase for Interior Mutability

    // Mock objects are specific types of test doubles that record what happens during a test so you can assert that the correct actions took place.
    // We’ll create a library that tracks a value against a maximum value and sends messages based on how close to the maximum value the current value is. 
    // This library could be used to keep track of a user’s quota for the number of API calls they’re allowed to make    

    // Refer lib.rs for the lib, it is a simple messenger application which has a send function and a limit tracker attached to it which checks the current messages/max messages percentage.


    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
    
    // Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data. 
    // If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!

    // Taking the same Cons example as before in the rc_smart_pointer package,
    // Let’s add in RefCell<T> to gain the ability to change the values in the lists.

    use crate::List::{Cons, Nil};

    // We create a value that is an instance of Rc<RefCell<i32>> and store it in a variable named value so we can access it directly later.

    let value = Rc::new(RefCell::new(5));

    // We need to clone value so both a and value have ownership of the inner 5 value rather than transferring ownership from value to a or having a borrow from value.
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // We wrap the list a in an Rc<T> so when we create lists b and c, they can both refer to a
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // We can now mutate this value because it is a RefCell<T>
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    /* SUMMARY:

        1. By using RefCell<T>, we have an outwardly immutable List value. We can use the methods on RefCell<T> that provide access to its interior mutability 
        so we can modify our data when we need to. 
        2. The runtime checks of the borrowing rules protect us from data races, and it’s sometimes worth trading a bit of speed for this flexibility in our data structures
        3. RefCell<T> does not work for multithreaded code! Mutex<T> is the thread-safe version

    */

    main2();


}

// Reference Cycles Can Leak Memory

// We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other in a cycle. 
// This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.

// Creating a Reference Cycle using a Linked List


#[derive(Debug)]
enum List2 {
    Cons(i32, RefCell<Rc<List2>>),
    Nil,
}

use crate::List2::{Cons, Nil};

impl List2 {
    fn tail(&self) -> Option<&RefCell<Rc<List2>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// List2: The second element in the Cons variant is now RefCell<Rc<List2>>, meaning that instead of having the ability to modify the i32 value, we want to modify the List2 value a Cons variant is pointing to. 
// We’re also adding a tail method to make it convenient for us to access the second item if we have a Cons variant.

fn main2() {

    // This code creates a list in a and a list in b that points to the list in a. Then it modifies the list in a to point to b, creating a reference cycle.

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());


    // We modify a so it points to b instead of Nil, creating a cycle. 
    // We do that by using the tail method to get a reference to the RefCell<Rc<List>> in a, which we put in the variable link

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle; it will overflow the stack
    // STACK OVERFLOW: println!("a next item = {:?}", a.tail());

    // If you have RefCell<T> values that contain Rc<T> values or similar nested combinations of types with interior mutability and reference counting, you must ensure that you don’t create cycles
    // Another solution for avoiding reference cycles is reorganizing your data structures so that some references express ownership and some references don’t. (will be explained)
    // Ownership: Will control which values will be dropped. Non Ownership: Dropped automatically when out of scope. A careful combination of the two is needed.

    // Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>

    // Calling Rc::clone increases the strong_count of an Rc<T> instance, and an Rc<T> instance is only cleaned up if its strong_count is 0.
    // You can also create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade and passing a reference to the Rc<T>.
    // Weak references don’t express an ownership relationship, and their count doesn’t affect when an Rc<T> instance is cleaned up. 
    // They won’t cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.

    // To access a Weak<T> call the upgrade() method on a Weak<T> instance, which will return an Option<Rc<T>>. 
    // You’ll get a result of Some if the Rc<T> value has not been dropped yet and a result of None if the Rc<T> value has been dropped.


    // Creating a Tree Data Structure, Node with Child Nodes

    // We want a Node to own its children, and we want to share that ownership with variables so we can access each Node in the tree directly.
    // So we have a Vec<T> with type Rc<Node> so that ownership can be shared.
    // We wrap the childern Vec<T> into a RefCell<T> in order to modify the children of each node.


    // IMPORTANT: a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. 
    // IMPORTANT: However, a child should not own its parent: if we drop a child node, the parent should still exist.
    // This is where we use Weak<T> references!

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>
    }

    // Create a couple of nodes, one parent pointing to a child node
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });

    dbg!("leaf value = {}", leaf.as_ref().value );

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new())
    });

    dbg!("branch value = {}", branch.as_ref().value );

    // Once we have the Node instance in branch, we can modify leaf to give it a Weak<Node> reference to its parent. We use the borrow_mut method on the RefCell<Weak<Node>> in the parent field of leaf, 
    // and then we use the Rc::downgrade() function to create a Weak<Node> reference to branch from the Rc<Node> in branch.

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // We clone the Rc<Node> in leaf and store that in branch, meaning the Node in leaf now has two owners: leaf and branch. 
    // We can get from branch to leaf through branch.children, but there’s no way to get from leaf to branch (i.e. no parent relationships)

    dbg!("leaf: {:#?} branch: {:#?}", &leaf, &branch);

    // Print the parent by accessing the Weak<T> reference

    // When we print the parent of leaf again, this time we’ll get a Some variant holding branch: now leaf can access its parent! 
    // When we print leaf, we also avoid the cycle that eventually ended in a stack overflow

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // Same with branch.children it contains leaf, and leaf.parent will point back to branch, but it is a Weak<T> reference so it doesn't stack overflow
    println!("branch children = {:?}", branch.children.borrow());


    // Visualizing changes to strong_count() and weak_count()

    // At each stage in the previous example of creating a leaf and a branch, we will print out the Rc::strong_count() and Rc::weak_count() for the nodes

    let leaf2 = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!(
        "leaf2 strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2)
    );

    // Here only one Rc<T> ref of Node exists and it is the leaf node.
    // leaf strongcount=1 weakcount=0

    {
        let branch2 = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf2)])
        });

        // Here one Rc<T> for branch exists, and then we have cloned the leaf node using Rc::clone() so leaf 2 0, branch 1 0
        // leaf strongcount=2 weakcount=0
        // branch strongcount=1 weakcount=0

        println!(
            "branch2 strong = {}, weak = {}",
            Rc::strong_count(&branch2),
            Rc::weak_count(&branch2)
        );

        println!(
            "leaf2 strong = {}, weak = {}",
            Rc::strong_count(&leaf2),
            Rc::weak_count(&leaf2)
        );

        *leaf2.parent.borrow_mut() = Rc::downgrade(&branch2);

        // Here we have set a Weak<T> from leaf to branch which means branch will have a weak ref created stored in leaf.parent, makes leaf 2 0 and branch 1 1
        // leaf strongcount=2 weakcount=0
        // branch strongcount=1 weakcount=1

        println!(
            "branch2 strong = {}, weak = {}",
            Rc::strong_count(&branch2),
            Rc::weak_count(&branch2)
        );

        println!(
            "leaf2 strong = {}, weak = {}",
            Rc::strong_count(&leaf2),
            Rc::weak_count(&leaf2)
        );
    } 

    // IMPORTANT: Here branch is dropped, because it goes out of scope its strongcount becomes 0 and it is cleaned up, however its weak count was still one,
    // but weak counts are not checked before cleaning up references! This is important to understand.
    // Leaf's strongcount is decremented from 2 to 1, because branch has now been dropped which was holding a strong reference to leaf in its children Vec<T>

    // Leaf still has strongcount 1, and will be cleaned up at the end of main, now its parent points to None because branch has already been dropped.
    // leaf strongcount=1 weakcount=0
    println!("leaf2 parent = {:?}", leaf2.parent.borrow().upgrade());
    println!(
        "leaf2 strong = {}, weak = {}",
        Rc::strong_count(&leaf2),
        Rc::weak_count(&leaf2)
    );

}

/* SUMMARY:

1. Box<T> type has a known size and points to data allocated on the heap
2. Rc<T> type keeps track of the number of references to data on the heap so that data can have multiple owners
3. RefCell<T> type with its interior mutability gives us a type that we can use when we need an immutable type but need to change an inner value of that type.
It also enforces the borrowing rules at runtime instead of at compile time.
4. Weak<T> can be used to prevent reference cycles by assigning them to places where cylical references are needed, and they will be dropped at the end of the scope no matter what their ref count is.

*/