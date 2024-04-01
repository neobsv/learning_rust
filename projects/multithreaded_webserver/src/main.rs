// Multithreaded Webserver

// In a single threaded implementation, if the server receives a request that takes a long time to process, subsequent requests will have to wait until the long request is finished, even if the new requests can be processed quickly.

use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration
};

use multithreaded_webserver::ThreadPool;

fn main() {
    // st_main();
    // mt_main();
    mt_main_shutdown();
}

fn st_main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection_with_validation(stream);
    }

}

// Simulating a Slow Request in the Single Threaded implementation

// We can create a path called /sleep which sleeps for 5 seconds before returning a reply, in order to simulate a slow request.
// If you enter the / URI a few times, as before, you’ll see it respond quickly. But if you enter /sleep and then load /, you’ll see that / waits until sleep has slept for its full 5 seconds before loading.

// So we say that serving the path / gets BLOCKED by the path /sleep

fn handle_connection_with_validation(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // We switched from if to match now that we have three cases. We need to explicitly match on a slice of request_line to pattern match against the string literal values.
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}

// Improving Throughput with a Thread Pool

// A thread pool is a group of spawned threads that are waiting and ready to handle a task. When the program receives a new task, it assigns one of the threads in the pool to the task, and that thread will process the task.
// The remaining threads in the pool are available to handle any other tasks that come in while the first thread is processing. When the first thread is done processing its task, it’s returned to the pool of idle threads, ready to handle a new task.
// A thread pool allows you to process connections concurrently, increasing the throughput of your server.

// We’ll have a fixed number of threads waiting in the pool. Requests that come in are sent to the pool for processing. The pool will maintain a queue of incoming requests. Each of the threads in the pool will pop off a request from this queue, 
// handle the request, and then ask the queue for another request. With this design, we can process up to N requests concurrently, where N is the number of threads.

// This technique is just one of many ways to improve the throughput of a web server. Other options you might explore are the fork/join model, the single-threaded async I/O model, or the multi-threaded async I/O model.

// Suggestion: Write the API of the code so it’s structured in the way you want to call it; then implement the functionality within that structure rather than implementing the functionality and then designing the public API.


// NOT Spawning a Thread for Each Request

// thread::spawn will create a new thread and then run the code in the closure in the new thread. If you run this code and load /sleep in your browser, then / in two more browser tabs, you'll see that responses to / return quickly without waiting for /sleep

/* Don't want to do something like this:

    thread::spawn(|| {
        handle_connection_with_validation(stream);
    });

*/

// Creating a Finite Number of Threads

// We use ThreadPool::new to create a new thread pool with a configurable number of threads, in this case four. 
// Then, in the for loop, pool.execute has a similar interface as thread::spawn in that it takes a closure the pool should run for each stream.
// We need to implement pool.execute so it takes the closure and gives it to a thread in the pool to run.

// Building the ThreadPool

// We will make this a library crate to create the threadpool, and make a public struct which defines a ThreadPool

// Write the new function: We know that new needs to have one parameter that can accept 4 as an argument and should return a ThreadPool instance.

// Write the execute function: We’ll implement the execute function so it takes the closure it’s given and gives it to an idle thread in the pool to run.
// We can take closures as parameters with three different traits: Fn, FnMut, and FnOnce. We need to decide which kind of closure to use here.

// Take a peek at thread::spawn implementation, and see that it uses FnOnce() as the trait bound:
/*
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
*/
// The F type parameter is the one we’re concerned with here; the T type parameter is related to the return value, and we’re not concerned with that.
// We can be further confident that FnOnce is the trait we want to use because the thread for running a request will only execute that request’s closure one time, which matches the Once in FnOnce.
// The F type parameter also has the trait bound Send and the lifetime bound 'static, which are useful in our situation: we need Send to transfer the closure from one thread to another and 'static because we don’t know how long the thread will take to execute.


// Validating the number of Threads in new

// Implementing new and execute: 
// A pool with zero threads also makes no sense, yet zero is a perfectly valid usize. We’ll add code to check that size is greater than zero before we return a ThreadPool instance

