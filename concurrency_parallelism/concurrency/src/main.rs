// Using Threads to Run Code Simultaneously

// Multiple threads to run multiple tasks at the same time can improve performance
// There’s no inherent guarantee about the order in which parts of your code on different threads will run:
    // 1. Race conditions, where threads are accessing data or resources in an inconsistent order
    // 2. Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
    // 3. Bugs that happen only in certain situations and are hard to reproduce and fix reliably

// The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread. 
// There are crates that implement other models of threading that make different tradeoffs to the 1:1 model.

use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

fn main() {
    // Creating a New Thread with spawn()

    // The thread::spawn() function gets a closure from the calling function.

    thread::spawn(|| {
        for i in 1..10 {
            println!("hello number {} from the spawned thread!", i);

            // The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run.
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The threads will probably take turns, but that isn’t guaranteed: it depends on how your operating system schedules the threads

    for i in 1..5 {
        println!("Hello {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // All spawned threads are shutdown when main ends, even if they haven't finished execution!

    main2();

    main3();

    main4();

    main5();

    main6();

    main7();

    main8();

    main9();

}

// Waiting for All Threads to Finish Using join Handles

fn main2() {

    // We can fix the problem of the spawned thread not running or ending prematurely by saving the return value of thread::spawn in a variable. 
    // The return type of thread::spawn is JoinHandle. A JoinHandle is an owned value that, when we call the join method on it, will wait for its thread to finish.

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Blocking a thread means that thread is prevented from performing work or exiting. Because we’ve put the call to join after the main thread’s for loop

    handle.join().unwrap();

}

// Don't interleave, The main thread will wait for the spawned thread to finish and then run its for loop, so the output won’t be interleaved anymore

fn main3() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

}

// Using move Closures with Threads

// We'll often use the move keyword with closures passed to thread::spawn because the closure will then take ownership of the values it uses from the environment, transferring ownership.
// The closure we pass to thread::spawn takes no arguments: we’re not using any data from the main thread in the spawned thread’s code. 
// To use data from the main thread in the spawned thread, the spawned thread’s closure must capture the values it needs.

fn main4() {

    let v = vec![1, 2, 3];

    // By adding the move keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // ERROR: drop(v); This may drop the value of v before the thread executes.
    // Here dangerous statements like drop won't be allowed because the move keyword has transffered ownership of v to the thread.
    // If we added move to the closure, we would move v into the closure’s environment, and we could no longer call drop on it in the main thread.

    handle.join().unwrap();

    // Rust was being conservative and only borrowing v for the thread, which meant the main thread could theoretically invalidate the spawned thread’s reference.
    // By telling Rust to move ownership of v to the spawned thread, we’re guaranteeing Rust that the main thread won’t use v anymore.

}

// Message Passing to Transfer Data Between Threads (safe concurrency)

// Message passing: where threads or actors communicate by sending each other messages containing data
// Rust's standard library provides an implementation of channels. A channel is a general programming concept by which data is sent from one thread to another.

// A channel has two halves: a transmitter and a receiver. A channel is said to be closed if either the transmitter or receiver half is dropped.
// We’ll be sending simple values between threads using a channel to illustrate the feature. Once you’re familiar with the technique, you could use channels for any threads that need to communicate between each other.

fn main5() {
    /* MPSC Channels:
        We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer. 
        In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.

        The mpsc::channel function returns a tuple, the first element of which is the sending end--the transmitter--and the second element is the receiving end--the receiver.
        The brackets (tx, rx) unpack or destructure the tuple.

        Move the transmitting end into a spawned thread and have it send one string so the spawned thread is communicating with the main thread.
    */

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // Again, we’re using thread::spawn to create a new thread and then using move to move tx into the closure so the spawned thread owns tx. 
    // The spawned thread needs to own the transmitter to be able to send messages through the channel. The transmitter has a send method that takes the value we want to send. 
    // The send method returns a Result<T, E> type, so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error.

    let received = rx.recv().unwrap();
    // We’re calling unwrap to panic in case of an error. But in a real application, we would handle it properly.
    println!("Got: {}", received);

    // The receiver has two useful methods: recv and try_recv. We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel. 
    // Once a value is sent, recv will return it in a Result<T, E>. When the transmitter closes, recv will return an error to signal that no more values will be coming.

    // The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time. 
    // Using try_recv is useful if this thread has other work to do while waiting for messages: we could write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other work for a little while until checking again.


}

// Channels and Ownership Transference

// Let’s do an experiment to show how channels and ownership work together to prevent problems: we’ll try to use a val value in the spawned thread after we’ve sent it down the channel.

fn main6() {

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // ERROR: println!("Using val after sending it: {}", val);
        // Our concurrency mistake has caused a compile time error. The send function takes ownership of its parameter, and when the value is moved, 
        // the receiver takes ownership of it. This stops us from accidentally using the value again after sending it.
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// Sending Multiple Values and Seeing the Receiver Waiting

// The spawned thread will now send multiple messages and pause for a second between each message.

fn main7() {

    let (tx, rx) = mpsc::channel();

    // The spawned thread has a vector of strings that we want to send to the main thread. 
    // We iterate over them, sending each individually, and pause between each by calling the thread::sleep
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    });


    // In the main thread, we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator. 
    // For each value received, we’re printing it. When the channel is closed, iteration will end.
    for received in rx {
        println!("Got: {}", received);
    }

}

// Creating Multiple Producers By Cloning the Transmitter

// Earlier we mentioned that mpsc was an acronym for multiple producer, single consumer. Let’s put mpsc to use and expand the code to create multiple threads that all send values to the same receiver.

fn main8() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); // Creating an extra transmitter
    
    // First Producer uses trasmitter tx1
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(132));
        }
    });

    // Second producer uses transmitter tx
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(30));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // If you experiment with thread::sleep, giving it various values in the different threads, each run will be more nondeterministic and create different output each time!

}

