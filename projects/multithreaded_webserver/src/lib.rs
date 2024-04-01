use std::{sync::{mpsc, Arc, Mutex}, thread};

// struct Job;

// We’ve changed the name of the field on ThreadPool from threads to workers because it’s now holding Worker instances instead of JoinHandle<()> instances. 

/* graceful shutdown, optional sender
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}
*/

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

/*

impl ThreadPool {
    // We chose usize as the type of the size parameter, because we know that a negative number of threads doesn’t make any sense
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size); // it preallocates space in the vector

        // The code is trying to pass receiver to multiple Worker instances. This won’t work, the channel implementation that Rust provides is multiple producer, single consumer. 
        // This means we can’t just clone the consuming end of the channel to fix this code. We also don’t want to send a message multiple times to multiple consumers; we want one list 
        // of messages with multiple workers such that each message gets processed once.

        // Additionally, taking a job off the channel queue involves mutating the receiver, so the threads need a safe way to share and modify receiver; otherwise, we might get race conditions

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers, sender }
    }

    // We still use the () after FnOnce because this FnOnce represents a closure that takes no parameters and returns the unit type (). 
    // Just like function definitions, the return type can be omitted from the signature, but even if we have no parameters, we still need the parentheses.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }

}

*/

// We need to send Job structs down the channel, so we change Job from a struct to a type alias for a trait object that holds the type of closure that execute receives.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {

    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // A new instance of the receiver is created, using Mutex::new and Arc::new, to create the lock and the ref counting smart pointer.
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size); // it preallocates space in the vector

        for id in 0..size {
            // Need to call Arc::clone, which does not actually clone the receiver but only clones the pointer and increments the reference count.
            workers.push(Worker::new(id, Arc::clone(&receiver) ));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        // We’re calling unwrap on send for the case that sending fails. This might happen if, for example, we stop all our threads from executing, meaning the receiving end has stopped receiving new messages.
        self.sender.as_ref().unwrap().send(job).unwrap();
    }


}


// The worker struct and its implementation are private, not to be used externally.

/*
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker { id, thread }
    }
}
*/

/* We made thread optional, so changing this, for graceful shutdown
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}
*/

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

/* We made thread optional, so changing this, for graceful shutdown
impl Worker {

    // Recall the thread-safe smart pointers to share ownership across multiple threads and allow the threads to mutate the value, we need to use Arc<Mutex<T>>.
    // The Arc type will let multiple workers own the receiver, and Mutex will ensure that only one worker gets a job from the receiver at a time.
    // In ThreadPool::new, we put the receiver in an Arc and a Mutex. For each new worker, we clone the Arc to bump the reference count so the workers can share ownership of the receiver.

    fn new( id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>> ) -> Worker {
        
        // Our closure being passed to thread::spawn still only references the receiving end of the channel. 
        // Instead, we need the closure to loop forever, asking the receiving end of the channel for a job and running the job when it gets one.

        let thread = thread::spawn(move || loop {
            // The first unwrap is for the lock to acquire the mutex. Acquiring a lock might fail if the mutex is in a poisoned state, which can happen if some other thread panicked while holding the lock rather than releasing the lock.
            // In this situation, calling unwrap to have this thread panic is the correct action to take. Feel free to change this unwrap to an expect with an error message that is meaningful to you.

            // The second unwrap is for the receiver from the channel. If we get the lock on the mutex, we call recv to receive a Job from the channel. 
            // A final unwrap moves past any errors here as well, which might occur if the thread holding the sender has shut down, similar to how the send method returns Err if the receiver shuts down.

            let job = receiver.lock().unwrap().recv().unwrap();

            // The call to recv is a BLOCKING call, if there is no job yet, the current thread will wait until a job becomes available. The Mutex<T> ensures that only one Worker thread at a time is trying to request a job.

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
*/

/* Another change for graceful shutdown, check each recv message in case the sender has been dropped, break and exit the loop,
impl Worker {

    // Recall the thread-safe smart pointers to share ownership across multiple threads and allow the threads to mutate the value, we need to use Arc<Mutex<T>>.
    // The Arc type will let multiple workers own the receiver, and Mutex will ensure that only one worker gets a job from the receiver at a time.
    // In ThreadPool::new, we put the receiver in an Arc and a Mutex. For each new worker, we clone the Arc to bump the reference count so the workers can share ownership of the receiver.

    fn new( id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>> ) -> Worker {
        
        // Our closure being passed to thread::spawn still only references the receiving end of the channel. 
        // Instead, we need the closure to loop forever, asking the receiving end of the channel for a job and running the job when it gets one.

        let thread = thread::spawn(move || loop {
            // The first unwrap is for the lock to acquire the mutex. Acquiring a lock might fail if the mutex is in a poisoned state, which can happen if some other thread panicked while holding the lock rather than releasing the lock.
            // In this situation, calling unwrap to have this thread panic is the correct action to take. Feel free to change this unwrap to an expect with an error message that is meaningful to you.

            // The second unwrap is for the receiver from the channel. If we get the lock on the mutex, we call recv to receive a Job from the channel. 
            // A final unwrap moves past any errors here as well, which might occur if the thread holding the sender has shut down, similar to how the send method returns Err if the receiver shuts down.

            let job = receiver.lock().unwrap().recv().unwrap();

            // The call to recv is a BLOCKING call, if there is no job yet, the current thread will wait until a job becomes available. The Mutex<T> ensures that only one Worker thread at a time is trying to request a job.

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread: Some(thread) }
    }
}
*/