// Creating Space to Store the Threads:
// The spawn function returns a JoinHandle<T>, where T is the type that the closure returns. In our case, the closures we’re passing to the thread pool will handle the connection and not return anything, so T will be the unit type ().
// We’ve changed the definition of ThreadPool to hold a vector of thread::JoinHandle<()> instances, initialized the vector with a capacity of size, set up a for loop that will run some code to create the threads

// A Worker Struct Responsible for Sending Code from the ThreadPool to the Thread

// We want to create the threads and have them wait for code that we’ll send later. The standard library’s implementation of threads doesn’t include any way to do that; we have to implement it manually.
// We’ll implement this behavior by introducing a new data structure between the ThreadPool and the threads that will manage this new behavior. We’ll call this data structure Worker, which is a common term in pooling implementations. 
// The Worker picks up code that needs to be run and runs the code in the Worker’s thread.

// Instead of storing a vector of JoinHandle<()> instances in the thread pool, we’ll store instances of the Worker struct. Each Worker will store a single JoinHandle<()> instance. 
// Then we’ll implement a method on Worker that will take a closure of code to run and send it to the already running thread for execution. We’ll also give each worker an id so we can distinguish between the different workers in the pool when logging or debugging.

// We’ll implement the code that sends the closure to the thread after we have Worker set up in this way:
    // 1. Define a Worker struct that holds an id and a JoinHandle<()>.
    // 2. Change ThreadPool to hold a vector of Worker instances.
    // 3. Define a Worker::new function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure.
    // 4. In ThreadPool::new, use the for loop counter to generate an id, create a new Worker with that id, and store the worker in the vector.


// Sending Requests to the Threads using a Channel

// Currently, we get the closure we want to execute in the execute method. But we need to give thread::spawn a closure to run when we create each Worker during the creation of the ThreadPool.

    // 1. The ThreadPool will create a channel and hold on to the sender.
    // 2. Each Worker will hold on to the receiver.
    // 3. We’ll create a new Job struct that will hold the closures we want to send down the channel.
    // 4. The execute method will send the job it wants to execute through the sender.
    // 5. In its thread, the Worker will loop over its receiver and execute the closures of any jobs it receives.

// Refer to src/lib.rs for the description of the implementation.

// Success! We now have a thread pool that executes connections asynchronously. There are never more than four threads created, so our system won’t get overloaded.

fn mt_main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection_with_validation(stream);
        });
    }
}

// Graceful Shutdown and Cleanup

// We get some warnings about the workers, id, and thread fields that we’re not using in a direct way that reminds us we’re not cleaning up anything. 
// When we use the less elegant ctrl-c method to halt the main thread, all other threads are stopped immediately as well, even if they’re in the middle of serving a request.

// We’ll implement the Drop trait to call join on each of the threads in the pool so they can finish the requests they’re working on before closing. Then we’ll implement a way to tell the threads they should stop accepting new requests and shut down.
// To see this code in action, we’ll modify our server to accept only two requests before gracefully shutting down its thread pool.

// Let’s start with implementing Drop on our thread pool. When the pool is dropped, our threads should all join to make sure they finish their work:
// Refer src/lib.rs for the implementation and description

// Signaling Threads to Stop Listening for Jobs

// The key is the logic in the closures run by the threads of the Worker instances: at the moment, we call join, but that won’t shut down the threads because they loop forever looking for jobs. 
// If we try to drop our ThreadPool with our current implementation of Drop, the main thread will block forever waiting for the first thread to finish.

// Refer src/lib.rs for further implementation and description

// To see this code in action, let’s modify main to accept only two requests before gracefully shutting down the server:

fn mt_main_shutdown() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection_with_validation(stream);
        });
    }
}

// During execution, the Drop implementation on ThreadPool starts executing before one of the workers even starts its job. 
// Dropping the sender disconnects all the workers and tells them to shut down. The workers each print a message when they disconnect, and then the thread pool calls join to wait for each worker thread to finish.
// Observe during execution: Notice one interesting aspect of this particular execution: the ThreadPool dropped the sender, and before any worker received an error, we tried to join worker 0. Worker 0 had not yet gotten an error from recv, so the main thread blocked waiting for worker 0 to finish.

// Congrats! We are now done implementing a threadpool which processes requests asynchronously and performs a graceful shutdown.
// Fin.