// Shared State Concurrency

// Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time.
// Smart pointers made multiple ownership possible, multiple ownership can add complexity because these different owners need managing.

// Using Mutexes to Allow Access to Data from One Thread at a Time

/* IMPORTANT:

// Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time.
// To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock. The lock is a data structure.
// that is part of the mutex that keeps track of who currently has exclusive access to the data.
// Therefore, the mutex is described as guarding the data it holds via the locking system.

*/

// You must attempt to acquire the lock before using the data.
// When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

// API of Mutex<T>

fn main9() {
    let m = Mutex::new(5);

    {
        // To access the data inside the mutex, we use the lock method to acquire the lock. 
        // This call will block the current thread so it can’t do any work until it’s our turn to have the lock.
        let mut num = m.lock().unwrap();
        // After we’ve acquired the lock, we can treat the return value, named num in this case, as a mutable reference to the data inside. 
        // The type system ensures that we acquire a lock before using the value in m, the type of m is Mutex<i32> and lock() lets you mutate the value
        *num = 6;
    }
    // The lock object is automatically dropped here, because it goes out of scope, and the lock is released.

    // Mutex<T> is a smart pointer. More accurately, the call to lock returns a smart pointer called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap. 
    // The MutexGuard smart pointer implements Deref to point at our inner data; the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope.

    println!("m = {:?}", m);


    // Sharing a Mutex<T> between Threads

    // Now, let’s try to share a value between multiple threads using Mutex<T>. 
    // We’ll spin up 10 threads and have them each increment a counter value by 1, so the counter goes from 0 to 10.


    // let counter = Mutex::new(0);
    // let mut handles = vec![];

    // We use thread::spawn and give all the threads the same closure: one that moves the counter into the thread, acquires a lock on the Mutex<T> by calling the lock method, and then adds 1 to the value in the mutex. 
    // When a thread finishes running its closure, num will go out of scope and release the lock so another thread can acquire it.
    //for _ in 0..10 {
        // ERROR: Moving the same value into multiple threads (closures)
        // Rust is telling us that we can’t move the ownership of lock counter into multiple threads. 
        // Let’s fix the compiler error with a multiple-ownership method, Rc<T> smart pointer.
        // let handle = thread::spawn(move || {
        //     let mut num = counter.lock().unwrap();

        //     *num += 1;
        // });
        // handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());

    /* ERROR: Nice, try but Rc<T> is not thread safe!

        let counter = Rc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Rc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    
    */

    // The compiler is also telling us the reason why: the trait `Send` is not implemented for `Rc<Mutex<i32>>` . 
    // We’ll talk about Send in the next section: it’s one of the traits that ensures the types we use with threads are meant for use in concurrent situations.

    /*  Why is Rc<T> not thread safe? 
    
        When Rc<T> manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped. 
        But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread. 
        This could lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value being dropped before we’re done with it.
    */

    // Atomic Reference Counting: Arc<T>

    // Arc<T> is a type like Rc<T> that is safe to use in concurrent situations. 
    // The a stands for atomic, meaning it’s an atomically reference counted type. Atomics are an additional kind of concurrency primitive
    // Arc<T> by default? The reason is that thread safety comes with a performance penalty that you only want to pay when you really need to.

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Finally, the example will work!
    // Using this strategy, you can divide a calculation into independent parts, split those parts across threads, and then use a Mutex<T> to have each thread update the final result with its part.

    // EXPLORE: there are types simpler than Mutex<T> types provided by the std::sync::atomic module of the standard library. These types provide safe, concurrent, atomic access to primitive types.


    // Similarities Between RefCell<T> / Rc<T> and Mutex<T> / Arc<T>

    // You might have noticed that counter is immutable but we could get a mutable reference to the value inside it; this means Mutex<T> provides interior mutability, as the Cell family does.
    // In the same way we used RefCell<T> to allow us to mutate contents inside an Rc<T>, we use Mutex<T> to mutate contents inside an Arc<T>.

    /*  IMPORTANT:
        Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use Mutex<T>. 
        Recall that using Rc<T> came with the risk of creating reference cycles, where two Rc<T> values refer to each other, causing memory leaks.
        Similarly, Mutex<T> comes with the risk of creating DEADLOCKS. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever.
        Research deadlock mitigation strategies for mutexes in any language and have a go at implementing them in Rust. The standard library API documentation for Mutex<T> and MutexGuard offers useful information.
    */

}