/*
impl Drop for ThreadPool {
    fn drop(&mut self) {

        // we loop through each of the thread pool workers. We use &mut for this because self is a mutable reference, and we also need to be able to mutate worker.
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // The error tells us we can’t call join because we only have a mutable borrow of each worker and join takes ownership of its argument. 
            // To solve this issue, we need to move the thread out of the Worker instance that owns thread so join can consume the thread.

            worker.thread.join().unwrap();
        }
    }
}
*/

// Worker holds an Option<thread::JoinHandle<()>> instead, we can call the take method on the Option to move the value out of the Some variant and leave a None variant in its place. 
// In other words, a Worker that is running will have a Some variant in thread, and when we want to clean up a Worker, we’ll replace Some with None so the Worker doesn’t have a thread to run.

/*
impl Drop for ThreadPool {
    fn drop(&mut self) {

        // we loop through each of the thread pool workers. We use &mut for this because self is a mutable reference, and we also need to be able to mutate worker.
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // The error tells us we can’t call join because we only have a mutable borrow of each worker and join takes ownership of its argument. 
            // To solve this issue, we need to move the thread out of the Worker instance that owns thread so join can consume the thread.
            // We intended to call take on the Option value to move thread out of worker.
            

            // The take method on Option takes the Some variant out and leaves None in its place. We’re using if let to destructure the Some and get the thread; then we call join on the thread. 
            // If a worker’s thread is already None, we know that worker has already had its thread cleaned up
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
            
        }
    }
}
*/


impl Drop for ThreadPool {
    fn drop(&mut self) {

        // First, we’ll change the ThreadPool drop implementation to explicitly drop the sender before waiting for the threads to finish. Listing 20-23 shows the changes to ThreadPool to explicitly Drop sender. 
        // We use the same Option and take technique as we did with the thread to be able to move sender out of ThreadPool:

        drop(self.sender.take());



        // we loop through each of the thread pool workers. We use &mut for this because self is a mutable reference, and we also need to be able to mutate worker.
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // The error tells us we can’t call join because we only have a mutable borrow of each worker and join takes ownership of its argument. 
            // To solve this issue, we need to move the thread out of the Worker instance that owns thread so join can consume the thread.
            // We intended to call take on the Option value to move thread out of worker.
            

            // The take method on Option takes the Some variant out and leaves None in its place. We’re using if let to destructure the Some and get the thread; then we call join on the thread. 
            // If a worker’s thread is already None, we know that worker has already had its thread cleaned up
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
            
        }
    }
}


// Dropping sender closes the channel, which indicates no more messages will be sent. When that happens, all the calls to recv that the workers do in the infinite loop will return an error. 
// We change the Worker loop to gracefully exit the loop in that case, which means the threads will finish when the ThreadPool drop implementation calls join on them

impl Worker {

    // Recall the thread-safe smart pointers to share ownership across multiple threads and allow the threads to mutate the value, we need to use Arc<Mutex<T>>.
    // The Arc type will let multiple workers own the receiver, and Mutex will ensure that only one worker gets a job from the receiver at a time.
    // In ThreadPool::new, we put the receiver in an Arc and a Mutex. For each new worker, we clone the Arc to bump the reference count so the workers can share ownership of the receiver.

    fn new( id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>> ) -> Worker {
        
        // Our closure being passed to thread::spawn still only references the receiving end of the channel. 
        // Instead, we need the closure to loop forever, asking the receiving end of the channel for a job and running the job when it gets one.

        let thread = thread::spawn(move || loop {
            // The first unwrap is for the lock to acquire the mutex. Acquiring a lock might fail if the mutex is in a poisoned state, which can happen if some other thread panicked while holding the lock rather than releasing the lock.
            // In this situation, calling unwrap to have this thread panic is the correct action to take. Feel free to change this unwrap to an expect with an error message that is meaningful to you.

            // The second unwrap is for the receiver from the channel. If we get the lock on the mutex, we call recv to receive a Job from the channel. 
            // A final unwrap moves past any errors here as well, which might occur if the thread holding the sender has shut down, similar to how the send method returns Err if the receiver shuts down.

            let message = receiver.lock().unwrap().recv();

            // The call to recv is a BLOCKING call, if there is no job yet, the current thread will wait until a job becomes available. The Mutex<T> ensures that only one Worker thread at a time is trying to request a job.

            // Graceful Shutdown: check each recv message in case the sender has been dropped, break and exit the loop,
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        
        });

        Worker { id, thread: Some(thread) }
    }
}




// Why did we not use while loops and implement it like this?

// This code compiles and runs but doesn’t result in the desired threading behavior: a slow request will still cause other requests to wait to be processed.
// The reason is somewhat subtle: the Mutex struct has no public unlock method because the ownership of the lock is based on the lifetime of the MutexGuard<T>
// This implementation can also result in the :lock being held longer: than intended if we aren’t mindful of the lifetime of the MutexGuard<T>

/* VERY IMPORTANT: lock being held longer

    let job = receiver.lock().unwrap().recv().unwrap(); works because with let, any temporary values used in the expression on the right hand side of the equals sign are immediately dropped when the let statement ends.
    However, while let (and if let and match) does not drop temporary values until the end of the associated block. The lock remains held for the duration of the call to job(), meaning other workers cannot receive jobs.

    Understand the purpose of the lock, the lock is on the channel, and the purpose of the lock is to only to block the receiver until the job is received, not to block the receiver until the job finishes executing!

*/

struct WorkerII {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl WorkerII {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> WorkerII {
        let thread = thread::spawn(move || {

            
            while let Ok(job) = receiver.lock().unwrap().recv() {
                println!("Worker {id} got a job; executing.");

                job(); 
                // lock is still being held here till the job completes, which is not good
            }
        });

        WorkerII { id, thread }
    }
}