// Extensible Concurrency with the Sync and Send Traits

// The Rust language has very few concurrency features. Almost every concurrency feature we’ve talked about so far in this chapter has been part of the standard library, not the language.
// Two concurrency concepts are embedded in the language: the std::marker traits Sync and Send.


// Allowing Transference of Ownership Between Threads with Send

// The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads. 

// Almost every Rust type is Send, but there are some exceptions, including Rc<T>: this cannot be Send because if you cloned an Rc<T> value and tried to transfer ownership of the clone to another thread, 
// both threads might update the reference count at the same time. For this reason, Rc<T> is implemented for use in single-threaded situations where you don’t want to pay the thread-safe performance penalty.

// Rust’s type system and trait bounds ensure that you can never accidentally send an Rc<T> value across threads unsafely.
// Any type composed entirely of Send types is automatically marked as Send as well. Almost all primitive types are Send, aside from raw pointers.

// Allowing Access from Multiple Threads with Sync

// The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
// Any type T is Sync if &T (an immutable reference to T) is Send, meaning the reference can be sent safely to another thread. 
// Similar to Send, primitive types are Sync, and types composed entirely of types that are Sync are also Sync.

// Rc<T> is not Sync, for the same reason it is not Send
// RefCell<T> is not Sync, and in general none of the Cell types are Sync. (implementation of borrow checking at runtime is not thread-safe)
// Mutex<T> is Sync and can be used to share access with multiple threads.

// UNSAFE: Implementing Send and Sync Manually is Unsafe

// Because types that are made up of Send and Sync traits are automatically also Send and Sync, we don’t have to implement those traits manually.
// As marker traits, they don’t even have any methods to implement, they’re just useful for enforcing invariants related to concurrency.

// For now, building new concurrent types not made up of Send and Sync parts requires careful thought to uphold the safety guarantees.
